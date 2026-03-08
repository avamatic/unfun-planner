use std::path::Path;

// Integration test: load reference data, simulate a known build, verify output.
#[test]
fn simulate_example_build() {
    // Find data file relative to workspace root
    let data_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("data/fnv_vanilla_data.json");

    let plan_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("examples/example_build.toml");

    assert!(data_path.exists(), "Reference data file not found: {}", data_path.display());
    assert!(plan_path.exists(), "Example plan file not found: {}", plan_path.display());

    // Run the CLI and check exit code
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_fnv-planner"))
        .args(["simulate", "--data", data_path.to_str().unwrap(), "--plan", plan_path.to_str().unwrap(), "--format", "json"])
        .output()
        .expect("Failed to run fnv-planner");

    assert!(output.status.success(), "CLI failed: {}", String::from_utf8_lossy(&output.stderr));

    let json: serde_json::Value = serde_json::from_slice(&output.stdout).expect("Invalid JSON output");
    let levels = json["levels"].as_array().expect("Missing levels array");

    // Should have 30 levels (level 1 through 30)
    assert_eq!(levels.len(), 30, "Expected 30 level snapshots");

    // Level 1 should have correct initial SPECIAL
    let level1 = &levels[0];
    assert_eq!(level1["level"], 1);
    assert_eq!(level1["special"]["IN"], 9);
    assert_eq!(level1["special"]["EN"], 9);
    assert_eq!(level1["special"]["CH"], 1);

    // Guns is tagged, so with AG=6, LK=5: 2 + (6*2) + ceil(5/2) = 2+12+3 = 17, +15 tag = 32
    // But Good Natured: +5 to Speech/Barter/Medicine/Repair/Science, -5 to combat skills
    // Good Natured effects: Guns gets -5, so 32-5 = 27
    let guns_l1 = level1["skills"]["Guns"].as_i64().unwrap();
    assert_eq!(guns_l1, 27, "Level 1 Guns should be 27 (tagged + Good Natured penalty)");

    // Repair is tagged, governed by IN=9: 2 + (9*2) + 3 = 23, +15 = 38, +5 (Good Natured) = 43
    let repair_l1 = level1["skills"]["Repair"].as_i64().unwrap();
    assert_eq!(repair_l1, 43, "Level 1 Repair should be 43");

    // Level 2 should have Confirmed Bachelor perk
    let level2 = &levels[1];
    assert_eq!(level2["perk"], "Confirmed Bachelor");

    // Level 4 should have Educated perk
    let level4 = &levels[3];
    assert_eq!(level4["perk"], "Educated");

    // After level 5, Guns should reflect skill books (+6 from 2 books)
    let guns_l5 = levels[4]["skills"]["Guns"].as_i64().unwrap();
    assert!(guns_l5 > guns_l1 + 10, "Level 5 Guns should reflect skill point investments and books");
}

#[test]
fn info_commands_succeed() {
    let data_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("data/fnv_vanilla_data.json");

    for topic in ["skills", "perks", "traits", "implants", "leveling"] {
        let output = std::process::Command::new(env!("CARGO_BIN_EXE_fnv-planner"))
            .args(["info", "--data", data_path.to_str().unwrap(), topic])
            .output()
            .expect("Failed to run fnv-planner");

        assert!(
            output.status.success(),
            "info {topic} failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        assert!(
            !output.stdout.is_empty(),
            "info {topic} produced no output"
        );
    }
}

#[test]
fn invalid_special_total_rejected() {
    let data_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("data/fnv_vanilla_data.json");

    // Create a temp plan with bad SPECIAL total
    let bad_plan = r#"
[character]
strength = 10
perception = 10
endurance = 10
charisma = 10
intelligence = 10
agility = 10
luck = 10
tagged_skills = ["Guns", "Repair", "Speech"]
traits = []
"#;

    let tmp = std::env::temp_dir().join("bad_plan.toml");
    std::fs::write(&tmp, bad_plan).unwrap();

    let output = std::process::Command::new(env!("CARGO_BIN_EXE_fnv-planner"))
        .args(["simulate", "--data", data_path.to_str().unwrap(), "--plan", tmp.to_str().unwrap()])
        .output()
        .expect("Failed to run fnv-planner");

    assert!(!output.status.success(), "Should reject invalid SPECIAL total");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("SPECIAL total"), "Error should mention SPECIAL total: {stderr}");
}
