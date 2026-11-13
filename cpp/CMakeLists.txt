
cmake_minimum_required(VERSION 3.20)

project(BloxOSComponents LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_library(bloxos_components STATIC
    components/system_bridge.cpp
    libraries/graphics.cpp
)

target_include_directories(bloxos_components PUBLIC
    components
    libraries
)
