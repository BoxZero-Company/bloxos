# Contributing to BloxOS

First of all, thank you for your interest in contributing to **BloxOS 27H1 Horizon**!

BloxOS is an open-source operating system project developed by **BoxZero**. We welcome developers, testers, designers, and enthusiasts who want to help improve the project.

Our goal is to build a modern desktop operating system through collaboration and open development.

## Getting Started

Before contributing, make sure you have:

- Git
- Rust and Cargo
- NASM
- GRUB utilities
- QEMU
- xorriso
- mtools
- A compatible development environment

Additional dependencies may be required depending on your operating system.

## Technology Stack

BloxOS uses several technologies:

- **Rust:** Kernel and core system development.
- **C / C++:** System and graphics components.
- **NASM:** Low-level assembly and boot components.
- **GRUB:** Bootloader and Multiboot2 support.
- **QEMU:** Virtual machine testing.

## Project Structure

The repository contains the main components of BloxOS, including:

```text
BloxOS/
├── boot/
├── kernel/
├── gui/
├── desktop/
├── system/
├── filesystem/
├── installer/
├── package-manager/
├── assets/
├── docs/
├── scripts/
├── README.md
├── LICENSE
├── CONTRIBUTING.md
└── SECURITY.md
```

Some components are still under development and their structure may change.

## How to Contribute

### 1. Fork the Repository

Fork the official repository:

https://github.com/BoxZero-Company/bloxos

### 2. Clone Your Fork

```bash
git clone https://github.com/YOUR-USERNAME/bloxos.git
cd bloxos
```

### 3. Create a Branch

```bash
git checkout -b feature/your-feature
```

### 4. Make Your Changes

Keep your changes focused and follow the existing coding conventions.

### 5. Test Your Changes

Before submitting a Pull Request:

- Build the project.
- Check for compilation errors.
- Test changes in QEMU when applicable.
- Verify that existing functionality has not been broken.

On supported Windows environments, use:

```cmd
build-all.cmd
```

On Linux:

```bash
./build-all.sh
```

Build scripts and dependencies may differ across platforms.

### 6. Submit a Pull Request

Push your branch:

```bash
git push origin feature/your-feature
```

Then open a Pull Request against the official BloxOS repository.

## Reporting Bugs

If you discover a bug, please open a GitHub Issue:

https://github.com/BoxZero-Company/bloxos/issues

Include:

- A clear description of the issue.
- Steps to reproduce it.
- Expected behavior.
- Actual behavior.
- Your environment and relevant logs.
- Screenshots when useful.

## Development Guidelines

- Write clean and maintainable code.
- Keep changes focused.
- Avoid unnecessary dependencies.
- Document important changes.
- Respect the existing project architecture.
- Do not commit generated build artifacts unless requested.
- Never commit passwords, API keys, or other secrets.

## Current Development Status

BloxOS is an actively developed project.

Some features, including the installer and recovery environment, are not yet fully implemented.

Contributions toward improving these components are welcome.

## Community

You can follow BoxZero and BloxOS through the official channels:

- Website: https://boxzero.ir
- GitHub: https://github.com/BoxZero-Company
- Instagram: https://instagram.com/boxzero.company
- Telegram: https://t.me/BoxZero_company

## Thank You

Every contribution matters, whether it is a bug report, documentation improvement, code contribution, or testing feedback.

Thank you for helping us build BloxOS.

---

**BloxOS 27H1 Horizon**

Developed by BoxZero.

*Building Beyond Limits.*