## ADDED Requirements

### Requirement: xNVSE plugin loads on game start
The system SHALL be an xNVSE plugin (DLL) that loads when Fallout: New Vegas starts with xNVSE.

#### Scenario: Plugin initialization
- **WHEN** the game launches with xNVSE and the plugin DLL is in the `Data/NVSE/Plugins/` directory
- **THEN** the plugin registers itself and is ready to extract data

### Requirement: Extract skill definitions
The system SHALL extract all skill actor values from the running game, capturing: skill name, editor ID, form ID, governing SPECIAL attribute, and initial value formula.

#### Scenario: Extract all 13 base game skills
- **WHEN** extraction runs on a vanilla FNV instance
- **THEN** the output includes all 13 skills (Barter, Energy Weapons, Explosives, Guns, Lockpick, Medicine, Melee Weapons, Repair, Science, Sneak, Speech, Survival, Unarmed) with their governing attributes

### Requirement: Extract perk definitions
The system SHALL extract all player-selectable perks, capturing: perk name, editor ID, form ID, description, level requirement, SPECIAL prerequisites, skill prerequisites, perk dependencies, maximum ranks, and effect descriptions.

#### Scenario: Extract perk with prerequisites
- **WHEN** a perk has conditions requiring minimum SPECIAL or skill values
- **THEN** the output includes those prerequisites as structured data (e.g., `{"type": "special", "attribute": "strength", "minimum": 6}`)

#### Scenario: Extract multi-rank perk
- **WHEN** a perk allows multiple ranks (e.g., Toughness)
- **THEN** the output includes the maximum rank count and per-rank prerequisites

### Requirement: Extract trait definitions
The system SHALL extract all selectable traits, capturing: trait name, editor ID, form ID, description, and effects (both positive and negative modifiers).

#### Scenario: Extract trait effects
- **WHEN** a trait modifies skills, SPECIAL, or other character values
- **THEN** the output includes structured effect data identifying what is modified and by how much

### Requirement: Extract implant definitions
The system SHALL extract implant data from the New Vegas Medical Clinic, capturing: implant name, form ID, SPECIAL attribute or effect modified, and magnitude.

#### Scenario: Extract SPECIAL implants
- **WHEN** extraction runs
- **THEN** the output includes all SPECIAL-boosting implants with their target attribute and +1 magnitude

### Requirement: Extract skill book and magazine data
The system SHALL extract all skill books (permanent bonuses) and skill magazines (temporary bonuses), capturing: item name, form ID, associated skill, and point value.

#### Scenario: Skill book extraction
- **WHEN** extraction runs
- **THEN** each skill book entry includes the associated skill and base point value (+3)

### Requirement: Extract leveling formulas
The system SHALL extract or compute leveling-related values: skill points per level formula, perk selection interval, maximum level, and any modifier effects on these values (e.g., Educated perk, Skilled trait).

#### Scenario: Skill points per level
- **WHEN** extraction runs
- **THEN** the output includes the skill points per level formula or its computed values for each Intelligence level (1-10)

#### Scenario: Level cap with DLCs
- **WHEN** DLC plugins are loaded that raise the level cap
- **THEN** the extracted maximum level reflects the actual cap (e.g., 50 with all 4 DLCs)

### Requirement: Output to JSON file
The system SHALL write all extracted data to a JSON file at a configurable path (defaulting to `Data/build_data.json`), conforming to a documented schema.

#### Scenario: Successful extraction
- **WHEN** extraction completes
- **THEN** a JSON file is written containing all skills, perks, traits, implants, books, magazines, and leveling data

#### Scenario: Extraction triggered via console command
- **WHEN** the user types a console command (e.g., `ExtractBuildData`)
- **THEN** the extraction runs and the JSON file is written, with a confirmation message displayed in the console
