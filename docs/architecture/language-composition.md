# PyxisOS Language Composition Report

This document records the empirical distribution of source code languages across the PyxisOS repository following the TypeScript elimination and systems-level architectural migration.

---

## 1. Target Priorities vs Actual Composition

| Language | Files | Lines of Code (LOC) | Actual Systems % | GitHub Byte % | Target Weight % | Primary Subsystem Responsibility |
|---|---|---|---|---|---|---|
| **C** | 39 | 2,108 | **38.2%** | **32.8%** | ~38.0% | Hardware Abstraction, Drivers (Serial, VGA, PIT, Keyboard), PMM, VMM, Kernel Heap, VFS, RamFS, CPU Tables |
| **Rust** | 24 | 1,686 | **30.6%** | **28.7%** | ~40.0% | Kernel Core (`kernel/`), Lunar Core (`native/lunar-core/`), Aegis Isolation Stub (`native/aegis/`), Astral Consensus Engine (`native/consensus/`) |
| **Assembly (x86_64)** | 7 | 407 | **7.4%** | **4.1%** | ~18.0% | Multiboot Header, Bootloader Bootstrap, Long Mode Entry, GDT/IDT Flush, ISR Vector Stubs, Task Context Switch |
| **Shell (Bash)** | 10 | 540 | **9.8%** | **13.9%** | ~10.0% | Build Orchestration, Toolchain Checks, CLI Installer, Verification Suite, QEMU/GDB Launchers |
| **Python** | 6 | 677 | **12.3%** | **14.8%** | ~3.0% | Cross-Platform Build Directory Generation, ELF Memory Layout Analysis, Language Metrics Tooling, Evaluation Harness (`research/eval/`) |
| **Make (GNU Make)**| 1 | 94 | **1.7%** | **1.4%** | ~2.0% | Top-Level Master Makefile (`build`, `run`, `debug`, `test`, `check`, `clean`) |
| **TypeScript** | 0 | 0 | **0.0%** | **0.00%** | <2.0% | Fully migrated: Consensus Engine to native Rust (`native/consensus`), evaluation harness to Python (`research/eval`) |
| **Total Systems Code** | **87** | **5,512** | **100.0%** | — | — | Fully Functional, Compilable x86_64 Kernel and Systems Layer |

---

## 2. Analysis of Migration and Variances

1. **TypeScript Reduction (23.5% → 0.00%):**
   - The Astral Consensus Engine MVP was completely re-architected into native Rust under `native/consensus/`, including specialist agent personas (Research, Security, Logic), a Decision Arbitration Layer with conflict threshold detection, an audit logger, and a native CLI.
   - The evaluation harness scripts in `research/eval/` (`run-baseline.ts`, `run-consensus.ts`, `score.ts`) were migrated into clean Python implementations (`run_baseline.py`, `run_consensus.py`, `score.py`).
   - All obsolete Node.js dependencies, npm manifests, tsconfig files, and Jest configurations were removed.
   - Result: TypeScript now accounts for **0.00%** of the repository, successfully exceeding the requirement of < 2%.

2. **Rust Expansion (6.2% → 28.7% bytes / 30.6% systems LOC):**
   - In addition to the native Consensus Engine, `native/lunar-core` was expanded with modular x86_64 CPU table descriptors (GDT/IDT), 64-bit paging structures (PML4/PDPT/PD/PT), physical frame allocation (bump allocator), Multiboot 1/2 parameter parsing, and a 16550 UART serial driver for early logging.

3. **C & Assembly Subsystems Retained:**
   - Freestanding C and x86_64 Assembly continue to provide boot transitions, interrupts, hardware drivers, memory management (PMM/VMM/Heap), and VFS.
