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

These components are part of the broader BloxOS graphical environment and are being developed alongside the underlying system infrastructure.

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

The planned application infrastructure includes:

- Separate application locations.
- User and system file organization.
- Application management.
- Package management.
- Future application installation and update support.

## BXZ-LANG Integration

**BXZ-LANG** is a language project developed within the BoxZero ecosystem.

BloxOS is designed to integrate BXZ-LANG into its installation experience.

During installation, users will be able to choose whether to install BXZ-LANG or continue without it.

This integration is part of the broader BoxZero ecosystem and its localization direction.

## Technology Stack

The technologies currently associated with BloxOS development include:

| Technology | Purpose |
|---|---|
| Rust | Kernel and low-level system development |
| x86_64 | Target processor architecture |
| Multiboot2 | Kernel boot information and boot protocol |
| Framebuffer | Low-level graphical output |
| GRUB | Bootloader infrastructure |
| UEFI | Modern boot environment testing |
| QEMU | Virtual machine and boot testing |

The technology stack may evolve as additional system components are implemented.

## Project Architecture

BloxOS is organized around separate system responsibilities.

The following diagram represents the project's architectural direction:

```text
BloxOS/
│
├── kernel/
│   └── Kernel implementation
│
├── base/
│   └── Essential system components
│
├── cli/
│   └── Command-line infrastructure
│
├── package-manager/
│   └── Package management
│
├── shell/
│   └── System shell
│
├── system-services/
│   └── Background system services
│
├── installer/
│   └── Operating system installer
│
├── gui/
│   └── Graphical rendering and desktop components
│
├── apps/
│   └── System applications
│
├── assets/
│   └── Icons and visual resources
│
├── docs/
│   └── Technical documentation
│
└── README.md
```

This is a high-level architectural overview, not a verified listing of every directory currently present in the repository.

## Building & Boot Testing

BloxOS development includes generating bootable ISO images and testing them in a virtual machine.

### QEMU Testing

A basic example for testing a generated ISO:

```bash
qemu-system-x86_64 \
  -cdrom build/bloxos-27H1-Horizon.iso \
  -m 512M \
  -smp 1
```

### UEFI Testing

BloxOS also undergoes UEFI boot testing with a compatible firmware image.

Example configuration:

```bash
qemu-system-x86_64 \
  -machine q35 \
  -m 512M \
  -smp 1 \
  -drive if=pflash,format=raw,unit=0,readonly=on,file="PATH_TO_UEFI_CODE.fd" \
  -cdrom build/bloxos-27H1-Horizon.iso \
  -boot d
```

Replace the firmware path with the location of the compatible UEFI firmware on your system.

**Note:** These commands are examples for testing an existing ISO image. Exact compilation and ISO-generation instructions will be documented alongside the actual build configuration and source files.

## Development Roadmap

BloxOS development is divided into several major areas.

| Area | Development Focus |
|---|---|
| Kernel | Kernel initialization and core functionality |
| Boot Infrastructure | Bootloader and UEFI integration |
| Graphics | Framebuffer and rendering infrastructure |
| Desktop | Desktop environment and system interface |
| Base System | Essential operating system components |
| System Services | Background system functionality |
| Installer | Installation and initial configuration |
| File Explorer | File and directory management |
| Package Manager | Application and package infrastructure |
| System Applications | Integrated operating system utilities |

### Future Development Goals

- Expanded hardware support.
- More complete kernel functionality.
- Improved graphics infrastructure.
- Advanced desktop and window management.
- Integrated system applications.
- Application installation and management.
- Expanded localization.
- Improved system stability and security.

Roadmap items are planned development goals, not claims of completed functionality.

## Development Status

**Current Status: In Development**

BloxOS is actively being developed by BoxZero.

Current work includes kernel development, boot infrastructure, framebuffer rendering, graphical system components, and virtual machine testing.

The project is still evolving and is not yet a final production release.

### Target Release

**BloxOS 27H1 Horizon**

Target release: **March 2027**

Release timing and feature availability may change as development progresses.

## Contributing

We welcome feedback, suggestions, bug reports, and contributions from the developer community.

You can contribute by:

1. Reporting bugs through GitHub Issues.
2. Suggesting improvements.
3. Improving technical documentation.
4. Reviewing source code.
5. Submitting Pull Requests.

Contributions should follow the project's architecture and development direction.

## About BoxZero

BloxOS is developed by **BoxZero**, an independent technology and software development initiative focused on building software, tools, and digital experiences.

### Official Links

- 🌐 Website: https://www.boxzero.ir
- 📢 Telegram: https://t.me/BoxZero_company
- 💬 Support: @boxzerosupport
- ✉️ Email: boxzero.company@gmail.com

## License

BloxOS is licensed under the **MIT License**.

See the [LICENSE](LICENSE) file for the complete license text.

---

<p align="center">
  <strong>BloxOS</strong>
  <br />
  Beyond the Desktop.
  <br /><br />
  <strong>Developed by BoxZero</strong>
  <br />
  Building Beyond Limits.
  <br /><br />
  © 2026 BoxZero. All rights reserved.
</p>
