## ADDED Requirements

### Requirement: JSON schema for game data interchange
The system SHALL define a JSON schema that serves as the contract between the xNVSE extractor and the Rust build planner. The schema SHALL cover: SPECIAL attributes, skills, perks, traits, implants, skill books, magazines, and leveling formulas.

#### Scenario: Schema validation
- **WHEN** the Rust planner loads a JSON file produced by the extractor
- **THEN** it validates the JSON against the expected schema and reports clear errors for missing or malformed fields

### Requirement: SPECIAL attributes model
The system SHALL represent the seven SPECIAL attributes (Strength, Perception, Endurance, Charisma, Intelligence, Agility, Luck) with their base values, minimum (1), and maximum (10), totaling 40 points at creation.

#### Scenario: SPECIAL bounds enforcement
- **WHEN** a SPECIAL attribute would exceed 10 or fall below 1
- **THEN** the system rejects the allocation with an error

#### Scenario: Default SPECIAL allocation
- **WHEN** a character is created with default SPECIAL values
- **THEN** each attribute starts at 5, with 5 additional points to distribute

### Requirement: Skills model
The system SHALL represent all skills with their governing SPECIAL attribute, base value formula, tag bonus (+15), and skill cap (100).

#### Scenario: Initial skill values
- **WHEN** a character is created with given SPECIAL stats and 3 tagged skills
- **THEN** each skill's initial value is calculated from its governing SPECIAL, and tagged skills receive +15

### Requirement: Perks model
The system SHALL represent perks with their name, description, level requirement, SPECIAL prerequisites, skill prerequisites, perk dependencies, maximum ranks, and effects.

#### Scenario: Perk eligibility check
- **WHEN** a character attempts to select a perk at a given level
- **THEN** the system validates all prerequisites: level, SPECIAL minimums, skill minimums, perk dependencies, and rank limits

### Requirement: Traits model
The system SHALL represent traits with their name, description, and effects (positive and negative modifiers to skills, SPECIAL, or other values).

#### Scenario: Trait selection at character creation
- **WHEN** a player selects 0-2 traits during character creation
- **THEN** the system applies both positive and negative effects to the character model

### Requirement: Implants model
The system SHALL represent implants with their target attribute/effect and magnitude, constrained by the character's Endurance stat.

#### Scenario: Implant limit
- **WHEN** a character with Endurance 7 attempts to install implants
- **THEN** the system allows a maximum of 7 implants total

### Requirement: Skill books and magazines model
The system SHALL represent skill books (permanent +3, or +4 with Comprehension) and skill magazines (temporary +10, or +20 with Comprehension) as separate modifier types.

#### Scenario: Skill book with Comprehension perk
- **WHEN** a character with the Comprehension perk reads a skill book
- **THEN** the skill book grants +4 points instead of +3

### Requirement: Level progression rules
The system SHALL encode level progression: skill points per level, perk selection interval, and maximum level — sourced from the extracted JSON data rather than hardcoded.

#### Scenario: Skill points with Intelligence 8
- **WHEN** a character with Intelligence 8 levels up
- **THEN** the character receives the correct number of skill points as defined by the extracted formula
