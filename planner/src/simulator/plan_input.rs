use anyhow::{Result, bail, ensure};
use serde::Deserialize;
use std::collections::HashMap;
use std::path::Path;

use crate::model::GameData;

/// Player's build plan loaded from TOML.
#[derive(Debug, Deserialize)]
pub struct BuildPlanInput {
    pub character: CharacterConfig,
    #[serde(default)]
    pub levels: HashMap<String, LevelConfig>,
    #[serde(default)]
    pub implants: ImplantsConfig,
    #[serde(default)]
    pub skill_books: Vec<SkillBookUse>,
}

#[derive(Debug, Deserialize)]
pub struct CharacterConfig {
    pub strength: i32,
    pub perception: i32,
    pub endurance: i32,
    pub charisma: i32,
    pub intelligence: i32,
    pub agility: i32,
    pub luck: i32,
    pub tagged_skills: Vec<String>,
    #[serde(default)]
    pub traits: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct LevelConfig {
    #[serde(default)]
    pub skills: HashMap<String, i32>,
    pub perk: Option<String>,
}

#[derive(Debug, Default, Deserialize)]
pub struct ImplantsConfig {
    #[serde(default)]
    pub install_at_level: HashMap<String, i32>,
}

#[derive(Debug, Deserialize)]
pub struct SkillBookUse {
    pub skill: String,
    pub at_level: i32,
    #[serde(default = "default_count")]
    pub count: i32,
}

fn default_count() -> i32 {
    1
}

impl BuildPlanInput {
    pub fn load(path: &Path) -> Result<Self> {
        let contents = std::fs::read_to_string(path)?;
        let plan: BuildPlanInput = toml::from_str(&contents)?;
        Ok(plan)
    }

    pub fn special_map(&self) -> HashMap<String, i32> {
        let c = &self.character;
        HashMap::from([
            ("ST".into(), c.strength),
            ("PE".into(), c.perception),
            ("EN".into(), c.endurance),
            ("CH".into(), c.charisma),
            ("IN".into(), c.intelligence),
            ("AG".into(), c.agility),
            ("LK".into(), c.luck),
        ])
    }

    pub fn level_config(&self, level: i32) -> Option<&LevelConfig> {
        self.levels.get(&level.to_string())
    }

    pub fn validate(&self, game_data: &GameData) -> Result<()> {
        let c = &self.character;
        let total = c.strength + c.perception + c.endurance + c.charisma + c.intelligence + c.agility + c.luck;
        ensure!(
            total == game_data.leveling.special_points_at_creation,
            "SPECIAL total is {total}, expected {}",
            game_data.leveling.special_points_at_creation
        );

        for val in [c.strength, c.perception, c.endurance, c.charisma, c.intelligence, c.agility, c.luck] {
            ensure!(val >= 1 && val <= 10, "SPECIAL values must be 1-10, got {val}");
        }

        ensure!(c.tagged_skills.len() == 3, "Must tag exactly 3 skills");
        for tag in &c.tagged_skills {
            ensure!(
                game_data.skill_by_name(tag).is_some(),
                "Unknown tagged skill: {tag}"
            );
        }

        ensure!(c.traits.len() <= 2, "Can select at most 2 traits");
        for t in &c.traits {
            ensure!(
                game_data.trait_by_name(t).is_some(),
                "Unknown trait: {t}"
            );
        }

        // Validate level configs
        for (level_str, config) in &self.levels {
            let level: i32 = level_str.parse()
                .map_err(|_| anyhow::anyhow!("Invalid level key: {level_str}"))?;
            ensure!(
                level >= 2 && level <= game_data.leveling.max_level,
                "Level {level} out of range 2-{}",
                game_data.leveling.max_level
            );

            // Validate skill names
            for skill_name in config.skills.keys() {
                ensure!(
                    game_data.skill_by_name(skill_name).is_some(),
                    "Unknown skill at level {level}: {skill_name}"
                );
            }

            // Validate perk exists
            if let Some(perk_name) = &config.perk {
                ensure!(
                    game_data.perk_by_editor_id(perk_name).is_some(),
                    "Unknown perk at level {level}: {perk_name}"
                );
                ensure!(
                    game_data.leveling.is_perk_level(level),
                    "Level {level} is not a perk level (perks every {} levels)",
                    game_data.leveling.perk_interval
                );
            }
        }

        // Validate implants
        let total_implants = self.implants.install_at_level.len();
        let endurance = c.endurance;
        if total_implants as i32 > endurance {
            bail!(
                "Too many implants ({total_implants}) for Endurance {endurance} (max {endurance})"
            );
        }

        Ok(())
    }
}
