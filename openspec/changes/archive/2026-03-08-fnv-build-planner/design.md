## Context

Fallout: New Vegas stores game data across ESM/ESP plugin files and engine binaries. At runtime, the engine parses all plugins in load order, resolves overrides, and hydrates records into in-memory C++ objects. Crucially, leveling formulas and SPECIAL-to-skill mappings are compiled into the engine — they cannot be extracted from data files alone.

xNVSE (New Vegas Script Extender) provides a plugin API that can access these runtime objects, making it the ideal extraction point. The extractor runs once in-game to dump data; the build planner operates entirely offline against that dump.

## Goals / Non-Goals

**Goals:**
- Extract complete, accurate build-relevant data from a running FNV instance via xNVSE plugin
- Produce a portable JSON file that fully describes the game's build system as loaded (including mods)
- Simulate level-by-level character progression with correct formula application
- Output a validated build plan via CLI

**Non-Goals:**
- GUI or web interface (CLI only)
- Real-time game integration or save file editing
- Combat simulation, damage calculation, or weapon/armor optimization
- Quest ordering or quest-dependent build choices
- Automatic mod detection — user must run the extractor with their desired load order active

## Decisions

### 1. Two-component architecture: Extractor + Planner

**Rationale**: Clean separation of concerns. The extractor handles the messy reality of game internals (C++ objects, engine quirks). The planner is pure domain logic against clean data. They communicate via a well-defined JSON schema. This means the planner never needs to understand ESM formats or engine internals.

**Alternatives considered**: Single Rust binary parsing ESM files — cannot access engine-level formulas. Single in-game mod doing everything — poor UX for iterating on builds (requires game running).

### 2. Extractor: xNVSE C++ plugin

**Rationale**: xNVSE is the standard extension framework for FNV modding. Its plugin API exposes game object hierarchies, form lookups, and actor value accessors. A C++ plugin can walk TESForm objects, read perk conditions, enumerate skill books, and call engine functions to extract computed values (like base skill formulas).

**Alternatives considered**: Papyrus/GECK scripting — too limited for bulk data extraction. JIP LN NVSE functions via console commands — possible but fragile and incomplete.

### 3. Data interchange: JSON with defined schema

**Rationale**: JSON is human-readable, easily consumed by Rust (`serde_json`), and can be version-controlled or shared. A defined schema ensures the extractor and planner stay in sync. The schema acts as the contract between the two components.

### 4. Planner: Rust CLI with TOML input

**Rationale**: Rust for performance and type safety on the simulation logic. TOML for build plans because it's human-readable and editable — players need to specify 30-50 levels of choices. CLI keeps the tool simple and scriptable.

### 5. Formulas: Extract where possible, hardcode with documentation where not

**Rationale**: Some formulas (skill points per level) can be extracted or validated at runtime by observing engine behavior. Others may require hardcoding. All hardcoded values will be clearly documented and sourced, and the JSON schema includes a `formulas` section so the extractor can override defaults when it can extract the real values.

## Risks / Trade-offs

- **xNVSE API coverage** → Some game data may not be directly accessible through the xNVSE API. Mitigation: JIP LN NVSE and other extension plugins expose additional functions. Research API coverage before implementation.
- **xNVSE version compatibility** → Plugin must target a specific xNVSE version. Mitigation: Target the latest stable xNVSE release and document the minimum version.
- **Incomplete formula extraction** → Engine formulas may be opaque even at runtime. Mitigation: The `formulas` section in JSON allows partial extraction. Hardcoded fallbacks documented with sources (GECK wiki, UESP).
- **User must run the game** → The extractor requires launching FNV. Mitigation: Extraction is a one-time step per load order; include a pre-extracted vanilla JSON for users who just want the base game.
- **C++ plugin development** → Higher barrier to entry than pure Rust. Mitigation: Keep the plugin minimal — its only job is data extraction and JSON serialization. All build logic lives in the Rust planner.
