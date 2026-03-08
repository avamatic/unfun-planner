cmake_minimum_required(VERSION 3.20)
project(BuildDataExtractor LANGUAGES C CXX)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# xNVSE source location — set via -DNVSE_ROOT=<path> or auto-detected
# Expected layout: NVSE_ROOT/nvse/nvse/PluginAPI.h
if(NOT NVSE_ROOT)
    if(EXISTS "${CMAKE_CURRENT_SOURCE_DIR}/../xNVSE/nvse/nvse/PluginAPI.h")
        set(NVSE_ROOT "${CMAKE_CURRENT_SOURCE_DIR}/../xNVSE")
    elseif(EXISTS "${CMAKE_CURRENT_SOURCE_DIR}/xNVSE/nvse/nvse/PluginAPI.h")
        set(NVSE_ROOT "${CMAKE_CURRENT_SOURCE_DIR}/xNVSE")
    else()
        message(FATAL_ERROR
            "xNVSE source not found. Set -DNVSE_ROOT=<path> to the xNVSE repo root.\n"
            "Clone it with: git clone https://github.com/xNVSE/NVSE.git xNVSE")
    endif()
endif()

# Normalize backslashes to forward slashes (Windows paths break CMake string parsing)
cmake_path(SET NVSE_ROOT NORMALIZE "${NVSE_ROOT}")

message(STATUS "Using xNVSE source from: ${NVSE_ROOT}")

set(NVSE_DIR "${NVSE_ROOT}/nvse/nvse")
set(NVSE_COMMON_DIR "${NVSE_ROOT}/common")

# --- xNVSE common library sources ---
set(NVSE_COMMON_SOURCES
    ${NVSE_COMMON_DIR}/IDebugLog.cpp
    ${NVSE_COMMON_DIR}/IFileStream.cpp
    ${NVSE_COMMON_DIR}/IErrors.cpp
)

# --- xNVSE source files compiled into the plugin ---
# Must match what nvse_plugin_example.vcxproj compiles
set(NVSE_SOURCES
    ${NVSE_DIR}/containers.cpp
    ${NVSE_DIR}/GameAPI.cpp
    ${NVSE_DIR}/GameBSExtraData.cpp
    ${NVSE_DIR}/GameData.cpp
    ${NVSE_DIR}/GameExtraData.cpp
    ${NVSE_DIR}/GameForms.cpp
    ${NVSE_DIR}/GameObjects.cpp
    ${NVSE_DIR}/GameOSDepend.cpp
    ${NVSE_DIR}/GameProcess.cpp
    ${NVSE_DIR}/GameRTTI.cpp
    ${NVSE_DIR}/GameScript.cpp
    ${NVSE_DIR}/GameSettings.cpp
    ${NVSE_DIR}/GameTasks.cpp
    ${NVSE_DIR}/GameTiles.cpp
    ${NVSE_DIR}/GameTypes.cpp
    ${NVSE_DIR}/NiTypes.cpp
    ${NVSE_DIR}/printf.cpp
    ${NVSE_DIR}/SafeWrite.cpp
    ${NVSE_DIR}/Utilities.cpp
    ${NVSE_DIR}/utility.cpp
)

# Filter out any that don't exist (in case of version differences)
set(NVSE_SOURCES_FILTERED)
foreach(src ${NVSE_SOURCES})
    if(EXISTS "${src}")
        list(APPEND NVSE_SOURCES_FILTERED "${src}")
    else()
        message(WARNING "xNVSE source not found, skipping: ${src}")
    endif()
endforeach()

# --- Plugin source files ---
set(PLUGIN_SOURCES
    main.cpp
    extract.cpp
    json_writer.cpp
)

# Filter common sources too
set(NVSE_COMMON_FILTERED)
foreach(src ${NVSE_COMMON_SOURCES})
    if(EXISTS "${src}")
        list(APPEND NVSE_COMMON_FILTERED "${src}")
    else()
        message(WARNING "xNVSE common source not found, skipping: ${src}")
    endif()
endforeach()

add_library(BuildDataExtractor SHARED
    ${PLUGIN_SOURCES}
    ${NVSE_SOURCES_FILTERED}
    ${NVSE_COMMON_FILTERED}
    exports.def
)

target_include_directories(BuildDataExtractor PRIVATE
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${NVSE_ROOT}
    ${NVSE_ROOT}/nvse
    ${NVSE_DIR}
    ${NVSE_COMMON_DIR}
    ${NVSE_ROOT}/common
)

target_compile_definitions(BuildDataExtractor PRIVATE RUNTIME)

if(MSVC)
    target_compile_options(BuildDataExtractor PRIVATE /W3 /MP)
    # Force-include prefix header (base types + std library headers xNVSE expects)
    target_compile_options(BuildDataExtractor PRIVATE /FI"${CMAKE_CURRENT_SOURCE_DIR}/prefix.h")
endif()

set_target_properties(BuildDataExtractor PROPERTIES
    PREFIX ""
    OUTPUT_NAME "BuildDataExtractor"
)
