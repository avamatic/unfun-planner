use crate::model::{BuildPlan, BuildPlanOutput, GameData, SPECIAL_ABBREVS};

pub fn print_build_table(plan: &BuildPlan, game_data: &GameData) {
    let output = plan.to_output();

    // Get sorted skill names
    let mut skill_names: Vec<&str> = game_data.skills.iter().map(|s| s.name.as_str()).collect();
    skill_names.sort();

    // Print header
    print!("{:<5}", "Lvl");
    for abbrev in &SPECIAL_ABBREVS {
        print!(" {:<3}", abbrev);
    }
    for skill in &skill_names {
        // Abbreviate skill names to 6 chars
        let abbrev = if skill.len() > 6 { &skill[..6] } else { skill };
        print!(" {:<6}", abbrev);
    }
    println!("  Perk");

    // Print separator
    let width = 5 + (SPECIAL_ABBREVS.len() * 4) + (skill_names.len() * 7) + 6;
    println!("{}", "-".repeat(width));

    // Print each level
    for level in &output.levels {
        print!("{:<5}", level.level);

        for abbrev in &SPECIAL_ABBREVS {
            let val = level.special.get(*abbrev).copied().unwrap_or(0);
            print!(" {:<3}", val);
        }

        for skill in &skill_names {
            let val = level.skills.get(*skill).copied().unwrap_or(0);
            print!(" {:<6}", val);
        }

        if let Some(perk) = &level.perk {
            print!("  {}", perk);
        }

        println!();

        // Print milestones
        for milestone in &level.milestones {
            if !milestone.starts_with("Perk:") {
                println!("      ^ {milestone}");
            }
        }
    }
}

pub fn print_build_json(plan: &BuildPlan) {
    let output = plan.to_output();
    println!("{}", serde_json::to_string_pretty(&output).unwrap());
}

pub fn print_skills(game_data: &GameData) {
    println!("{:<20} {:<8} {}", "Skill", "SPECIAL", "Formula");
    println!("{}", "-".repeat(60));
    for skill in &game_data.skills {
        println!(
            "{:<20} {:<8} {}",
            skill.name,
            skill.governing_special,
            skill.base_value_formula.as_deref().unwrap_or("-")
        );
    }
}

pub fn print_perks(game_data: &GameData) {
    println!("{:<30} {:<5} {:<15} {}", "Perk", "Lvl", "Prerequisites", "Ranks");
    println!("{}", "-".repeat(80));
    for perk in game_data.playable_perks() {
        let mut prereqs = Vec::new();
        for sp in &perk.prerequisites.special {
            prereqs.push(format!("{} {}", sp.attribute, sp.minimum));
        }
        for sk in &perk.prerequisites.skills {
            prereqs.push(format!("{} {}", sk.skill, sk.minimum));
        }
        let prereq_str = if prereqs.is_empty() {
            "-".to_string()
        } else {
            prereqs.join(", ")
        };
        println!(
            "{:<30} {:<5} {:<15} {}",
            perk.name, perk.level_requirement, prereq_str, perk.max_ranks
        );
    }
}

pub fn print_traits(game_data: &GameData) {
    for t in &game_data.traits {
        println!("{}:", t.name);
        println!("  {}", t.description);
        for effect in &t.effects {
            let sign = if effect.magnitude.unwrap_or(0.0) >= 0.0 { "+" } else { "" };
            println!(
                "  {} {}{}",
                effect.effect_type,
                sign,
                effect.magnitude.map(|m| m.to_string()).unwrap_or_default()
            );
        }
        println!();
    }
}

pub fn print_implants(game_data: &GameData) {
    println!("{:<30} {:<10} {}", "Implant", "Cost", "Effect");
    println!("{}", "-".repeat(60));
    for implant in &game_data.implants {
        let effects: Vec<String> = implant.effects.iter().map(|e| {
            if !e.description.is_empty() {
                e.description.clone()
            } else {
                let target = e.target.as_deref().unwrap_or("?");
                let mag = e.magnitude.map(|m| format!("{:+}", m as i32)).unwrap_or_default();
                format!("{mag} {target}")
            }
        }).collect();
        println!(
            "{:<30} {:<10} {}",
            implant.name,
            implant.cost.map(|c| format!("{c} caps")).unwrap_or("-".into()),
            effects.join(", ")
        );
    }
}

pub fn print_leveling(game_data: &GameData) {
    let l = &game_data.leveling;
    println!("Max Level:       {}", l.max_level);
    println!("Perk Interval:   Every {} levels", l.perk_interval);
    println!("Tag Bonus:       +{}", l.tag_bonus);
    println!("Skill Cap:       {}", l.skill_cap);
    println!("SPECIAL Points:  {}", l.special_points_at_creation);
    println!();
    println!("Skill Points Per Level (base {} + INT * {}):", l.skill_points_per_level.base, l.skill_points_per_level.intelligence_multiplier);
    for int in 1..=10 {
        let pts = game_data.skill_points_for_intelligence(int);
        println!("  INT {:>2}: {} skill points", int, pts);
    }
}

pub fn generate_template(game_data: &GameData, output: &std::path::Path) -> anyhow::Result<()> {
    let mut buf = String::new();

    buf.push_str("# Fallout: New Vegas Build Plan\n");
    buf.push_str("# Generated from extracted game data\n\n");

    buf.push_str("[character]\n");
    buf.push_str("strength = 5\n");
    buf.push_str("perception = 5\n");
    buf.push_str("endurance = 5\n");
    buf.push_str("charisma = 5\n");
    buf.push_str("intelligence = 5\n");
    buf.push_str("agility = 5\n");
    buf.push_str("luck = 10\n");
    buf.push_str("tagged_skills = [\"Guns\", \"Lockpick\", \"Speech\"]\n");
    buf.push_str("traits = []\n\n");

    // Generate level entries
    for level in 2..=game_data.leveling.max_level {
        buf.push_str(&format!("[levels.{}]\n", level));
        buf.push_str("skills = {}\n");
        if game_data.leveling.is_perk_level(level) {
            buf.push_str("# perk = \"PerkEditorId\"\n");
        }
        buf.push_str("\n");
    }

    buf.push_str("[implants]\n");
    buf.push_str("# install_at_level = { \"Strength Implant\" = 4, \"Intelligence Implant\" = 6 }\n");
    buf.push_str("install_at_level = {}\n\n");

    buf.push_str("# [[skill_books]]\n");
    buf.push_str("# skill = \"Guns\"\n");
    buf.push_str("# at_level = 5\n");
    buf.push_str("# count = 1\n");

    std::fs::write(output, &buf)?;
    println!("Template written to {}", output.display());

    // Print available perks as reference
    println!("\nAvailable perks (use editor_id in plan):");
    for perk in game_data.playable_perks() {
        println!("  {} (lvl {}) - {}", perk.editor_id, perk.level_requirement, perk.name);
    }

    Ok(())
}
