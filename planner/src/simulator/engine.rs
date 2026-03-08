use anyhow::{Result, bail, ensure};
use std::collections::HashMap;

use crate::model::{BuildPlan, CharacterSnapshot, GameData, SpecialStats};
use super::BuildPlanInput;

pub fn simulate(game_data: &GameData, plan: &BuildPlanInput) -> Result<BuildPlan> {
    plan.validate(game_data)?;

    let special = SpecialStats::new(plan.special_map())?;
    let mut snapshot = CharacterSnapshot::new_at_creation(
        special,
        &plan.character.tagged_skills,
        &plan.character.traits,
        game_data,
    )?;

    let mut snapshots = vec![snapshot.clone()];

    for level in 2..=game_data.leveling.max_level {
        snapshot = advance_level(
            &snapshot,
            level,
            plan.level_config(level),
            plan,
            game_data,
        )?;
        snapshots.push(snapshot.clone());
    }

    Ok(BuildPlan { snapshots })
}

fn advance_level(
    prev: &CharacterSnapshot,
    level: i32,
    level_config: Option<&super::LevelConfig>,
    plan: &BuildPlanInput,
    game_data: &GameData,
) -> Result<CharacterSnapshot> {
    let mut snapshot = prev.clone();
    snapshot.level = level;
    snapshot.perk_this_level = None;
    snapshot.skill_points_spent = HashMap::new();

    // Apply implants scheduled for this level
    for (implant_name, &install_level) in &plan.implants.install_at_level {
        if install_level == level {
            apply_implant(&mut snapshot, implant_name, game_data)?;
        }
    }

    // Apply skill books scheduled for this level
    for book_use in &plan.skill_books {
        if book_use.at_level == level {
            apply_skill_books(&mut snapshot, &book_use.skill, book_use.count, game_data)?;
        }
    }

    // Calculate skill points earned
    let intelligence = snapshot.special.get("IN");
    let mut skill_points = game_data.skill_points_for_intelligence(intelligence);

    // Check for Educated perk
    if snapshot.perks_taken.iter().any(|p| p.eq_ignore_ascii_case("Educated")) {
        skill_points += 2;
    }

    snapshot.skill_points_earned = skill_points;
    snapshot.skill_points_available = skill_points;

    // Distribute skill points
    if let Some(config) = level_config {
        let total_spent: i32 = config.skills.values().sum();
        ensure!(
            total_spent <= skill_points,
            "Level {level}: spending {total_spent} skill points but only {skill_points} available"
        );

        for (skill_name, &points) in &config.skills {
            let current = snapshot.skills.get(skill_name).copied().unwrap_or(0);
            let new_val = current + points;
            ensure!(
                new_val <= game_data.leveling.skill_cap,
                "Level {level}: {skill_name} would be {new_val}, exceeding cap {}",
                game_data.leveling.skill_cap
            );
            snapshot.skills.insert(skill_name.clone(), new_val);
            snapshot.skill_points_spent.insert(skill_name.clone(), points);
        }
        snapshot.skill_points_available = skill_points - total_spent;

        // Apply perk
        if let Some(perk_id) = &config.perk {
            apply_perk(&mut snapshot, perk_id, level, game_data)?;
        }
    }

    Ok(snapshot)
}

fn apply_perk(
    snapshot: &mut CharacterSnapshot,
    perk_editor_id: &str,
    level: i32,
    game_data: &GameData,
) -> Result<()> {
    let perk = game_data
        .perk_by_editor_id(perk_editor_id)
        .ok_or_else(|| anyhow::anyhow!("Unknown perk: {perk_editor_id}"))?;

    // Check level requirement
    ensure!(
        level >= perk.level_requirement,
        "Perk '{}' requires level {}, current level is {level}",
        perk.name,
        perk.level_requirement
    );

    // Check SPECIAL prerequisites
    for prereq in &perk.prerequisites.special {
        let val = snapshot.special.get(&prereq.attribute);
        ensure!(
            val >= prereq.minimum,
            "Perk '{}' requires {} >= {}, have {val}",
            perk.name,
            prereq.attribute,
            prereq.minimum
        );
    }

    // Check skill prerequisites
    for prereq in &perk.prerequisites.skills {
        let val = snapshot.skill_value(&prereq.skill);
        ensure!(
            val >= prereq.minimum,
            "Perk '{}' requires {} >= {}, have {val}",
            perk.name,
            prereq.skill,
            prereq.minimum
        );
    }

    // Check perk dependencies
    for req_perk in &perk.prerequisites.perks {
        ensure!(
            snapshot.perks_taken.iter().any(|p| p.eq_ignore_ascii_case(req_perk)),
            "Perk '{}' requires perk '{}' which hasn't been taken",
            perk.name,
            req_perk
        );
    }

    // Check rank limit
    let current_ranks = snapshot
        .perks_taken
        .iter()
        .filter(|p| p.eq_ignore_ascii_case(perk_editor_id))
        .count() as i32;
    ensure!(
        current_ranks < perk.max_ranks,
        "Perk '{}' already at max ranks ({}/{})",
        perk.name,
        current_ranks,
        perk.max_ranks
    );

    snapshot.perks_taken.push(perk_editor_id.to_string());
    snapshot.perk_this_level = Some(perk.name.clone());

    // Apply perk effects on skills
    for effect in &perk.effects {
        if effect.effect_type == "skill_modifier" {
            if let (Some(target), Some(magnitude)) = (&effect.target, effect.magnitude) {
                if let Some(val) = snapshot.skills.get_mut(target.as_str()) {
                    *val += magnitude as i32;
                }
            }
        }
    }

    Ok(())
}

fn apply_implant(
    snapshot: &mut CharacterSnapshot,
    implant_name: &str,
    game_data: &GameData,
) -> Result<()> {
    let implant = game_data
        .implants
        .iter()
        .find(|i| i.name.eq_ignore_ascii_case(implant_name))
        .ok_or_else(|| anyhow::anyhow!("Unknown implant: {implant_name}"))?;

    // Check endurance limit (based on current endurance, before this implant)
    let max_implants = snapshot.special.get("EN");
    ensure!(
        (snapshot.implants_installed.len() as i32) < max_implants,
        "Cannot install more implants (have {}, max {max_implants} based on Endurance)",
        snapshot.implants_installed.len()
    );

    for effect in &implant.effects {
        if effect.effect_type == "special_modifier" {
            if let (Some(target), Some(magnitude)) = (&effect.target, effect.magnitude) {
                snapshot.special.modify(target, magnitude as i32)?;
                // Recalculate skills governed by this SPECIAL
                recalculate_skills_for_special(snapshot, target, game_data);
            }
        }
    }

    snapshot.implants_installed.push(implant_name.to_string());
    Ok(())
}

fn apply_skill_books(
    snapshot: &mut CharacterSnapshot,
    skill_name: &str,
    count: i32,
    game_data: &GameData,
) -> Result<()> {
    let book = game_data
        .skill_books
        .iter()
        .find(|b| b.skill.eq_ignore_ascii_case(skill_name))
        .ok_or_else(|| anyhow::anyhow!("No skill book for: {skill_name}"))?;

    let mut point_value = book.point_value;

    // Comprehension perk: +1 per book (3 -> 4)
    if snapshot.perks_taken.iter().any(|p| p.eq_ignore_ascii_case("Comprehension")) {
        point_value += 1;
    }

    let current = snapshot.skills.get(&book.skill).copied().unwrap_or(0);
    let new_val = (current + point_value * count).min(game_data.leveling.skill_cap);
    snapshot.skills.insert(book.skill.clone(), new_val);

    for _ in 0..count {
        snapshot.skill_books_read.push(book.name.clone());
    }

    Ok(())
}

fn recalculate_skills_for_special(
    snapshot: &mut CharacterSnapshot,
    special_abbrev: &str,
    game_data: &GameData,
) {
    // When a SPECIAL stat changes (e.g., from implant), the base skill values change.
    // We only adjust by the delta since skills accumulate points over levels.
    // The SPECIAL change of +1 means +2 to governed skills (from the formula: governing * 2).
    for skill in &game_data.skills {
        if skill.governing_special == special_abbrev {
            if let Some(val) = snapshot.skills.get_mut(&skill.name) {
                *val += 2; // +1 SPECIAL = +2 to governed skill base
            }
        }
    }
}
