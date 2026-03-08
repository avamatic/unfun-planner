// Prefix header — force-included before all translation units.
// Provides base types and standard library headers that xNVSE
// headers expect to be available globally.

#pragma once

// Windows API (xNVSE uses CRITICAL_SECTION, InterlockedIncrement, etc.)
#define WIN32_LEAN_AND_MEAN
#include <windows.h>

// Standard library headers used by xNVSE
#include <cstdint>
#include <cstdio>
#include <cstring>
#include <string>
#include <vector>
#include <map>
#include <set>
#include <list>
#include <unordered_map>
#include <unordered_set>
#include <tuple>
#include <algorithm>
#include <functional>
#include <memory>

// xNVSE base integer types
#include "common/ITypes.h"

// xNVSE containers (UnorderedMap used in CommandTable.h)
#include "nvse/nvse/containers.h"
