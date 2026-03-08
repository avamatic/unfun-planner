# BuildDataExtractor — xNVSE Plugin Setup

## Prebuilt DLL

The easiest way to get the plugin is from CI artifacts:

1. Go to the [Actions tab](../../actions) on GitHub
2. Open the latest successful build
3. Download the `BuildDataExtractor` artifact
4. Copy `BuildDataExtractor.dll` to `<Fallout NV>/Data/NVSE/Plugins/`

## Building Locally

### Prerequisites

- Visual Studio 2022 (MSVC v143 toolset, Win32/x86) or just the Build Tools
- CMake 3.20+
- Fallout: New Vegas with [xNVSE](https://github.com/xNVSE/NVSE) installed (for testing)

### Steps

```bash
# Clone the xNVSE source (from the repo root)
git clone --depth 1 https://github.com/xNVSE/NVSE.git xNVSE

# Configure and build
cmake -B build -S extractor -A Win32 -DNVSE_ROOT=xNVSE
cmake --build build --config Release
```

The DLL is output to `build/Release/BuildDataExtractor.dll`.

For `TESObjectBOOK` support: the full class definition may need to be
copied from [ShowOff-NVSE's GameForms.h](https://github.com/Demorome/ShowOff-NVSE)
since the official xNVSE only forward-declares it.

## Install

Copy `BuildDataExtractor.dll` to:
```
<Fallout NV>/Data/NVSE/Plugins/BuildDataExtractor.dll
```

## Usage

1. Launch Fallout: New Vegas through xNVSE (`nvse_loader.exe`)
2. Open the console (`~` key)
3. Type: `ExtractBuildData`
4. Data is written to `<Fallout NV>/Data/build_data.json`

Optional: specify a custom output path:
```
ExtractBuildData "C:\builds\my_modded_data.json"
```

## Notes

- The opcode base `0x3A00` is temporary — register your own at the
  [xNVSE Opcode Registry](https://geckwiki.com/index.php?title=NVSE_Opcode_Base)
  before releasing
- Skill magazines are not yet auto-extracted (they're ingestibles with
  script effects, which require magic effect parsing). Use the reference
  data or add them manually.
- The `extract.cpp` condition parsing for perk prerequisites needs
  refinement against the actual xNVSE `Condition` struct — verify
  against the specific xNVSE version you're building against.
