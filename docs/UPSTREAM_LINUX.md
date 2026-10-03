# Upstream Linux Kernel Provenance & Licensing Boundaries

This document defines the provenance, attribution, licensing boundaries, and architectural relationship between the upstream Linux kernel and the PyxisOS project.

---

## 1. Upstream Kernel Provenance

| Property | Details |
|---|---|
| **Upstream Project** | Linux Kernel |
| **Upstream Repository** | [https://github.com/torvalds/linux](https://github.com/torvalds/linux) |
| **Reference Architecture Tree** | [https://github.com/obstinix/linux-kernelsrctree](https://github.com/obstinix/linux-kernelsrctree) |
| **Operational Substrate Version** | Linux 6.12 LTS series (x86_64) |
| **Target Hardware Platform** | Physical x86_64 (Lenovo ThinkPad X1 Carbon 5th Gen / Intel Core i7-7600U) |
| **Upstream License** | GNU General Public License v2.0 only (`GPL-2.0-only`) |
| **Upstream Copyright & Attribution** | Linus Torvalds and Linux kernel contributors |

---

## 2. Mandatory Provenance Statement

> The Linux kernel source contained in this repository originates from the upstream Linux kernel project and remains subject to its applicable license and attribution requirements. It is not PyxisOS-original code.

---

## 3. Relationship to PyxisOS Architecture

PyxisOS is an experimental AI-oriented operating-system project operating on a disciplined dual-track model:

```text
PyxisOS System Architecture
│
├── Track A — Operational Foundation
│   ├── Upstream Linux kernel substrate (hardware drivers, ACPI, NVMe, Wi-Fi)
│   ├── System infrastructure & bootloader (systemd-boot)
│   ├── PyxisOS operational tooling (CLI, diagnostics, service hooks)
│   ├── Graphical environment (Wayland / KDE Plasma operational shell)
│   └── Physical x86_64 deployment machine
│
└── Track B — Native AI-OS Architecture
    ├── Freestanding x86_64 kernel core (arch/, kernel/, mm/, fs/, drivers/)
    ├── Lunar Core (experimental Rust #![no_std] microkernel primitives)
    ├── Astral Consensus (multi-agent reasoning and decision arbitration)
    ├── Aegis (planned capability isolation and hypervisor boundary)
    └── Celestial Automation (planned system-level orchestration)
```

The current Linux foundation is an **operational substrate** for PyxisOS development, not the final architectural destination. It provides the reliable hardware, driver, storage, and userspace environment required to operate the physical development machine while native components are incrementally researched, implemented, and verified.

---

## 4. Strict License Separation

PyxisOS maintains strict legal and license separation between original project code and upstream Linux kernel material:

```text
PyxisOS-Original Code
        │
        ▼
Apache License 2.0 (Copyright 2026 The PyxisOS Authors)
• Lunar Core (`native/lunar-core/`)
• Astral Consensus (`native/consensus/`)
• Aegis (`native/aegis/`)
• Freestanding kernel implementation (`arch/`, `kernel/`, `mm/`, `fs/`, `drivers/`, `include/`)
• Pyxis CLI and system tooling (`cli/`, `system/`)
• Userspace interfaces (`userspace/`)

Upstream Linux Kernel & Reference Code
        │
        ▼
GPL-2.0-only / Upstream Linux Licensing Terms
• Upstream Linux kernel source trees, patches, and configurations
• Upstream Linux header imports and drivers
• Upstream build infrastructure and licensing files (COPYING, LICENSES/)
```

### Governing Rules
1. **No Relabeling:** Upstream Linux code is never relabeled under Apache License 2.0.
2. **Attribution Preservation:** All upstream copyright notices, license headers, and author attributions are preserved intact.
3. **No False Ownership:** PyxisOS does not claim upstream Linux code as original PyxisOS development.

---

## 5. Codebase Accounting & Integrity

PyxisOS explicitly rejects artificial code-count inflation. Upstream kernel source is included solely to make the development environment self-contained around the operational kernel substrate and establish a clear source relationship between the running OS and its underlying hardware.

When evaluating repository metrics, code is explicitly partitioned:

1. **PyxisOS-Native Source:** Original low-level C, Rust, Assembly, and Shell implementation authored specifically for PyxisOS.
2. **Upstream Linux Source:** External operational substrate code maintained under upstream GPL-2.0 terms.
3. **Third-Party Dependencies:** Vendored or declared library dependencies (e.g., Rust crates).
4. **Research & Prototype Specifications:** Architecture specifications, benchmark suites, and roadmap documentation.

PyxisOS never reports upstream Linux lines of code as original or proprietary codebase volume.
