// Prefix header — force-included before all translation units.
// Provides base types and standard library headers that xNVSE
// headers expect to be available globally.

#pragma once

// Standard library headers used by xNVSE
#include <cstdint>
#include <string>
#include <vector>
#include <unordered_map>
#include <tuple>
#include <algorithm>

// xNVSE base integer types
#include "common/ITypes.h"
