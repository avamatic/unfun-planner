## ADDED Requirements

### Requirement: Character creation simulation
The system SHALL simulate character creation by accepting SPECIAL allocation (40 total points), 3 tagged skills, and 0-2 trait selections, then computing initial character state.

#### Scenario: Complete character creation
- **WHEN** a player specifies SPECIAL {S:5, P:5, E:9, C:1, I:9, A:6, L:5}, tags {Guns, Repair, Speech}, and trait {Skilled}
- **THEN** the system produces a level 1 character snapshot with correct initial skill values including tag bonuses and trait effects

#### Scenario: Invalid SPECIAL total
- **WHEN** a player specifies SPECIAL points that do not sum to 40
- **THEN** the system rejects the build with a validation error

### Requirement: Level-up simulation
The system SHALL simulate each level-up from 2 to the maximum level, applying: skill point distribution, perk selection (at the game-defined interval), and modifier effects.

#### Scenario: Level-up with perk selection
- **WHEN** a character reaches an even level
- **THEN** the system applies the player's skill point distribution and selected perk

#### Scenario: Level-up without perk
- **WHEN** a character reaches an odd level
- **THEN** the system applies skill point distribution only

#### Scenario: Skill cap enforcement
- **WHEN** skill point allocation would raise a skill above 100
- **THEN** the system rejects the allocation with an error

### Requirement: Modifier stacking
The system SHALL correctly stack modifiers from all sources: base SPECIAL, implants, perks, traits, skill books, and per-level skill point allocations.

#### Scenario: Intelligence implant affects future levels
- **WHEN** a character installs the Intelligence implant at level 4
- **THEN** skill points per level increase accordingly for all subsequent levels

#### Scenario: Skilled trait effect
- **WHEN** a character has the Skilled trait
- **THEN** all skills receive +5 points

### Requirement: Build validation
The system SHALL validate the entire build plan before simulation: SPECIAL totals, perk prerequisites at each level, skill point budget per level, and rank limits.

#### Scenario: Perk prerequisite not met
- **WHEN** a build plan selects a perk at a level where prerequisites are not met
- **THEN** the system reports which prerequisite fails and at which level

#### Scenario: Overspending skill points
- **WHEN** a build plan allocates more skill points at a level than the character earns
- **THEN** the system reports the overspend amount and level

### Requirement: Build plan output
The system SHALL produce a level-by-level output showing: current SPECIAL values, all skill values, perk acquired (if any), skill points spent, and cumulative modifiers applied.

#### Scenario: Full build plan output
- **WHEN** simulation completes successfully
- **THEN** the output contains one entry per level showing the complete character state

#### Scenario: Highlight key milestones
- **WHEN** outputting the build plan
- **THEN** the system marks levels where perks are acquired, implants installed, or skill thresholds crossed (25, 50, 75, 100)
