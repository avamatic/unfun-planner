## 1. xNVSE Extractor Plugin Setup

- [x] 1.1 Set up C++ project for xNVSE plugin (CMake or MSVC solution, link against xNVSE headers)
- [x] 1.2 Implement plugin entry point (`NVSEPlugin_Query`, `NVSEPlugin_Load`) with version checks
- [x] 1.3 Register a console command (`ExtractBuildData`) that triggers extraction

## 2. xNVSE Data Extraction

- [x] 2.1 Extract skill definitions: iterate actor value info forms, capture name, editor ID, form ID, governing SPECIAL attribute
- [x] 2.2 Extract perk definitions: iterate perk forms, capture name, description, level requirement, prerequisites (SPECIAL, skill, perk conditions), max ranks
- [x] 2.3 Extract trait definitions: iterate trait forms, capture name, description, positive/negative effects
- [x] 2.4 Extract implant definitions: capture target attribute, magnitude, form ID
- [x] 2.5 Extract skill books and magazines: iterate book forms with skill-teaching flags, capture associated skill and point value
- [x] 2.6 Extract leveling formulas: compute skill points per level for each INT value (1-10), extract perk interval, extract level cap
- [x] 2.7 Serialize all extracted data to JSON and write to `Data/build_data.json`

## 3. JSON Schema Definition

- [x] 3.1 Define the JSON schema documenting all fields: skills, perks, traits, implants, books, magazines, formulas, metadata (game version, load order)
- [x] 3.2 Create a sample/reference `build_data.json` for vanilla FNV to use during planner development

## 4. Rust Planner Project Setup

- [x] 4.1 Initialize Rust project with `cargo init`, add dependencies: `clap` (CLI), `serde`/`serde_json` (JSON), `toml` (plan parsing), `anyhow` (errors)
- [x] 4.2 Define module structure: `model/`, `simulator/`, `cli/`

## 5. Build Data Model (Rust)

- [x] 5.1 Define Rust types matching the JSON schema: `GameData`, `Skill`, `Perk`, `Trait`, `Implant`, `SkillBook`, `LevelingFormulas`
- [x] 5.2 Implement JSON deserialization with validation and clear error messages for schema mismatches
- [x] 5.3 Define SPECIAL attribute types and validation (7 attributes, range 1-10, total 40)
- [x] 5.4 Define character state types: `CharacterSnapshot` (per-level state), `BuildPlan` (full progression)

## 6. Build Simulator

- [x] 6.1 Implement character creation: SPECIAL allocation validation, initial skill calculation from governing attributes, tag bonus application, trait effects
- [x] 6.2 Implement level-up logic: skill points per level from extracted formulas, skill point distribution with cap enforcement (100)
- [x] 6.3 Implement perk selection validation: prerequisite checking at each level, rank tracking
- [x] 6.4 Implement implant application: SPECIAL modification, Endurance-based limit check, downstream recalculation
- [x] 6.5 Implement skill book application: permanent skill increase with Comprehension perk interaction
- [x] 6.6 Implement full build simulation loop: iterate level 1 through max, produce per-level CharacterSnapshot
- [x] 6.7 Implement build plan pre-validation: check entire plan for errors before simulation

## 7. Build Plan TOML Format

- [x] 7.1 Define TOML schema for build plan: `[character]` (SPECIAL, tags, traits), `[level.<N>]` (skill points, perk), `[implants]`, `[skill_books]`
- [x] 7.2 Implement TOML parser with validation and clear error messages
- [x] 7.3 Create example build plan file demonstrating all options

## 8. CLI Interface

- [x] 8.1 Implement `simulate` subcommand: load JSON data + TOML plan, run simulation, output build plan
- [x] 8.2 Implement `info` subcommand: display extracted game data (perks, skills, traits) in readable format
- [x] 8.3 Implement `init` subcommand: generate template TOML build plan from game data
- [x] 8.4 Implement `--format` flag: `table` output with level-by-level progression, `json` output
- [x] 8.5 Implement table formatter: level-by-level display with SPECIAL, skills, perks, milestone highlights

## 9. Testing

- [x] 9.1 Unit tests for data model: SPECIAL validation, skill calculation formulas, JSON deserialization
- [x] 9.2 Unit tests for simulator: level-up math, perk prerequisites, modifier stacking, edge cases
- [x] 9.3 Unit tests for TOML plan parsing with valid and invalid inputs
- [x] 9.4 Integration test: load reference `build_data.json`, simulate a known build, verify output matches expected progression
