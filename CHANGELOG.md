# Changelog

All notable changes to the PyxisOS project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [0.1.0] - 2026-09-23

### Added
- Track A base operating system substrate operational on Lenovo ThinkPad X1 Carbon 5th Gen.
- PyxisOS operating system branding and metadata specification in `/etc/os-release`.
- UEFI dual-boot configuration with Windows 10 managed via systemd-boot 261.2.
- Dedicated PyxisOS EFI boot loader entry (`pyxisos.conf`).
- KDE Plasma 6 desktop environment with SDDM graphical display manager.
- NetworkManager network service integration.
- PyxisOS management CLI tool (`cli/pyxis`) supporting `info`, `system`, `status`, `doctor`, `version`, and `help`.
- Automated Pyxis CLI installer script (`scripts/cli.sh`).
- Automated repository and system component verification suite (`scripts/verify.sh`).
- PyxisOS terminal banner prototype (`system/branding/pyxis-banner`).
- Structured milestone tracking (`docs/milestones/`).
- Hardware reference and dual-boot installation guides (`docs/installation/`).

### Changed
- Refactored repository structure to conform to modern Linux operating-system project standards.
- Realigned documentation with two-track delivery model (Track A: Linux substrate; Track B: from-scratch native kernel/hypervisor).
- Boot debug procedures moved to `system/boot/pyxisos_boot_commands.sh`.
