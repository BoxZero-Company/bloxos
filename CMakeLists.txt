cmake_minimum_required(VERSION 3.20)

project(BloxOS
    VERSION 0.1.0
    DESCRIPTION "BloxOS desktop and native system components"
    LANGUAGES C CXX
)

set(CMAKE_C_STANDARD 17)
set(CMAKE_C_STANDARD_REQUIRED ON)
set(CMAKE_C_EXTENSIONS OFF)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

include(CTest)

add_library(bloxos_base STATIC
    base/src/platform.c
)

target_include_directories(bloxos_base
    PUBLIC "${CMAKE_CURRENT_SOURCE_DIR}/base/include"
)

add_executable(bloxctl
    cli/src/main.cpp
)

target_link_libraries(bloxctl PRIVATE bloxos_base)

# Desktop: use the existing QML interface when Qt 6 is installed.
find_package(Qt6 6.2 QUIET COMPONENTS Quick)

if(Qt6_FOUND)
    add_executable(bloxos-desktop
        shell/src/main.cpp
    )

    target_link_libraries(bloxos-desktop
        PRIVATE Qt6::Quick
    )

    target_compile_definitions(bloxos-desktop
        PRIVATE
        BLOXOS_MAIN_QML="${CMAKE_CURRENT_SOURCE_DIR}/shell/Main.qml"
    )

    message(STATUS "BloxOS desktop enabled: Qt 6 Quick")
else()
    message(STATUS
        "Qt 6 development files not found; desktop target disabled"
    )
endif()

# Rust is built with Cargo, independently of the CMake C/C++ targets.
find_program(CARGO_EXECUTABLE cargo)

if(CARGO_EXECUTABLE)
    add_custom_target(bloxos-rust
        COMMAND "${CARGO_EXECUTABLE}" build
            --manifest-path
            "${CMAKE_CURRENT_SOURCE_DIR}/system-services/rust/Cargo.toml"
        WORKING_DIRECTORY "${CMAKE_CURRENT_SOURCE_DIR}"
        USES_TERMINAL
        COMMENT "Building BloxOS Rust services"
    )
endif()

# NASM is optional. This produces an ELF object, not a bootable kernel.
find_program(NASM_EXECUTABLE nasm)

if(NASM_EXECUTABLE AND CMAKE_SYSTEM_PROCESSOR MATCHES "x86_64|amd64|AMD64")
    set(BLOXOS_ASM_OBJECT "${CMAKE_CURRENT_BINARY_DIR}/bloxos_asm.o")

    add_custom_command(
        OUTPUT "${BLOXOS_ASM_OBJECT}"
        COMMAND "${NASM_EXECUTABLE}" -f elf64
            "${CMAKE_CURRENT_SOURCE_DIR}/base/boot/x86_64/utility.asm"
            -o "${BLOXOS_ASM_OBJECT}"
        DEPENDS base/boot/x86_64/utility.asm
        VERBATIM
    )

    add_custom_target(bloxos-asm
        DEPENDS "${BLOXOS_ASM_OBJECT}"
    )
endif()

if(BUILD_TESTING)
    add_test(NAME bloxctl-version COMMAND bloxctl --version)
endif()

message(STATUS "Configuring BloxOS ${PROJECT_VERSION}")


# Bootable x86_64 kernel (requires NASM, Rust target, and GRUB tools for ISO).
find_program(BLOXOS_MAKE_EXECUTABLE make)
if(NASM_EXECUTABLE AND CARGO_EXECUTABLE AND BLOXOS_MAKE_EXECUTABLE)
    add_custom_target(bloxos-kernel
        COMMAND "${BLOXOS_MAKE_EXECUTABLE}" -f boot/Makefile kernel
        WORKING_DIRECTORY "${CMAKE_CURRENT_SOURCE_DIR}"
        USES_TERMINAL
        COMMENT "Building the BloxOS Multiboot2 x86_64 kernel"
    )
    add_custom_target(bloxos-iso
        COMMAND "${BLOXOS_MAKE_EXECUTABLE}" -f boot/Makefile iso
        WORKING_DIRECTORY "${CMAKE_CURRENT_SOURCE_DIR}"
        USES_TERMINAL
        COMMENT "Creating a bootable BloxOS ISO"
    )
else()
    message(STATUS "BloxOS boot targets require NASM, Cargo, and make")
endif()

find_package(Python3 COMPONENTS Interpreter QUIET)
if(Python3_Interpreter_FOUND)
    add_custom_target(bloxos-package-example
        COMMAND "${Python3_EXECUTABLE}"
            "${CMAKE_CURRENT_SOURCE_DIR}/package-manager/src/bloxpm.py"
            verify "${CMAKE_CURRENT_SOURCE_DIR}/package-manager/examples/BloxOS-Calculator-1.0.0.blox"
        VERBATIM
        COMMENT "Verifying the sample .blox package"
    )
    if(BUILD_TESTING)
        add_test(NAME bloxpm-tests
            COMMAND "${Python3_EXECUTABLE}" -m unittest discover
                -s "${CMAKE_CURRENT_SOURCE_DIR}/package-manager/tests" -v)
    endif()
endif()
