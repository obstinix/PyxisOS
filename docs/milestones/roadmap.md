# PyxisOS Project Roadmap

This roadmap documents the status of all past, present, and future PyxisOS subsystems.

---

## Status Legend

- `[COMPLETED]` — Built, tested, and actively functioning on physical reference hardware.
- `[TEMPORARY]` — Active and functional intermediate implementation serving Track A.
- `[IN DEVELOPMENT]` — Active codebase or working prototype currently being developed.
- `[PROTOTYPE]` — Conceptual proof-of-concept running in emulator/browser.
- `[PLANNED]` — Architected and scheduled for future phases; implementation not yet started.
- `[PAUSED]` — Exploration paused deliberately to prioritize core dependencies.

---

## Subsystem Milestones

### 1. Operating System Substrate & Identity
- `[COMPLETED]` Base Linux OS installation on ThinkPad X1 Carbon 5th Gen
- `[COMPLETED]` Distribution rebranding to PyxisOS in `/etc/os-release`
- `[COMPLETED]` Hostname definition (`pyxisos`) and user environment
- `[TEMPORARY]` Arch Linux base package layer for Track A
- `[PLANNED]` Independent PyxisOS package repository and build infrastructure

### 2. Boot & Storage Architecture
- `[COMPLETED]` UEFI systemd-boot 261.2 integration
- `[COMPLETED]` PyxisOS loader configuration (`pyxisos.conf`)
- `[COMPLETED]` Windows 10 dual-boot chainloading entry (`windows.conf`)
- `[COMPLETED]` Boot troubleshooting & NVMe root discovery scripts (`system/boot/`)
- `[TEMPORARY]` systemd-boot loader substrate
- `[PROTOTYPE]` Track B BIOS/legacy MBR bootloader in NASM (Lesson 1 completed)
- `[PLANNED]` Custom Rust UEFI bootloader for Track B

### 3. Desktop Shell & Window Management
- `[COMPLETED]` KDE Plasma 6 environment operational
- `[COMPLETED]` SDDM display manager integration
- `[TEMPORARY]` KDE Plasma as primary graphical desktop
- `[PLANNED]` Hyprland Wayland compositor integration (exploration paused)
- `[PROTOTYPE]` Nebula Engine 3D spatial multitasking & parallax interface (`nebula/prototype/`)
- `[PLANNED]` Native Bevy/Rust Wayland compositor for Track B

### 4. CLI & System Tooling
- `[COMPLETED]` PyxisOS CLI tool (`cli/pyxis`)
- `[COMPLETED]` Subcommands: `info`, `system`, `status`, `doctor`, `version`, `help`
- `[COMPLETED]` Official CLI installer script (`scripts/cli.sh`)
- `[COMPLETED]` Automated repository verification suite (`scripts/verify.sh`)
- `[PROTOTYPE]` Pyxis terminal banner prototype (`system/branding/pyxis-banner`)
- `[PAUSED]` Getty / TTY login banner integration (paused to maintain standard console)

### 5. Native Systems Architecture (Track B)
- `[IN DEVELOPMENT]` Lunar Core `#![no_std]` Rust microkernel scaffold (`native/lunar-core`)
- `[IN DEVELOPMENT]` Custom compiler target specification (`x86_64-pyxis.json`)
- `[PLANNED]` Memory subsystem (paging, heap allocator, physical frame manager)
- `[PLANNED]` Preemptive multi-priority scheduler
- `[PLANNED]` IPC message passing subsystem
- `[PLANNED]` Aegis Type-1 hypervisor / security sandbox (`native/aegis`)

### 6. Autonomous Intelligence Layer
- `[IN DEVELOPMENT]` Astral Consensus Engine multi-agent system (`consensus/`)
- `[IN DEVELOPMENT]` Specialized agents (Research, Security, Logic, Arbitration)
- `[IN DEVELOPMENT]` Consensus benchmark and evaluation suite (`research/eval/`)
- `[PLANNED]` Celestial Automation workflow engine (task DAG execution)
- `[PLANNED]` Constellation Network peer device federation

### 7. Academic Research & Publications
- `[IN DEVELOPMENT]` PyxisOS systems architecture research paper outline (`docs/research-paper/`)
- `[IN DEVELOPMENT]` Multi-agent consensus vs single-model evaluation dataset
