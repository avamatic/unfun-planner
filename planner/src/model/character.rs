use anyhow::{Result, bail, ensure};
use std::collections::HashMap;

use super::GameData;

/// The seven SPECIAL attributes.
pub const SPECIAL_NAMES: [&str; 7] = [
    "Strength",
    "Perception",
    "Endurance",
    "Charisma",
    "Intelligence",
    "Agility",
    "Luck",
];

pub const SPECIAL_ABBREVS: [&str; 7] = ["ST", "PE", "EN", "CH", "IN", "AG", "LK"];

/// SPECIAL attribute values for a character.
#[derive(Debug, Clone)]
pub struct SpecialStats {
    values: HashMap<String, i32>,
}

impl SpecialStats {
    pub fn new(values: HashMap<String, i32>) -> Result<Self> {
        for &abbrev in &SPECIAL_ABBREVS {
            if !values.contains_key(abbrev) {
                bail!("Missing SPECIAL attribute: {abbrev}");
            }
        }
        for (key, &val) in &values {
            if val < 1 || val > 10 {
                bail!("{key} value {val} out of range 1-10");
            }
        }
        Ok(Self { values })
    }

    pub fn validate_total(&self, expected: i32) -> Result<()> {
        let total: i32 = self.values.values().sum();
        ensure!(
            total == expected,
            "SPECIAL total is {total}, expected {expected}"
        );
        Ok(())
    }

    pub fn get(&self, abbrev: &str) -> i32 {
        self.values.get(abbrev).copied().unwrap_or(0)
    }

    pub fn set(&mut self, abbrev: &str, value: i32) -> Result<()> {
        if value < 1 || value > 10 {
            bail!("{abbrev} value {value} out of range 1-10");
        }
        self.values.insert(abbrev.to_string(), value);
        Ok(())
    }

    pub fn modify(&mut self, abbrev: &str, delta: i32) -> Result<()> {
        let current = self.get(abbrev);
        self.set(abbrev, current + delta)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&String, &i32)> {
        self.values.iter()
    }
}

/// Complete character state at a given level.
#[derive(Debug, Clone)]
pub struct CharacterSnapshot {
    pub level: i32,
    pub special: SpecialStats,
    pub skills: HashMap<String, i32>,
    pub perks_taken: Vec<String>,
    pub perk_this_level: Option<String>,
    pub traits: Vec<String>,
    pub implants_installed: Vec<String>,
    pub skill_books_read: Vec<String>,
    pub skill_points_spent: HashMap<String, i32>,
    pub skill_points_available: i32,
    pub skill_points_earned: i32,
}

impl CharacterSnapshot {
    pub fn new_at_creation(
        special: SpecialStats,
        tagged_skills: &[String],
        selected_traits: &[String],
        game_data: &GameData,
    ) -> Result<Self> {
        special.validate_total(game_data.leveling.special_points_at_creation)?;

        // Validate tagged skills
        ensure!(
            tagged_skills.len() == 3,
            "Must tag exactly 3 skills, got {}",
            tagged_skills.len()
        );
        for tag in tagged_skills {
            ensure!(
                game_data.skill_by_name(tag).is_some(),
                "Unknown tagged skill: {tag}"
            );
        }

        // Validate traits
        ensure!(
            selected_traits.len() <= 2,
            "Can select at most 2 traits, got {}",
            selected_traits.len()
        );
        for t in selected_traits {
            ensure!(
                game_data.trait_by_name(t).is_some(),
                "Unknown trait: {t}"
            );
        }

        // Calculate initial skill values
        let mut skills = HashMap::new();
        for skill in &game_data.skills {
            let governing_val = special.get(&skill.governing_special);
            let luck_val = special.get("LK");
            // FNV formula: 2 + (governing * 2) + ceil(luck / 2)
            let base = 2 + (governing_val * 2) + ((luck_val + 1) / 2);
            let tag_bonus = if tagged_skills.iter().any(|t| t.eq_ignore_ascii_case(&skill.name)) {
                game_data.leveling.tag_bonus
            } else {
                0
            };
            skills.insert(skill.name.clone(), base + tag_bonus);
        }

        // Apply trait effects on skills
        for trait_name in selected_traits {
            if let Some(t) = game_data.trait_by_name(trait_name) {
                for effect in &t.effects {
                    if effect.effect_type == "skill_modifier" {
                        if let (Some(target), Some(magnitude)) = (&effect.target, effect.magnitude) {
                            if target == "all" {
                                for val in skills.values_mut() {
                                    *val += magnitude as i32;
                                }
                            } else if let Some(val) = skills.get_mut(target.as_str()) {
                                *val += magnitude as i32;
                            }
                        }
                    }
                }
            }
        }

        Ok(Self {
            level: 1,
            special,
            skills,
            perks_taken: Vec::new(),
            perk_this_level: None,
            traits: selected_traits.to_vec(),
            implants_installed: Vec::new(),
            skill_books_read: Vec::new(),
            skill_points_spent: HashMap::new(),
            skill_points_available: 0,
            skill_points_earned: 0,
        })
    }

    pub fn skill_value(&self, skill_name: &str) -> i32 {
        self.skills.get(skill_name).copied().unwrap_or(0)
    }
}
