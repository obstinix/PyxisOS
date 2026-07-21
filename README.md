# PyxisOS

![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)
![Status](https://img.shields.io/badge/status-early--stage-orange.svg)
![Target](https://img.shields.io/badge/target-x86__64-lightgrey.svg)

**An operating system built from scratch — bootloader to kernel to desktop — as a deliberate deep-systems learning project.**

PyxisOS is not built on top of an existing kernel or hypervisor. Every layer is written by hand: a NASM bootloader, a Rust kernel, and everything above it, for x86_64, starting from BIOS/legacy boot.

---

## Why build from scratch?

Two paths were on the table for this project:

- **Track A** — an AI-orchestration layer on top of an existing Linux/KVM stack. Faster to ship, but most of the actual OS internals stay hidden behind someone else's implementation.
- **Track B** — a kernel and hypervisor built entirely from scratch. Slower, harder, and the one this project commits to, because the point isn't the fastest path to a working demo — it's actually understanding every layer of the machine.

**PyxisOS follows Track B.** Depth over speed, deliberately.

---

## Current Status

🚧 **Early stage.** This is a curriculum-driven build, not a race to a 1.0.


|
 Milestone 
|
 Status 
|
|
---
|
---
|
|
 Phase 1 — Fundamentals 
|
 ✅ Complete 
|
|
 Phase 2 — Bootloader (Lesson 1: MBR bootloader in NASM) 
|
 ✅ Complete 
|
|
 Phase 2 — Bootloader (remaining lessons) 
|
 🔜 In progress 
|
|
 Kernel core 
|
 ⬜ Not started 
|
|
 Memory management 
|
 ⬜ Not started 
|

---

## Roadmap

The full curriculum is broken into detailed phases; the major milestones it builds toward are:

Bootloader → Kernel Core → Memory Management → Drivers/IO
→ Desktop Environment → AI Integration → Distributed Systems
→ Compiler Design


Each milestone is treated as a first-class learning phase — implemented, tested in an emulator, and documented — before moving to the next, rather than stubbing pieces out to reach a demo faster.

---

## Architecture & Toolchain

┌─────────────────────────────────────────┐
│ PyxisOS (x86_64) │
├─────────────────────────────────────────┤
│ Bootloader NASM (BIOS/legacy) │
│ Kernel Rust │
│ Emulation QEMU │
│ Debugging GDB (-s -S) │
└─────────────────────────────────────────┘


| Component | Choice |
|---|---|
| Target architecture | x86_64 |
| Bootloader | NASM, BIOS/legacy boot (initial phase) |
| Kernel language | Rust |
| Emulator | QEMU |
| Debugger | GDB — launched with `-s -S`; `set architecture i8086` for real-mode bootloader debugging |
| Build | Make (`run`, `debug`, `clean` targets) |
| Editor | VS Code (rust-analyzer, hex editor, NASM syntax highlighting) |
| Dev environment | Linux / WSL2 |

---

## Getting Started

**Prerequisites:** NASM, QEMU, GDB, a Rust toolchain, Make.

```bash
make run      # assemble/build and boot in QEMU
make debug    # boot with a GDB stub attached (-s -S)
make clean    # clean build artifacts
```

---

## Repository Conventions

- **Branches:** `track-b/<description>`
- **Commits:** [Conventional Commits](https://www.conventionalcommits.org/) (`feat:`, `fix:`, `docs:`, …)
- **Documentation:** architecture notes and design decisions live in `docs/architecture/`

---

## Philosophy

This project is built methodically, one fully-understood layer at a time. No component is treated as a black box just to move faster — if that means the timeline is measured in phases rather than weeks, that's the deliberate tradeoff being made.

---

## License

Apache License 2.0 — see [`LICENSE`](./LICENSE).
