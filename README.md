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

**BloxOS** is an operating system project developed by **BoxZero**, with the vision of building a modern, independent, and extensible computing platform.

Designed around a custom system architecture and a modern graphical environment, BloxOS aims to bring together performance, usability, customization, and system-level functionality in one cohesive platform.

The project combines low-level system development with modern desktop technologies, creating a foundation for a complete operating system rather than just a graphical desktop application.

BloxOS is actively evolving, with development focused on its kernel, system components, graphical interface, application infrastructure, and installation experience.

## Core Vision

BloxOS is being developed around several fundamental principles:

- **Modern Architecture** — A modular foundation designed for future expansion.
- **Performance** — Efficient system components and optimized resource usage.
- **User Experience** — A consistent, intuitive, and visually refined interface.
- **Customization** — Flexible desktop and system configuration.
- **Extensibility** — A foundation for applications, services, and additional system components.
- **Independence** — Building a distinctive operating system ecosystem under the BoxZero brand.

## Features & Components

The following sections describe the current project direction. Feature availability depends on the implementation status of each component.

### System Architecture

- Custom BloxOS system architecture.
- Modular organization of system components.
- Dedicated kernel development.
- Separation between core services and user-facing applications.
- Foundation for future hardware and system integrations.

### Graphical Environment

- Modern desktop interface.
- Qt Quick and QML-based graphical components.
- Desktop navigation and application launcher.
- Dock and taskbar concepts.
- Window controls and application interfaces.
- Light and dark theme support.
- Responsive desktop layouts.

### System Applications

The planned system application ecosystem includes:

- **File Manager** — Browse and organize files.
- **Terminal** — Access command-line functionality.
- **Settings** — Configure desktop and system preferences.
- **Application Launcher** — Discover and launch applications.
- **System Installer** — Install and configure BloxOS.
- **Lock Screen** — Provide a dedicated session entry interface.

### Application Infrastructure

BloxOS aims to provide a structured application environment with support for:

- System and user applications.
- Separate locations for 64-bit and 32-bit applications.
- Application management.
- Package management infrastructure.
- Future application installation and update workflows.

### Language & Localization

BloxOS is designed with localization and language integration in mind.

**BXZ-LANG** is part of the BoxZero ecosystem and is being integrated into the BloxOS installation experience, with the option to install or skip it during setup.

---

## Technology Stack

BloxOS uses a combination of technologies selected for different system layers.

| Technology | Purpose |
|---|---|
| Rust | Kernel and low-level system development |
| C | System-level functionality |
| C++ | Native system components and integrations |
| Assembly | Low-level architecture-specific operations |
| Qt 6 | Native graphical framework |
| Qt Quick | Modern interface rendering |
| QML | Desktop UI and component development |
| JavaScript | QML interaction logic |
| CMake | Build system configuration |
| Ninja | Build execution |
| NASM | Assembly development |
| GRUB / UEFI | Boot infrastructure, depending on target configuration |

*The exact technology requirements may vary between BloxOS components and build targets.*

## Architecture

BloxOS follows a modular architecture intended to keep system responsibilities separated and maintainable.

```text
BloxOS/
│
├── kernel/
│   └── Core kernel implementation
│
├── base/
│   └── Base system components
│
├── cli/
│   └── Command-line infrastructure
│
├── package-manager/
│   └── Package management system
│
├── shell/
│   └── System shell
│
├── system-services/
│   └── Background services
│
├── installer/
│   └── Operating system installation
│
├── gui/
│   └── Graphical environment
│
├── apps/
│   └── System applications
│
├── assets/
│   └── Icons, images, and visual resources
│
├── docs/
│   └── Technical documentation
│
├── CMakeLists.txt
└── README.md
```

*This diagram represents the intended architectural organization. Individual directories and components may differ in the current source tree.*

## Installation & Boot Experience

The BloxOS installation system is being designed to provide a straightforward setup process.

The planned installation experience includes:

- Operating system installation workflow.
- Installation configuration.
- Optional BXZ-LANG installation.
- System configuration.
- Boot environment integration.

The final installation capabilities depend on the development stage and target release.

## Development Environment

The development environment currently targets a Linux-based toolchain, with Windows development supported through compatible environments such as WSL.

Recommended tools:

- Qt 6
- CMake
- Ninja
- Rust toolchain
- GCC / G++
- NASM
- QEMU
- GRUB and UEFI development tools, where required

## Building from Source

Clone the repository:

```bash
git clone <YOUR-REPOSITORY-URL>
cd BloxOS
```

Configure the build:

```bash
cmake -S . -B build -G Ninja -DCMAKE_BUILD_TYPE=Debug
```

Build:

```bash
cmake --build build
```

Run the desktop target, if available:

```bash
./build/bloxos
```

### Running in QEMU

For an ISO image produced by the project:

```bash
qemu-system-x86_64 \
  -cdrom build/bloxos.iso \
  -m 512M \
  -smp 2
```

For UEFI-based testing, configure QEMU with a compatible OVMF firmware image and the appropriate boot configuration.

**Important:** These are example commands. Actual build targets, output paths, firmware requirements, and runtime options depend on the current build configuration.

## Development Roadmap

BloxOS development is organized around several major areas.

| Component | Development Focus |
|---|---|
| Kernel | Core architecture and system foundations |
| Base System | Essential system functionality |
| GUI | Desktop environment and interface |
| System Services | Background system operations |
| Installer | Installation and configuration |
| Package Manager | Application and package infrastructure |
| System Applications | Built-in desktop applications |
| Documentation | Developer and user documentation |

### Planned Improvements

- Expanded hardware compatibility.
- More complete desktop environment.
- Improved application management.
- Advanced system configuration.
- Enhanced package management.
- Expanded localization support.
- Improved security and stability.
- Additional developer tools.

*Roadmap items are development goals and should not be interpreted as completed features.*

## Development Status

**Current Status: In Development**

BloxOS is under active development. Its architecture, features, and system components are continuously evolving.

The project is not yet a final production release.

## Contributing

We welcome feedback, suggestions, bug reports, and contributions from the developer community.

You can contribute by:

1. Reporting bugs through GitHub Issues.
2. Suggesting new features.
3. Improving technical documentation.
4. Reviewing code and proposing improvements.
5. Submitting Pull Requests.

Please ensure that contributions are consistent with the project's architecture and development direction.

## About BoxZero

BloxOS is developed by **BoxZero**, an independent technology and software development initiative focused on building software, tools, and digital experiences.

🌐 Website: https://www.boxzero.ir  
📢 Telegram: https://t.me/BoxZero_company  
💬 Support: @boxzerosupport  
✉️ Email: boxzero.company@gmail.com

## License

The project license will be specified in the repository.

Until a license is added, no permission to redistribute, modify, or reuse the source code should be assumed.

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
