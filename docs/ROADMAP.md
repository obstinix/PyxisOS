# PyxisOS Development Roadmap

This document outlines the multi-phase engineering and research roadmap for PyxisOS, tracking the evolution from an operational Linux substrate to a fully native, AI-oriented operating system.

---

## Roadmap Overview

```text
Phase 1: Operational Foundation          [Substantially Implemented]
  ├── Physical x86_64 boot & dual-boot UEFI
  ├── System identity & branding
  ├── System tooling (CLI & diagnostics)
  └── Operational desktop environment (KDE/Wayland)
       │
       ▼
Phase 2: Native Systems Research         [Active Development]
  ├── Freestanding C/Assembly bootstrap & kernel core
  ├── Lunar Core (Rust #![no_std] microkernel primitives)
  ├── 4-level PML4 paging & frame allocators
  └── QEMU emulation & bare-metal validation
       │
       ▼
Phase 3: Agentic OS Architecture         [Research / In Development]
  ├── Astral Consensus arbitration engine
  ├── Multi-agent coordination & lifecycle management
  ├── Capability tokens & permission validation
  └── Kernel-level event streaming
       │
       ▼
Phase 4: Isolation & Automation          [Planned]
  ├── Aegis isolation boundaries & hardware virtualization
  ├── Sandboxed agent runtimes
  └── Celestial Automation DAG orchestration & approval gates
       │
       ▼
Phase 5: Native AI-Oriented OS           [Long-Term Research Goal]
  ├── Sovereign microkernel host environment
  ├── Native AI runtime integration
  └── System-level agent participation
```

---

## Phase 1 — Operational Foundation
* **Status:** Substantially Implemented
* **Primary Target:** Lenovo ThinkPad X1 Carbon 5th Gen (Intel Core i7-7600U, 16GB RAM)

Phase 1 establishes a rock-solid, production-grade operating environment on real physical x86_64 hardware. By leveraging an established Linux-based foundation, PyxisOS can be used directly for daily development, hardware profiling, driver validation, and user interface experiments without waiting for decades of kernel driver development.

### Key Milestones:
- [x] **Physical x86_64 Boot:** UEFI boot configuration via `systemd-boot`, supporting clean dual-boot alongside existing partitions.
- [x] **System Identity:** Operating-system release metadata (`/etc/os-release`, Pyxis branding banners, version specifications).
- [x] **System Tooling:** Production Bash/POSIX management tool (`cli/pyxis`) providing system status, diagnostics, and environment monitoring.
- [x] **Diagnostics & Telemetry:** Hardware, thermal, battery, and memory telemetry collectors for physical validation.
- [x] **Desktop Environment:** Operational Wayland session running KDE Plasma 6 for windowed development and multitasking.
- [x] **Boot Infrastructure:** Boot loader configuration entries, boot parameter tuning, and kernel command-line profiles.
- [x] **Testing & Verification:** Comprehensive test scripts (`tests/cli_test.sh`, `scripts/verify.sh`).
- [x] **Repository Structure:** Clean multi-language layout separating operational tooling, native kernel source, and research modules.

---

## Phase 2 — Native Systems Research
* **Status:** Active Development
* **Primary Target:** Freestanding x86_64 Architecture (`native/lunar-core/` and `arch/`, `kernel/`, `mm/`, `fs/`, `drivers/`)

Phase 2 develops independent, low-level operating system primitives to move beyond reliance on the Linux kernel for core operating-system abstractions.

### Key Milestones:
- [x] **Freestanding Multiboot Bootstrap:** 32-bit Multiboot headers (`header.S`) and transition to 64-bit Long Mode (`boot.S`).
- [x] **GDT & IDT Implementation:** 64-bit Global Descriptor Table with TSS and full 256-vector Interrupt Descriptor Table with register-save ISR stubs.
- [x] **Memory Management Subsystem:**
  - Physical Memory Manager: Deterministic bitmap page frame allocator (`mm/pmm.c`).
  - Virtual Memory Manager: 4-level PML4 paging infrastructure with 4KB pages and 2MB huge-page mapping (`mm/vmm.c`).
  - Kernel Heap: Dynamic chunk allocator with boundary tags (`mm/heap.c`).
- [x] **Device Abstraction:** 16550 UART serial driver, VGA 80x25 text-mode console, 8254 PIT timer (100 Hz), PS/2 keyboard driver.
- [x] **VFS & RamFS:** In-memory virtual filesystem switch with directory traversal.
- [x] **Lunar Core (`native/lunar-core/`):** Rust `#![no_std]` microkernel architecture with safe task control blocks, round-robin scheduler, synchronization primitives, and typed IPC message channels.
- [ ] **Cross-Language FFI Binding:** Comprehensive linking and call-graph integration between C kernel drivers and Rust scheduler modules.
- [ ] **Bare-Metal Boot on Hardware:** Booting the compiled ELF image (`bin/pyxis-kernel.elf`) via GRUB/multiboot directly on test machine hardware.

---

## Phase 3 — Agentic OS Architecture
* **Status:** Research / In Development
* **Primary Target:** Astral Consensus (`native/consensus/`)

Phase 3 introduces native mechanisms for multi-agent reasoning, coordination, and deterministic arbitration directly into operating-system decision flows.

### Key Milestones:
- [x] **Astral Consensus Core:** Rust implementation of structured agent proposals, evidence logging, and arbitration algorithms (`native/consensus/`).
- [ ] **Specialized Agent Roles:** Dedicated system agents for Security Auditing, Performance Profiling, Memory Optimization, and Network Management.
- [ ] **Agent Lifecycle Management:** Daemonization, heartbeat tracking, supervised crash recovery, and health attestation.
- [ ] **Capability & Permission Model:** Cryptographically signed capability tokens restricting which system APIs each agent may invoke.
- [ ] **System Event Bus:** High-throughput kernel event streaming feeding hardware and system events directly into agent perception pipelines.
- [ ] **Conflict Arbitration:** Deterministic multi-criteria decision algorithms to resolve conflicting actions from disparate agents.

---

## Phase 4 — Isolation and Automation
* **Status:** Planned
* **Primary Target:** Aegis (`native/aegis/`) and Celestial Automation

Phase 4 hardens the execution environment, preventing autonomous agent faults from compromising system stability or data integrity.

### Key Milestones:
- [ ] **Aegis Capability Isolation:** Fine-grained capability checks guarding file, memory, and hardware access.
- [ ] **Spatial Sandboxing:** Strict CR3 address-space isolation, W^X page permissions, and system call filtering for agent runtimes.
- [ ] **Hardware-Assisted Virtualization:** Evaluating Intel VT-x micro-VM containers to isolate autonomous agent code from supervisor memory.
- [ ] **Celestial Automation Task Engine:** Directed Acyclic Graph (DAG) workflow orchestrator for system maintenance and automation tasks.
- [ ] **Approval Gates:** Mandatory cryptographic and human authorization gates for destructive or privileged operations.
- [ ] **Transactional Operations:** Automated rollback mechanisms for system configuration mutations.

---

## Phase 5 — Native AI-Oriented OS
* **Status:** Long-Term Research Goal
* **Primary Target:** Fully Autonomous, Self-Hosting PyxisOS

> **Important:** Phase 5 is an ambitious long-term research objective. It is **not** currently implemented.

Phase 5 represents the ultimate destination of the PyxisOS project: an operating system where AI models and autonomous agents are not merely software running on top of an OS, but **first-class system participants** integrated into the kernel's scheduler, security boundaries, and storage architectures.

### Research Goals:
- Native execution of neural network inference within low-latency supervisor or hypervisor contexts.
- Continuous real-time optimization of kernel scheduling and cache allocation driven by local agent models.
- Transparent, self-healing operating system services that detect anomalies, propose verified patches, and maintain system health autonomously.
