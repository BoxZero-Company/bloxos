# BloxOS

<p align="center">
  <strong>BloxOS — Beyond the Desktop.</strong>
  <br />
  A new vision for the next generation of operating systems.
</p>

<p align="center">
  <img src="assets/bloxos-banner.png" alt="BloxOS Banner" width="100%" />
</p>

<p align="center">
  <img src="https://img.shields.io/badge/Project-BloxOS-171717?style=for-the-badge" alt="Project" />
  <img src="https://img.shields.io/badge/Developer-BoxZero-9BFF36?style=for-the-badge" alt="Developer" />
  <img src="https://img.shields.io/badge/Status-In%20Development-blue?style=for-the-badge" alt="Status" />
  <img src="https://img.shields.io/badge/Release-Target%202027-805AD5?style=for-the-badge" alt="Target Release" />
</p>

---

## About BloxOS

**BloxOS** is an independent operating system project developed by **BoxZero**.

BloxOS is being built as a real operating system, with its own kernel development, boot process, memory-level operations, framebuffer rendering, and graphical environment.

The project focuses on developing the essential layers of an operating system rather than building a desktop application on top of an existing graphical framework.

Its development combines low-level Rust programming, x86_64 architecture support, boot infrastructure, and custom graphical system components.

The long-term goal is to create a modern, independent, and extensible operating system under the BoxZero ecosystem.

## Core Vision

BloxOS is built around the following principles:

- **System Independence** — Developing dedicated operating system components.
- **Low-Level Engineering** — Working directly with kernel-level functionality and hardware interfaces.
- **Performance** — Building efficient system components with careful resource management.
- **Modern Experience** — Creating a consistent and visually refined desktop environment.
- **Modularity** — Keeping system components organized and extensible.
- **Long-Term Development** — Establishing a foundation for future system capabilities.

## Current Development

BloxOS development currently focuses on its kernel, boot process, graphical rendering infrastructure, and system foundations.

### Kernel Development

The BloxOS kernel is being developed in Rust.

Current implementation areas include:

- A `no_std` kernel environment.
- A `no_main` entry point.
- x86_64 target architecture.
- Multiboot2 boot information handling.
- Kernel initialization.
- Low-level system operations.
- Framebuffer integration.

The kernel is one of the central components of BloxOS and provides the foundation for future system functionality.

### Boot Infrastructure

BloxOS includes a dedicated boot and startup development process.

Current development and testing involve:

- Multiboot2 boot support.
- GRUB-based boot configurations.
- ISO image generation.
- UEFI-based virtual machine testing.
- Kernel loading and startup.
- QEMU boot testing.

The boot infrastructure is being developed to provide a reliable transition from the boot environment to the BloxOS kernel.

### Graphics & Framebuffer

BloxOS includes its own graphical rendering development.

The current implementation includes:

- Framebuffer access.
- Pixel format handling.
- Framebuffer-based rendering infrastructure.
- Rust-based graphical modules.
- Integration between the kernel and graphical components.

The graphical system is being developed as part of the operating system itself.

### Desktop Environment

The BloxOS desktop environment is under active development.

Its intended components include:

- Desktop interface.
- Start Menu.
- Taskbar.
- TopBar.
- Window management.
- Application launcher.
- Lock Screen.
- System notifications.
- Integrated system applications.

The current desktop has an early graphical implementation. Other components remain under development.

## Planned System Applications

The following applications are part of the BloxOS development direction:

| Component | Purpose |
|---|---|
| File Explorer | Browse and organize files and directories. |
| Terminal | Access system commands and command-line functionality. |
| Settings | Configure operating system preferences. |
| Start Menu | Access and launch applications. |
| System Installer | Install and configure BloxOS. |
| Lock Screen | Provide a dedicated session entry interface. |

These applications are development goals and should not be interpreted as fully implemented components.

## File System & Application Organization

BloxOS is planned to provide a structured environment for system files, applications, and user data.

The intended user-facing organization includes:

```text
BloxOS/
│
├── Program Files/
│   └── 64-bit applications
│
├── Program Files (x86)/
│   └── 32-bit applications
│
└── Users/
    └── User accounts and personal files
```

This is a proposed organization for the future system environment, not a claim that the complete directory structure is already implemented.

The planned application infrastructure includes:

- Separate application locations.
- User and system file organization.
- Application management.
- Package management.
- Future application installation and update support.

## BXZ-LANG Integration

**BXZ-LANG** is a programming language project developed within the BoxZero ecosystem.

BloxOS is designed to integrate BXZ-LANG into its installation experience.

The planned installation flow will allow users to choose whether to install BXZ-LANG or continue without it.

This integration is part of the broader BoxZero ecosystem and its localization direction.

## Technology Stack

The technologies currently associated with BloxOS development include:

| Technology | Purpose |
|---|---|
| Rust | Kernel and low-level system development |
| C / C++ | Supporting system and graphics components |
| NASM | Assembly-level boot components |
| x86_64 | Target processor architecture |
| Multiboot2 | Kernel boot information and boot protocol |
| Framebuffer | Low-level graphical output |
| GRUB | Bootloader infrastructure |
| UEFI | Modern boot environment testing |
| QEMU | Virtual machine and boot testing |
| Cargo | Rust workspace and build management |
| CMake | Supporting native components |

The technology stack may evolve as additional system components are implemented.

## Project Architecture

BloxOS is organized around separate system responsibilities.

The following diagram represents the project's architectural direction and may evolve during development:

```text
bloxos/
│
├── assets/
│   └── Icons, fonts, wallpapers and visual resources
│
├── base/
│   └── Essential system components
│
├── boot/
│   └── Bootloader and startup infrastructure
│
├── cli/
│   └── Command-line infrastructure
│
├── cpp/
│   └── Native system and graphics components
│
├── desktop/
│   └── Desktop environment components
│
├── filesystem/
│   └── Filesystem implementation
│
├── gui/
│   └── Graphical rendering infrastructure
│
├── installer/
│   └── System installation components
│
├── kernel/
│   └── Kernel implementation
│
├── package-manager/
│   └── Package management infrastructure
│
├── shell/
│   └── System shell
│
├── system/
│   └── Core system components
│
├── system-services/
│   └── Background system services
│
├── Cargo.toml
├── CMakeLists.txt
├── Makefile
├── build-all.cmd
├── build-all.sh
├── CONTRIBUTING.md
├── COPYRIGHT
├── LICENSE
├── README.md
└── SECURITY.md
```

This overview describes the current project organization. Individual components may be reorganized as development progresses.

## Building & Boot Testing

BloxOS provides build scripts for supported development environments.

### Windows

Run the following command from the repository root:

```powershell
.\build-all.cmd
```

### Linux

Make the build script executable and run it:

```bash
chmod +x build-all.sh
./build-all.sh
```

Build requirements may vary depending on the operating system, installed toolchain, and environment configuration.

### QEMU Testing

After successfully generating an ISO image, it can be tested using QEMU:

```bash
qemu-system-x86_64 \
  -cdrom build/bloxos-27H1-Horizon.iso \
  -m 512M
