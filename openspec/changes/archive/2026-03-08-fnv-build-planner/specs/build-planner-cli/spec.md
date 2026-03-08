## ADDED Requirements

### Requirement: Accept game data JSON
The system SHALL accept a `--data` argument specifying the path to the JSON file exported by the xNVSE extractor plugin.

#### Scenario: Explicit data file
- **WHEN** the user runs `fnv-planner simulate --data build_data.json --plan build.toml`
- **THEN** the system loads game data from that JSON file

#### Scenario: Missing data file
- **WHEN** the specified JSON file does not exist
- **THEN** the system reports a clear error

### Requirement: Accept build plan file
The system SHALL accept a `--plan` argument specifying a TOML file containing the player's build choices.

#### Scenario: Valid build plan
- **WHEN** the user provides a TOML file with SPECIAL allocation, tagged skills, traits, and per-level choices
- **THEN** the system parses the plan and uses it for simulation

#### Scenario: Invalid TOML syntax
- **WHEN** the TOML file has syntax errors
- **THEN** the system reports the parse error with line number

### Requirement: Simulate subcommand
The system SHALL support a `simulate` subcommand that runs the full build simulation given game data JSON and a plan file.

#### Scenario: Run simulation
- **WHEN** the user runs `fnv-planner simulate --data build_data.json --plan build.toml`
- **THEN** the system loads game data, validates the plan, simulates progression, and outputs the build plan

### Requirement: Info subcommand
The system SHALL support an `info` subcommand that displays extracted game data in a human-readable format (list perks, skills, traits, etc.).

#### Scenario: List all perks
- **WHEN** the user runs `fnv-planner info --data build_data.json perks`
- **THEN** the system displays all perks with their prerequisites and descriptions

#### Scenario: List all skills
- **WHEN** the user runs `fnv-planner info --data build_data.json skills`
- **THEN** the system displays all skills with their governing attributes

### Requirement: Output format selection
The system SHALL support `--format` argument with values `table` (default) and `json`.

#### Scenario: Table output
- **WHEN** the user runs with `--format table` or no format flag
- **THEN** the system outputs a human-readable table showing level-by-level progression

#### Scenario: JSON output
- **WHEN** the user runs with `--format json`
- **THEN** the system outputs the full build plan as structured JSON

### Requirement: Init subcommand
The system SHALL support an `init` subcommand that generates a template TOML build plan file the user can edit.

#### Scenario: Generate template
- **WHEN** the user runs `fnv-planner init --data build_data.json --output my_build.toml`
- **THEN** the system creates a TOML file with default SPECIAL, empty skill allocations per level, and commented-out perk options
