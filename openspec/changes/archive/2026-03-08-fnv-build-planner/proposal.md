## Why

There is no tool that extracts live game data from a Fallout: New Vegas installation and uses it to simulate character builds. Players rely on wikis, spreadsheets, or hardcoded planners that go stale when mods change the game's data. Key build formulas (skill point calculations, SPECIAL-to-skill mappings) live in the engine binary, not in data files, making static ESM parsing incomplete. By extracting data at runtime via an NVSE plugin — where the engine has already parsed, resolved, and hydrated everything — the planner gets accurate data regardless of mod configuration.

## What Changes

- New xNVSE plugin (C++) that runs inside the game, walks in-memory game objects, and exports all build-relevant data (skills, perks, traits, implants, skill books, leveling formulas) to a structured JSON file
- New Rust CLI that consumes the exported JSON, accepts a player's build choices via TOML, simulates level-by-level character progression, and outputs a validated build plan
- Two-component architecture: extractor mod (runs once in-game to dump data) + build planner (runs offline against the dump)

## Capabilities

### New Capabilities
- `nvse-extractor`: xNVSE plugin that extracts build-relevant game data from running FNV instance to JSON — skills, perks, traits, implants, skill books, magazines, leveling formulas, SPECIAL effects
- `build-data-model`: Structured JSON schema and Rust domain model covering SPECIAL stats, skills, perks, traits, implants, skill books, magazines, and level progression rules
- `build-simulator`: Simulate character progression level-by-level, applying skill point allocation, perk selection, trait effects, and modifier stacking (Educated, Comprehension, implants)
- `build-planner-cli`: CLI interface that accepts exported game data JSON and player build choices (TOML), then outputs a complete level-by-level build plan

### Modified Capabilities

(none -- greenfield project)

## Impact

- **Dependencies**: xNVSE (New Vegas Script Extender) for the extractor plugin; Rust CLI ecosystem (`clap`, `serde`, `toml`) for the planner
- **Runtime requirement**: FNV must be launched once with the extractor plugin to generate the data dump; planner runs offline after that
- **No external APIs or services** -- entirely local tool
