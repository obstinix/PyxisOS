# PyxisOS

<div align="center">

```
    ____             _      ____  _____
   / __ \__  ___  __(_)____/ __ \/ ___/
  / /_/ / / / / |/_/ / ___/ / / /\__ \ 
 / ____/ /_/ />  </ (__  ) /_/ /___/ / 
/_/    \__, /_/|_/_/____/\____//____/  
      /____/                           
```

**LUNAR-PYXIS // BETA**  
*An experimental custom Linux operating system built on an Arch Linux base, evolving toward autonomous multi-agent intelligence and a native microkernel architecture.*

[![Version](https://img.shields.io/badge/version-0.1.0-38%3B2%3B220%3B40%3B80.svg)](VERSION)
[![Stage](https://img.shields.io/badge/stage-beta-crimson.svg)](docs/milestones/current-state.md)
[![Codename](https://img.shields.io/badge/codename-Lunar--Pyxis-black.svg)](docs/milestones/current-state.md)
[![Base](https://img.shields.io/badge/base-Arch%20Linux%20Substrate-blue.svg)](docs/milestones/temporary.md)
[![License](https://img.shields.io/badge/license-Apache--2.0-black.svg)](LICENSE)
[![Target](https://img.shields.io/badge/target-x86__64-lightgrey.svg)](docs/installation/hardware-setup.md)

</div>

---

## 1. What is PyxisOS?

**PyxisOS** is an experimental operating-system project designed to bridge conventional Linux system stability with modern multi-agent reasoning and spatial window management.

Unlike standard distributions that treat artificial intelligence merely as application-level chatbots or background daemons, PyxisOS reframes the operating system pipeline:

$$\text{User} \longrightarrow \text{Astral Consensus Engine} \longrightarrow \text{Celestial Automation} \longrightarrow \text{OS Runtime} \longrightarrow \text{Hardware}$$

To ensure steady progress without blocking usability on long-term kernel development, PyxisOS employs a **two-track delivery strategy**:
- **Track A (Current Functional Implementation):** A custom Linux OS built on top of an Arch Linux base substrate running on physical x86_64 hardware. It delivers daily operational reliability, hardware driver support, UEFI dual boot, KDE Plasma, SDDM, and the standalone `pyxis` CLI management tool.
- **Track B (Long-Term Native Vision):** A from-scratch systems engineering effort comprising a custom Rust microkernel (**Lunar Core**), a Type-1 hypervisor/sandbox (**Aegis**), and a native 3D spatial compositor (**Nebula Engine**).

---

## 2. Current Project Status

- **Release Stage:** Beta
- **Version:** `0.1.0`
- **Codename:** `Lunar-Pyxis`
- **Reference Hardware:** Lenovo ThinkPad X1 Carbon 5th Gen (Intel Core i7-7600U, 16 GB RAM)
- **Active Kernel:** Linux `7.1.11-arch1-1` (x86_64)
- **Primary Delivery Track:** Track A operational on physical hardware; Track B in active research and prototyping.

---

## 3. Architecture & Subsystems

```text
┌────────────────────────────────────────────────────────────────────────┐
│                        PyxisOS Shell Layer                             │
│                                                                        │
│   Stellar Canvas (Desktop UX)        Astral Consensus Engine           │
│   • KDE Plasma 6 (Track A)           • 8 Specialized Reasoning Agents  │
│   • Nebula 3D Spatial (Prototype)    • Decision Arbitration Layer      │
├────────────────────────────────────────────────────────────────────────┤
│                       Automation & Management                          │
│                                                                        │
│   Celestial Automation Engine        Pyxis Management Suite            │
│   • Task Graph (DAG) Execution       • Pyxis CLI (/usr/local/bin/pyxis)│
│   • User-visible Approval Gates      • Pyxis Doctor Health Diagnostics │
├────────────────────────────────────────────────────────────────────────┤
│                       Runtime Host Substrate                           │
│                                                                        │
│   Track A (Active Operational Substrate):                              │
│   • Arch Linux Userspace Substrate + Linux 7.1.11 Kernel               │
│   • systemd-boot 261.2 + Windows 10 Dual Boot                          │
│   • SDDM + NetworkManager                                              │
│                                                                        │
│   Track B (Future Native Substrate - In Development):                  │
│   • Lunar Core (Rust Microkernel, #![no_std])                          │
│   • Aegis (Microkernel Hypervisor / Security Sandbox)                  │
├────────────────────────────────────────────────────────────────────────┤
│                          Physical Hardware                             │
│   Lenovo ThinkPad X1 Carbon 5th Gen (NVMe, Intel i7-7600U, 16GB RAM)   │
└────────────────────────────────────────────────────────────────────────┘
```

---

## 4. Current Desktop Environment

- **Current Shell:** KDE Plasma 6
- **Display Manager:** SDDM (Simple Desktop Display Manager)
- **Session Types:** Wayland / X11
- **Future Spatial Shell:** [Nebula Engine](nebula/) — an interactive 3D spatial multitasking and parallax window manager prototype (Three.js / WebXR) located at `nebula/prototype/index.html`.
- **Compositor Exploration:** Hyprland exploration is planned; intentionally not installed on the reference machine to preserve desktop stability.

---

## 5. Boot Configuration & Dual Boot

PyxisOS operates alongside Windows 10 on shared NVMe storage using `systemd-boot 261.2`:

- **PyxisOS Root Partition:** `/dev/nvme0n1p7` (UUID: `00955744-524c-403a-99f2-69818444cd30`, `ext4`)
- **PyxisOS Boot/ESP:** `/dev/nvme0n1p6` (UUID: `76B7-515F`, `vfat`, mounted at `/boot`)
- **Windows 10 EFI:** `/dev/nvme0n1p2` (UUID: `9A05-B7D2`, `vfat`)
- **Boot Configuration Files:**
  - Loader: `system/boot/loader.conf.example` (`/boot/loader/loader.conf`)
  - PyxisOS Entry: `system/boot/pyxisos.conf.example` (`/boot/loader/entries/pyxisos.conf`)
  - Windows Entry: `system/boot/windows.conf.example` (`/boot/loader/entries/windows.conf`)
  - Recovery Script: `system/boot/pyxisos_boot_commands.sh`

---

## 6. Pyxis Management CLI

PyxisOS includes a management utility installed at `/usr/local/bin/pyxis`.

### Available Commands

```text
pyxis info       Display operating system identity, hardware, and desktop info
pyxis system     Display CPU, memory metrics, uptime, and storage mounts
pyxis status     Display status of system services (SDDM, NetworkManager, Boot)
pyxis doctor     Run comprehensive diagnostic health check across 6 subsystems
pyxis version    Display current release version (0.1.0, Lunar-Pyxis, Beta)
pyxis help       Display CLI usage and help
```

### Health Diagnostics (`pyxis doctor`)

```text
============================================================
                    PYXISOS DOCTOR
============================================================

01  IDENTITY
    [ OK ] PyxisOS identity
    [ OK ] Version 0.1.0
    [ OK ] Release Lunar-Pyxis
    [ OK ] Hostname: pyxisos

02  BOOT
    [ OK ] /boot mounted
    [WARN] systemd-boot installation state
    [ OK ] PyxisOS loader entry

03  KERNEL
    [ OK ] Linux kernel
    [ OK ] Linux initramfs
    [ OK ] Intel microcode

04  SERVICES
    [ OK ] SDDM active
    [ OK ] SDDM enabled
    [ OK ] NetworkManager active

05  DESKTOP
    [ OK ] KDE Plasma available
    [WARN] Hyprland not installed

06  STORAGE
    [ OK ] Root filesystem mounted
    [ OK ] Boot filesystem mounted

------------------------------------------------------------
RESULT :: NO CRITICAL PROBLEMS DETECTED
------------------------------------------------------------
```

### Installing the CLI

To install or update `/usr/local/bin/pyxis` from source:

```bash
sudo ./scripts/cli.sh
```

---

## 7. Milestones & Progress

PyxisOS transparently tracks both operational and research milestones:

| Status | Milestone Area | Details |
|---|---|---|
| `[COMPLETED]` | **Base Substrate** | Arch Linux base running on ThinkPad X1 Carbon 5th Gen |
| `[COMPLETED]` | **OS Identity** | Distribution rebranding in `/etc/os-release` and hostname `pyxisos` |
| `[COMPLETED]` | **Dual Boot** | systemd-boot 261.2 dual-booting PyxisOS and Windows 10 |
| `[COMPLETED]` | **Desktop Shell** | KDE Plasma + SDDM operational graphical environment |
| `[COMPLETED]` | **Pyxis CLI** | `/usr/local/bin/pyxis` with `info`, `system`, `status`, `doctor` |
| `[TEMPORARY]` | **Track A Substrate** | Current Arch Linux package base and desktop environment |
| `[PROTOTYPE]` | **Spatial Multitasking** | Nebula 3D spatial desktop prototype (`nebula/prototype/`) |
| `[PROTOTYPE]` | **Terminal Banner** | Pyxis slant banner prototype (`system/branding/pyxis-banner`) |
| `[IN DEVELOPMENT]` | **Consensus Engine** | Multi-agent reasoning framework (`consensus/`) |
| `[IN DEVELOPMENT]` | **Lunar Core** | Custom `#![no_std]` Rust microkernel stub (`native/lunar-core/`) |
| `[IN DEVELOPMENT]` | **Research Paper** | Systems architecture academic paper outline (`docs/research-paper/`) |
| `[PLANNED]` | **Aegis Hypervisor** | Type-1 microkernel hypervisor and sandboxing layer |
| `[PLANNED]` | **Celestial Automation** | Autonomous task graph scheduling and workflow engine |
| `[PAUSED]` | **TTY Login Banner** | getty integration paused to maintain standard console login |

See the detailed milestone documentation:
- [Completed Milestones](docs/milestones/completed.md)
- [Temporary Substrate Details](docs/milestones/temporary.md)
- [Current System State](docs/milestones/current-state.md)
- [Full Project Roadmap](docs/milestones/roadmap.md)

---

## 8. Repository Structure

```text
PyxisOS/
├── README.md                          # Project documentation and architectural overview
├── LICENSE                            # Apache License 2.0
├── CONTRIBUTING.md                   # Development workflow and contribution guidelines
├── CODE_OF_CONDUCT.md                 # Contributor covenant code of conduct
├── SECURITY.md                        # Security vulnerability reporting policy
├── CHANGELOG.md                       # Release notes and history
├── VERSION                            # Active release version string (0.1.0)
│
├── cli/
│   └── pyxis                          # Standalone PyxisOS management CLI tool
│
├── scripts/
│   ├── cli.sh                         # CLI installer targeting /usr/local/bin/pyxis
│   └── verify.sh                      # Repository verification and syntax validation suite
│
├── system/
│   ├── boot/
│   │   ├── pyxisos_boot_commands.sh  # Boot recovery & NVMe diagnostic reference
│   │   ├── loader.conf.example       # systemd-boot loader configuration template
│   │   ├── pyxisos.conf.example      # PyxisOS loader entry template
│   │   └── windows.conf.example      # Windows 10 chainloader entry template
│   ├── branding/
│   │   ├── os-release                # Reference /etc/os-release branding template
│   │   └── pyxis-banner              # Terminal banner prototype
│   └── services/
│       └── README.md                  # Systemd service audit (SDDM, NetworkManager)
│
├── desktop/
│   └── kde/
│       └── README.md                  # Current KDE Plasma + SDDM desktop configuration
│
├── docs/
│   ├── PRD.md                        # Comprehensive Product Requirements Document
│   ├── architecture/                  # System, boot, memory, and scheduling architectures
│   ├── installation/                  # Hardware reference and dual-boot setup guides
│   ├── milestones/                    # Completed, temporary, current-state, and roadmap docs
│   ├── onboarding/                    # Developer workstation setup guide
│   └── research-paper/                # Academic paper outline
│
├── native/                            # Track B Rust workspace
│   ├── Cargo.toml
│   ├── rust-toolchain.toml
│   ├── aegis/                         # Type-1 hypervisor / security layer stub
│   └── lunar-core/                    # Custom Rust microkernel entrypoint & target spec
│
├── consensus/                         # Astral Consensus Engine (TypeScript multi-agent system)
│   ├── package.json
│   └── src/
│
├── nebula/                            # 3D spatial multitasking & parallax prototype
│   ├── README.md
│   └── prototype/
│       └── index.html
│
├── research/                          # Consensus evaluation benchmarks and comparative results
│   ├── eval/
│   └── results/
│
└── tests/
    └── cli_test.sh                    # Automated test suite for Pyxis CLI
```

---

## 9. Verification & Testing

PyxisOS includes automated verification tooling to validate repository integrity, shell script syntax, and CLI functionality before commits:

```bash
# Run repository pre-flight checks
./scripts/verify.sh

# Run CLI automated tests
./tests/cli_test.sh
```

---

## 10. Known Limitations

1. **Host Base Substrate:** PyxisOS currently relies on an Arch Linux base substrate (Track A); it is not yet a standalone independently bootstrapped distribution.
2. **Microkernel Maturity:** Lunar Core (`native/lunar-core`) is in an early scaffolding stage and boots in emulator environments, not on bare-metal ThinkPad hardware.
3. **Spatial Desktop Shell:** The 3D spatial multitasking interface exists as a browser-based prototype (`nebula/prototype/index.html`) rather than a native Wayland compositor.
4. **systemd-boot Status Warning:** `pyxis doctor` reports a non-critical installation state warning from `bootctl status` caused by dual EFI partition layout, though boot execution functions normally.

---

## 11. Contributing & Conventions

- **Branching:** Use descriptive prefixes: `feat/`, `fix/`, `docs/`, `refactor/`.
- **Commits:** Follow [Conventional Commits](https://www.conventionalcommits.org/) format.
- **Code Style:** Maintain clean, readable code with minimal unnecessary comments.
- **Guidelines:** Review [`CONTRIBUTING.md`](CONTRIBUTING.md) and [`SECURITY.md`](SECURITY.md) before submitting pull requests.

---

## 12. License

PyxisOS is open-source software licensed under the [Apache License 2.0](LICENSE).
