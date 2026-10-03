# PyxisOS — Experimental AI-Oriented Operating System

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
*An experimental AI-oriented operating-system project for physical x86_64 hardware, combining a Linux-based operational foundation with an evolving native systems architecture for multi-agent reasoning, capability isolation, and autonomous system orchestration.*

[![Version](https://img.shields.io/badge/version-0.1.0-38%3B2%3B220%3B40%3B80.svg)](VERSION)
[![Stage](https://img.shields.io/badge/stage-Beta%20%2F%20Experimental-crimson.svg)](docs/PROJECT_STATUS.md)
[![Target](https://img.shields.io/badge/target-x86__64%20physical%20hardware-lightgrey.svg)](docs/installation/hardware-setup.md)
[![Substrate](https://img.shields.io/badge/substrate-Linux%20Operational%20Foundation-blue.svg)](docs/UPSTREAM_LINUX.md)
[![License](https://img.shields.io/badge/license-Apache--2.0-black.svg)](LICENSE)
[![Vision](https://img.shields.io/badge/vision-Project%20Vision-purple.svg)](docs/PROJECT_VISION.md)
[![Roadmap](https://img.shields.io/badge/roadmap-Phased%20Milestones-green.svg)](docs/ROADMAP.md)

</div>

---

## 1. What is PyxisOS?

**PyxisOS** is an experimental AI-oriented operating-system project for physical x86_64 hardware. The current implementation uses a **Linux-based operational foundation** while developing a longer-term **native systems architecture** for multi-agent reasoning, agentic automation, system-level orchestration, isolation, and AI-oriented operating-system primitives.

> **Architectural Premise:** The current Linux foundation is an operational substrate for PyxisOS development, not the final architectural destination.

PyxisOS operates on a disciplined two-track architecture:

```text
PyxisOS
│
├── Track A — Operational Foundation
│   ├── Linux kernel substrate (drivers, ACPI, NVMe, Wi-Fi)
│   ├── System infrastructure & bootloader (systemd-boot)
│   ├── Pyxis system tooling & health diagnostics (CLI)
│   ├── Desktop environment (KDE Plasma / Wayland)
│   └── Physical x86_64 hardware (Lenovo ThinkPad X1 Carbon)
│
└── Track B — Native AI-OS Architecture
    ├── Freestanding kernel core (arch/, kernel/, mm/, fs/, drivers/)
    ├── Lunar Core (Rust #![no_std] microkernel primitives)
    ├── Astral Consensus (multi-agent reasoning & arbitration)
    ├── Aegis (planned capability isolation & hypervisor boundary)
    ├── Celestial Automation (planned system DAG orchestration)
    └── Nebula (spatial interface research)
```

By maintaining this dual-track strategy, PyxisOS ensures continuous usability and physical hardware validation today without compromising the ambitious, long-term engineering of a sovereign AI-native operating system.

---

## 2. Current Project Status

```text
Version: 0.1.0
Stage: Beta / Experimental
Development: Active
Target: x86_64 physical hardware
```

| Component | Status | Implementation | Primary Location |
|---|---|---|---|
| Physical x86_64 environment | **Working** | Bare-metal x86_64 | Lenovo ThinkPad X1 Carbon 5th Gen |
| PyxisOS identity | **Working** | Shell / Metadata | `system/branding/` |
| System management CLI | **Working** | POSIX Bash / Shell | `cli/pyxis` |
| Diagnostics | **Working** | POSIX Bash / Shell | `cli/pyxis`, `tests/` |
| Desktop environment | **Working** | Wayland / KDE Plasma | `desktop/kde/` |
| Linux operational substrate | **Working** | Linux 6.x / Upstream | Track A Foundation |
| Astral Consensus | **In development** | Native Rust | `native/consensus/` |
| Lunar Core | **Experimental** | Rust `#![no_std]` | `native/lunar-core/` |
| Nebula | **Prototype** | Research & Spec | `research/nebula/` |
| Aegis | **Planned** | Rust / Capability spec | `native/aegis/` |
| Celestial Automation | **Planned** | DAG Orchestration spec | `docs/PROJECT_VISION.md` |
| Native AI-oriented OS | **Long-term research goal** | Integrated Architecture | `docs/ROADMAP.md` |

For the canonical, comprehensive subsystem breakdown, see [`docs/PROJECT_STATUS.md`](docs/PROJECT_STATUS.md).

---

## 3. Architecture & Subsystems

```text
┌────────────────────────────────────────────────────────────────────────┐
│                        PyxisOS Shell Layer                             │
│                                                                        │
│   Stellar Canvas (Desktop UX)        Astral Consensus Engine           │
│   • KDE Plasma 6 (Track A)           • Specialized Reasoning Agents    │
│   • Nebula Spatial (Research)        • Decision Arbitration Layer      │
├────────────────────────────────────────────────────────────────────────┤
│                       Automation & Management                          │
│                                                                        │
│   Celestial Automation Engine        Pyxis Management Suite            │
│   • Task Graph (DAG) Execution       • Pyxis CLI (/usr/local/bin/pyxis)│
│   • Cryptographic Approval Gates     • Pyxis Doctor Diagnostics        │
├────────────────────────────────────────────────────────────────────────┤
│                       Runtime Host Substrate                           │
│                                                                        │
│   Track A (Operational Foundation):                                    │
│   • Linux Operational Kernel Substrate                                 │
│   • systemd-boot 261.2 + Windows 10 Dual Boot                          │
│   • Wayland / SDDM / NetworkManager                                    │
│                                                                        │
│   Track B (Native Low-Level Architecture - In Development):            │
│   • Freestanding x86_64 kernel (arch/, kernel/, mm/, fs/, drivers/)    │
│   • Lunar Core (Rust #![no_std] microkernel scheduler & IPC)           │
│   • Aegis (Capability Isolation & Security Sandbox - Planned)          │
├────────────────────────────────────────────────────────────────────────┤
│                          Physical Hardware                             │
│   Lenovo ThinkPad X1 Carbon 5th Gen (Intel i7-7600U, 16GB RAM, NVMe)   │
└────────────────────────────────────────────────────────────────────────┘
```

Detailed architectural specifications:
- [Long-Term Project Vision](docs/PROJECT_VISION.md)
- [Phased Milestone Roadmap](docs/ROADMAP.md)
- [Canonical Project Status](docs/PROJECT_STATUS.md)
- [Upstream Linux Provenance](docs/UPSTREAM_LINUX.md)
- [Subsystem Architecture Overview](docs/architecture/system-overview.md)
- [Memory Subsystem](docs/architecture/memory-subsystem.md)
- [Boot Sequence](docs/architecture/boot-sequence.md)

---

## 4. Track A — Operational Environment

Track A provides an immediately functional operating system on real x86_64 hardware, supporting everyday development, multi-agent evaluation, and hardware telemetry:

- **Desktop Shell:** KDE Plasma 6 on Wayland via SDDM.
- **Boot Management:** UEFI dual-booting alongside Windows 10 via `systemd-boot 261.2`.
- **System Management CLI:** `/usr/local/bin/pyxis` provides hardware inspection, service monitoring, and system diagnostics.
- **Hardware Integration:** Full support for Intel Core i7-7600U, NVMe SSD storage, Wi-Fi networking, Intel HD Graphics, and ThinkPad power/thermal management.

### Pyxis Management CLI

```text
pyxis info       Display OS identity, hardware specification, and desktop status
pyxis system     Display CPU load, memory utilization, uptime, and storage mounts
pyxis status     Display status of system services (SDDM, NetworkManager, Boot)
pyxis doctor     Run comprehensive diagnostic health check across 6 subsystems
pyxis version    Display release version (0.1.0, Lunar-Pyxis, Beta)
pyxis help       Display CLI usage syntax and parameters
```

---

## 5. Track B — Native Systems Architecture

Track B is developing the native, low-level foundation intended to eventually succeed the Linux operational substrate:

1. **Freestanding x86_64 Kernel Core:**
   - Multi-language architecture implemented in freestanding C, Rust, and x86_64 Assembly.
   - 32-bit Multiboot bootstrap transitioning into 64-bit Long Mode (`arch/x86_64/boot/`).
   - 64-bit GDT, TSS, IDT with 48 assembly ISR stubs and 8259 PIC remapping (`arch/x86_64/cpu/`, `arch/x86_64/interrupts/`).
   - Physical Memory Manager with bitmap frame allocation (`mm/pmm.c`).
   - Virtual Memory Manager with 4-level PML4 paging (`mm/vmm.c`).
   - Dynamic Kernel Heap allocator with boundary-tag tracking (`mm/heap.c`).
   - Hardware drivers: 16550 UART serial (`drivers/char/`), VGA text mode (`drivers/video/`), 8254 PIT (`drivers/timer/`), PS/2 keyboard (`drivers/input/`).
   - In-memory Virtual Filesystem Switch and RamFS (`fs/vfs.c`, `fs/ramfs.c`).
   - Built-in kernel shell for early interactive console control (`userspace/shell/`).

2. **Lunar Core (`native/lunar-core/`):**
   - Experimental Rust `#![no_std]` microkernel crate.
   - Implements safe task representations (`Task`), round-robin scheduler, and typed IPC message channels.

3. **Astral Consensus (`native/consensus/`):**
   - Native Rust multi-agent reasoning and decision arbitration engine.
   - Provides structured agent proposals, evidence logging, confidence scoring, and conflict arbitration for operating-system actions.

4. **Aegis (`native/aegis/`):**
   - Planned capability-based security layer and execution sandbox, designed to enforce strict isolation between autonomous agent workloads.

---

## 6. Repository Layout

```text
PyxisOS/
├── Makefile                           # Master build orchestrator (build, check, test, clean)
├── linker.ld                          # Kernel linker script (entry: _start at 1MB)
├── LICENSE                            # Apache License 2.0
├── VERSION                            # Active release version string (0.1.0)
├── CHANGELOG.md                       # Release notes and history
├── CONTRIBUTING.md                   # Development workflow and contribution guidelines
├── CODE_OF_CONDUCT.md                 # Contributor covenant code of conduct
├── SECURITY.md                        # Security vulnerability reporting policy
│
├── arch/                              # Architecture-specific kernel implementation
│   └── x86_64/                        # Multiboot bootstrap, GDT, IDT, ISRs, context switch
├── boot/                              # Multiboot headers and boot parameter definitions
├── drivers/                           # Device drivers (UART serial, VGA, PIT timer, PS/2)
├── fs/                                # Virtual Filesystem (VFS) and RamFS implementation
├── include/                           # Kernel internal headers (pyxis/) and UAPI (syscalls)
├── kernel/                            # Freestanding C kernel core and Rust scheduler/IPC bindings
├── mm/                                # Physical frame, PML4 virtual paging, and heap allocators
│
├── cli/                               # Pyxis management CLI (/usr/local/bin/pyxis)
├── system/                            # Boot configuration templates and OS branding
├── userspace/                         # Userspace programs and interactive kernel shell
│
├── native/                            # Native systems Rust workspace
│   ├── Cargo.toml                     # Rust workspace configuration
│   ├── aegis/                         # Capability isolation & security layer (Planned)
│   ├── consensus/                     # Astral Consensus multi-agent engine (In development)
│   └── lunar-core/                    # Lunar Core Rust #![no_std] microkernel (Experimental)
│
├── research/                          # Systems and interface research
│   ├── consensus/                     # Consensus engine evaluation benchmarks
│   ├── eval/                          # Quantitative reasoning evaluation harness
│   ├── nebula/                        # Spatial computing & window management research
│   └── results/                       # Empirical baseline vs. consensus results
│
├── docs/                              # Project documentation
│   ├── PROJECT_VISION.md              # Long-term systems and AI-OS technical vision
│   ├── ROADMAP.md                     # Five-phase milestone roadmap
│   ├── PROJECT_STATUS.md              # Canonical subsystem readiness matrix
│   ├── UPSTREAM_LINUX.md              # Upstream Linux provenance & licensing boundaries
│   ├── architecture/                  # Subsystem deep dives (boot, memory, scheduling)
│   ├── installation/                  # Hardware reference and dual-boot setup guides
│   ├── milestones/                    # Detailed historical milestone records
│   └── onboarding/                    # Developer workstation environment setup
│
├── scripts/                           # Build, run, test, toolchain, and verification scripts
└── tests/                             # Automated test suites
```

---

## 7. Verification & Build Tooling

PyxisOS provides verification and build automation across its multi-language codebase:

```bash
# Verify repository integrity, paths, and shell syntax
./scripts/verify.sh

# Run CLI automated tests
./tests/cli_test.sh

# Build native freestanding kernel components
make build

# Validate native Rust workspace
cd native && cargo check --workspace
```

---

## 8. Known Limitations & Technical Realities

PyxisOS maintains rigorous technical honesty regarding current implementation maturity:

1. **Linux Operational Substrate:** PyxisOS currently relies on a Linux foundation for hardware execution on physical machines (Track A); it is not yet an independent, self-hosting microkernel distribution.
2. **Lunar Core Maturity:** Lunar Core (`native/lunar-core`) is an experimental Rust `#![no_std]` prototype; it compiles and validates under target specifications, but full bare-metal replacement of the operational substrate is an active research goal.
3. **Aegis Status:** Aegis is an architecturally planned capability isolation model, not a completed Type-1 hypervisor.
4. **Nebula Research Boundary:** Nebula is exploratory spatial interface research; operational desktop multitasking on physical hardware is handled via KDE Plasma and Wayland.

---

## 9. Licensing & Provenance Boundaries

- **PyxisOS Original Code:** Licensed under the [Apache License 2.0](LICENSE). Copyright 2026 The PyxisOS Authors.
- **Upstream Linux Substrate:** Subject to upstream [GNU General Public License v2.0 only (`GPL-2.0-only`)](https://github.com/torvalds/linux).
- For complete details on upstream attribution, licensing separation, and codebase accounting, see [`docs/UPSTREAM_LINUX.md`](docs/UPSTREAM_LINUX.md).
