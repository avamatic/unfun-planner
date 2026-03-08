cmake_minimum_required(VERSION 3.20)
project(BuildDataExtractor LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# xNVSE source location — set via -DNVSE_ROOT=<path> or auto-detected
# Expected layout: NVSE_ROOT/nvse/nvse/PluginAPI.h
if(NOT NVSE_ROOT)
    # Check common locations
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

# --- xNVSE source files compiled into the plugin ---
set(NVSE_SOURCES
    ${NVSE_DIR}/GameAPI.cpp
    ${NVSE_DIR}/GameData.cpp
    ${NVSE_DIR}/GameForms.cpp
    ${NVSE_DIR}/GameObjects.cpp
    ${NVSE_DIR}/GameTypes.cpp
    ${NVSE_DIR}/CommandTable.cpp
    ${NVSE_DIR}/Utilities.cpp
    ${NVSE_DIR}/GameBSExtraData.cpp
)

# --- Plugin source files ---
set(PLUGIN_SOURCES
    main.cpp
    extract.cpp
    json_writer.cpp
)

add_library(BuildDataExtractor SHARED
    ${PLUGIN_SOURCES}
    ${NVSE_SOURCES}
    exports.def
)

target_include_directories(BuildDataExtractor PRIVATE
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${NVSE_ROOT}
    ${NVSE_ROOT}/nvse
    ${NVSE_DIR}
    ${NVSE_COMMON_DIR}
)

target_compile_definitions(BuildDataExtractor PRIVATE RUNTIME)

if(MSVC)
    target_compile_options(BuildDataExtractor PRIVATE /W3 /MP)
endif()

set_target_properties(BuildDataExtractor PROPERTIES
    PREFIX ""
    OUTPUT_NAME "BuildDataExtractor"
)
