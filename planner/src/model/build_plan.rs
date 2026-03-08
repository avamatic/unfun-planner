use serde::Serialize;
use super::CharacterSnapshot;

/// Complete build plan output — one snapshot per level.
#[derive(Debug)]
pub struct BuildPlan {
    pub snapshots: Vec<CharacterSnapshot>,
}

/// Serializable version of a build plan for JSON output.
#[derive(Debug, Serialize)]
pub struct BuildPlanOutput {
    pub levels: Vec<LevelOutput>,
}

#[derive(Debug, Serialize)]
pub struct LevelOutput {
    pub level: i32,
    pub special: std::collections::HashMap<String, i32>,
    pub skills: std::collections::HashMap<String, i32>,
    pub perk: Option<String>,
    pub skill_points_earned: i32,
    pub skill_points_spent: std::collections::HashMap<String, i32>,
    pub milestones: Vec<String>,
}

impl BuildPlan {
    pub fn to_output(&self) -> BuildPlanOutput {
        let mut levels = Vec::new();
        let mut prev_skills: Option<&std::collections::HashMap<String, i32>> = None;

        for snapshot in &self.snapshots {
            let mut milestones = Vec::new();

            // Check for perk acquisition
            if let Some(perk) = &snapshot.perk_this_level {
                milestones.push(format!("Perk: {perk}"));
            }

            // Check for skill thresholds crossed
            if let Some(prev) = prev_skills {
                for (skill, &value) in &snapshot.skills {
                    let prev_val = prev.get(skill).copied().unwrap_or(0);
                    for threshold in [25, 50, 75, 100] {
                        if prev_val < threshold && value >= threshold {
                            milestones.push(format!("{skill} reached {threshold}"));
                        }
                    }
                }
            }

            let special_map = snapshot
                .special
                .iter()
                .map(|(k, v)| (k.clone(), *v))
                .collect();

            levels.push(LevelOutput {
                level: snapshot.level,
                special: special_map,
                skills: snapshot.skills.clone(),
                perk: snapshot.perk_this_level.clone(),
                skill_points_earned: snapshot.skill_points_earned,
                skill_points_spent: snapshot.skill_points_spent.clone(),
                milestones,
            });

            prev_skills = Some(&snapshot.skills);
        }

        BuildPlanOutput { levels }
    }
}
