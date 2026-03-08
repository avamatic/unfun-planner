use anyhow::{Result, bail};
use serde::Deserialize;
use std::collections::HashMap;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct GameData {
    pub metadata: Metadata,
    pub special_attributes: Vec<SpecialAttribute>,
    pub skills: Vec<Skill>,
    pub perks: Vec<Perk>,
    pub traits: Vec<Trait>,
    pub implants: Vec<Implant>,
    pub skill_books: Vec<SkillBook>,
    pub skill_magazines: Vec<SkillMagazine>,
    pub leveling: LevelingFormulas,
}

#[derive(Debug, Deserialize)]
pub struct Metadata {
    pub game_version: String,
    pub extractor_version: String,
    pub load_order: Vec<String>,
    pub extraction_date: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct SpecialAttribute {
    pub name: String,
    pub abbreviation: String,
    pub default_value: i32,
    pub min_value: i32,
    pub max_value: i32,
}

#[derive(Debug, Deserialize)]
pub struct Skill {
    pub name: String,
    pub editor_id: String,
    pub form_id: String,
    pub governing_special: String,
    pub base_value_formula: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Perk {
    pub name: String,
    pub editor_id: String,
    pub form_id: String,
    pub description: String,
    pub max_ranks: i32,
    #[serde(default)]
    pub level_requirement: i32,
    #[serde(default)]
    pub prerequisites: PerkPrerequisites,
    #[serde(default)]
    pub effects: Vec<Effect>,
    #[serde(default = "default_true")]
    pub is_playable: bool,
}

fn default_true() -> bool {
    true
}

#[derive(Debug, Default, Deserialize)]
pub struct PerkPrerequisites {
    #[serde(default)]
    pub special: Vec<SpecialPrereq>,
    #[serde(default)]
    pub skills: Vec<SkillPrereq>,
    #[serde(default)]
    pub perks: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct SpecialPrereq {
    pub attribute: String,
    pub minimum: i32,
}

#[derive(Debug, Deserialize)]
pub struct SkillPrereq {
    pub skill: String,
    pub minimum: i32,
}

#[derive(Debug, Deserialize)]
pub struct Trait {
    pub name: String,
    pub editor_id: String,
    pub form_id: String,
    pub description: String,
    pub effects: Vec<Effect>,
}

#[derive(Debug, Deserialize)]
pub struct Effect {
    #[serde(rename = "type")]
    pub effect_type: String,
    #[serde(default)]
    pub description: String,
    pub target: Option<String>,
    pub magnitude: Option<f64>,
}

#[derive(Debug, Deserialize)]
pub struct Implant {
    pub name: String,
    pub form_id: String,
    pub cost: Option<i32>,
    pub effects: Vec<Effect>,
}

#[derive(Debug, Deserialize)]
pub struct SkillBook {
    pub name: String,
    pub form_id: String,
    pub skill: String,
    pub point_value: i32,
}

#[derive(Debug, Deserialize)]
pub struct SkillMagazine {
    pub name: String,
    pub form_id: String,
    pub skill: String,
    pub point_value: i32,
    pub duration: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct LevelingFormulas {
    pub max_level: i32,
    pub perk_interval: i32,
    pub skill_points_per_level: SkillPointsFormula,
    pub tag_bonus: i32,
    pub skill_cap: i32,
    pub special_points_at_creation: i32,
}

#[derive(Debug, Deserialize)]
pub struct SkillPointsFormula {
    pub base: i32,
    pub intelligence_multiplier: f64,
}

impl GameData {
    pub fn load(path: &Path) -> Result<Self> {
        let contents = std::fs::read_to_string(path)?;
        let data: GameData = serde_json::from_str(&contents)?;
        data.validate()?;
        Ok(data)
    }

    fn validate(&self) -> Result<()> {
        if self.special_attributes.len() != 7 {
            bail!(
                "Expected 7 SPECIAL attributes, found {}",
                self.special_attributes.len()
            );
        }
        if self.skills.is_empty() {
            bail!("No skills found in game data");
        }
        if self.leveling.max_level < 1 {
            bail!("Invalid max level: {}", self.leveling.max_level);
        }
        Ok(())
    }

    pub fn skill_by_name(&self, name: &str) -> Option<&Skill> {
        self.skills.iter().find(|s| s.name.eq_ignore_ascii_case(name))
    }

    pub fn perk_by_editor_id(&self, editor_id: &str) -> Option<&Perk> {
        self.perks.iter().find(|p| p.editor_id.eq_ignore_ascii_case(editor_id))
    }

    pub fn trait_by_name(&self, name: &str) -> Option<&Trait> {
        self.traits.iter().find(|t| t.name.eq_ignore_ascii_case(name))
    }

    pub fn skill_points_for_intelligence(&self, intelligence: i32) -> i32 {
        let f = &self.leveling.skill_points_per_level;
        f.base + (intelligence as f64 * f.intelligence_multiplier).floor() as i32
    }

    pub fn playable_perks(&self) -> impl Iterator<Item = &Perk> {
        self.perks.iter().filter(|p| p.is_playable)
    }

    /// Build a map of skill name -> governing SPECIAL abbreviation.
    pub fn skill_governing_map(&self) -> HashMap<String, String> {
        self.skills
            .iter()
            .map(|s| (s.name.clone(), s.governing_special.clone()))
            .collect()
    }
}

impl LevelingFormulas {
    pub fn is_perk_level(&self, level: i32) -> bool {
        level >= 2 && level % self.perk_interval == 0
    }
}
