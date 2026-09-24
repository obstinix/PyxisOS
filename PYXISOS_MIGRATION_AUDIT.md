# PyxisOS Master Migration & Codebase Audit Report

## 1. Executive Summary

This audit assesses the state of the PyxisOS codebase prior to its systems-level architectural migration. PyxisOS is evolving from a hybrid repository (combining an Arch Linux Track A documentation layer, experimental TypeScript agents, and early Rust kernel stubs) into a unified, low-level, multi-language x86_64 operating system built from the hardware upward in C, Rust, and x86_64 Assembly, with Shell, Python, and GNU Make providing developer and build infrastructure.

---

## 2. Current Repository Structure

```text
PyxisOS/
├── .github/                       # CI workflows and issue templates
├── .gitignore                     # Git ignore patterns
├── CHANGELOG.md                   # Version release notes
├── CODE_OF_CONDUCT.md             # Contributor covenant
├── CONTRIBUTING.md               # Contribution guidelines
├── LICENSE                        # Apache License 2.0
├── README.md                      # Top-level documentation
├── SECURITY.md                    # Security vulnerability policy
├── VERSION                        # Project version string (0.1.0)
│
├── automation/                    # Celestial Automation placeholder README
├── cli/                           # PyxisOS Track A management CLI (POSIX bash)
│   └── pyxis
├── consensus/                     # Astral Consensus Engine (Node.js/TypeScript, 215KB lockfile)
│   ├── package.json
│   ├── tsconfig.json
│   └── src/
├── desktop/                       # KDE Plasma & Wayland notes
│   └── kde/README.md
├── docs/                          # Architecture, onboarding, milestone, and installation docs
│   ├── PRD.md
│   ├── architecture/
│   ├── installation/
│   ├── milestones/
│   ├── onboarding/
│   └── research-paper/
├── intelligence/                  # Orbital Intelligence placeholder README
├── native/                        # Track B Rust workspace
│   ├── Cargo.lock
│   ├── Cargo.toml
│   ├── rust-toolchain.toml
│   ├── aegis/                     # Security layer stub
│   │   ├── Cargo.toml
│   │   └── src/lib.rs
│   └── lunar-core/                # Rust microkernel stub
│       ├── .cargo/config.toml
│       ├── Cargo.toml
│       ├── x86_64-pyxis.json
│       └── src/
│           ├── arch/mod.rs
│           ├── boot/mod.rs
│           ├── logging.rs
│           ├── main.rs
│           ├── memory/mod.rs
│           └── panic.rs
├── nebula/                        # Three.js WebXR spatial desktop prototype
│   ├── README.md
│   └── prototype/index.html
├── network/                       # Constellation Network placeholder README
├── research/                      # Consensus benchmarks and evaluation rubrics
│   ├── eval/
│   └── results/
├── scripts/                       # CLI installer and verify scripts
│   ├── cli.sh
│   └── verify.sh
├── shell/                         # Stellar Canvas placeholder README
├── system/                        # Track A boot configurations and branding
│   ├── boot/
│   │   ├── pyxisos_boot_commands.sh
│   │   ├── loader.conf.example
│   │   ├── pyxisos.conf.example
│   │   └── windows.conf.example
│   ├── branding/
│   │   ├── os-release
│   │   └── pyxis-banner
│   └── services/
│       └── README.md
└── tests/                         # Track A CLI test suite
    └── cli_test.sh
```

---

## 3. Current Language Composition

An analysis of the tracked lines of code (excluding markdown documentation and JSON assets) reveals:

| Language | Primary Locations | Role / Scope |
|---|---|---|
| **TypeScript / JS** | `consensus/`, `research/eval/` | Multi-agent LLM consensus prototypes and evaluation harnesses |
| **Rust** | `native/lunar-core/`, `native/aegis/` | Bare-metal `#![no_std]` kernel stubs |
| **Shell (Bash)** | `cli/`, `scripts/`, `tests/`, `system/` | System configuration, CLI tool, installer, and verification |
| **HTML / CSS** | `nebula/prototype/` | WebXR 3D spatial multitasking demo |
| **C** | *None* | No C code currently present in the kernel tree |
| **x86_64 Assembly** | *None* | No standalone `.S` or `.asm` source files present in the kernel tree |
| **Make** | *None* | No root `Makefile` present |
| **Python** | *None* | No Python build or verification scripts present |

---

## 4. Current Architecture Assessment

1. **Two-Track Disconnect:**
   - **Track A** documents an operational Arch Linux installation on a Lenovo ThinkPad X1 Carbon 5th Gen (systemd-boot, KDE Plasma, SDDM, NetworkManager).
   - **Track B** represents an embryonic Rust microkernel (`lunar-core`) that has not yet booted beyond an empty loop stub.
2. **Missing Kernel Core Subsystems:**
   - No Physical Memory Manager (PMM) or Virtual Memory Manager (VMM).
   - No Interrupt Descriptor Table (IDT), Global Descriptor Table (GDT), or exception handlers in assembly.
   - No hardware abstraction layer or driver interfaces in C.
   - No context switching, process control blocks, or scheduler.
   - No Virtual Filesystem (VFS) interface or Inter-Process Communication (IPC) primitives.
3. **Build System Absence:**
   - There is no top-level `Makefile` to assemble, compile C/Rust, link an ELF kernel, produce a bootable image, or invoke QEMU/GDB.

---

## 5. Build Audit & Verification Failures

During the audit, building the native kernel components was tested using `cargo check` inside `native/` and `native/lunar-core/`:

### Defect 1: Cargo Workspace Profile Invalidation
- **Location:** `native/lunar-core/Cargo.toml`
- **Symptom:** `warning: profiles for the non root package will be ignored, specify profiles at the workspace root`
- **Root Cause:** Profile definitions (`[profile.dev]` and `[profile.release]`) must reside in the workspace root `native/Cargo.toml`.

### Defect 2: Target Specification Schema Error
- **Location:** `native/lunar-core/x86_64-pyxis.json`
- **Symptom:** `error loading target specification: target-pointer-width: invalid type: string "64", expected u16 at line 6 column 32`
- **Root Cause:** Modern `rustc` expects integer values for `target-pointer-width: 64` and `target-c-int-width: 32`, not string literals `"64"` and `"32"`.

### Defect 3: Missing Target-Spec Flags & Build-Std
- **Location:** `native/` cargo invocation
- **Symptom:** `error: unwinding panics are not supported without std` and `.json target specs require -Zjson-target-spec`
- **Root Cause:** Compiling a bare-metal `#![no_std]` binary requires `-Zbuild-std=core,alloc`, `panic = "abort"`, and `-Zjson-target-spec` configured at the workspace level.

---

## 6. Subsystem Breakdown & Classification

| Subsystem | Current State | Deficiencies / Debt | Target Action |
|---|---|---|---|
| **Boot Layer** | Text notes only (`system/boot/`) | No multiboot / UEFI entrypoint, no bootloader stage | **Implement** x86_64 multiboot assembly + C boot header |
| **CPU / Arch** | Stub (`native/lunar-core/src/arch/`) | Empty module, no GDT/IDT/interrupt stubs | **Implement** x86_64 Assembly for GDT, IDT, ISRs, context switch |
| **Memory** | Stub (`native/lunar-core/src/memory/`) | Empty module, no allocator or page table manipulation | **Implement** PMM (bitmap/buddy) and VMM (4-level paging) in C/Rust |
| **Drivers** | None | No serial, VGA/framebuffer, timer, keyboard, or PCI | **Implement** C HAL: UART 16550, PIT 8254, PIC 8259, Framebuffer |
| **Kernel Core** | Empty `loop {}` in `_start` | No panic handler output, no logging, no mainloop | **Implement** Multilingual kernel initialization in C & Rust |
| **Process / Scheduler** | None | No task structure, no runqueue | **Implement** Cooperative/preemptive scheduler and task PCB in Rust |
| **IPC** | None | No channels, message queues, or shared memory | **Implement** Microkernel message-passing abstraction in Rust |
| **VFS / Filesystem** | None | No file/inode structures or ramfs | **Implement** In-memory VFS (tarfs / initramfs) in Rust |
| **Userspace / Shell** | Host script (`cli/pyxis`) | Not linked to kernel; runs only in Linux host | **Maintain** host tooling, create in-kernel basic shell interface |
| **Consensus / AI** | Node.js TS (`consensus/`) | Isolated web app, no OS kernel integration | **Classify** as `research/consensus`, remove lockfile clutter |
| **Spatial Desktop** | Three.js HTML (`nebula/`) | Browser demo, no native Wayland/graphics pipeline | **Classify** as `research/nebula` prototype |

---

## 7. Migration Risks & Mitigation Strategy

1. **Risk: Cargo Culting Linux Complexity**
   - *Mitigation:* PyxisOS borrows architectural modularity from `linux-kernelsrctree` (subsystem directories, clean header separation, driver abstractions) but maintains a lightweight, readable microkernel/hybrid design. No complex Linux internals (e.g., cgroups, namespaces, eBPF) are imported.
2. **Risk: Toolchain Incompatibility across Host Environments**
   - *Mitigation:* Support standard GNU Make, GCC / Clang, NASM / GNU `as`, and Cargo. Provide a robust `scripts/toolchain.sh` and `make check` target to diagnose missing dependencies.
3. **Risk: Unsafe C/Rust FFI Boundaries**
   - *Mitigation:* Explicit `extern "C"` interfaces, strict ABI compatibility (standard integer types, explicit struct layouts `#[repr(C)]`), pointer validation, and zero unverified pointer arithmetic.

---

## 8. Target Architecture & Target Directory Blueprint

```text
PyxisOS/
├── Makefile                           # Top-level build orchestration (GNU Make)
├── Cargo.toml                         # Unified kernel Rust workspace
├── linker.ld                          # x86_64 kernel linker script
├── README.md                          # Production-grade OS documentation
├── VERSION                            # 0.1.0
│
├── arch/
│   └── x86_64/
│       ├── boot/
│       │   ├── boot.S                 # Multiboot2 entry point, 32-bit to 64-bit transition
│       │   └── header.S               # Multiboot header specification
│       ├── cpu/
│       │   ├── gdt.S / gdt.c          # Global Descriptor Table & TSS setup
│       │   ├── idt.S / idt.c          # Interrupt Descriptor Table & gate loading
│       │   └── cpu.c                  # CPUID, CR0/CR3/CR4, MSR helpers
│       ├── interrupts/
│       │   ├── isr.S                  # Interrupt Service Routine entry/exit assembly stubs
│       │   ├── pic.c                  # 8259 Legacy PIC driver / masking
│       │   └── irq.c                  # Common IRQ dispatching in C
│       ├── context/
│       │   └── switch.S               # Task context switching assembly
│       ├── syscall/
│       │   └── syscall.S              # Fast system call (syscall/sysret) entry
│       └── linker/
│           └── kernel.ld              # Architecture linker script
│
├── boot/
│   ├── multiboot.h                    # Multiboot 1 / 2 protocol header
│   └── boot_params.h                  # Kernel handover structure
│
├── kernel/
│   ├── core/
│   │   ├── main.c                     # Kernel early entry point in C
│   │   ├── kprintf.c                  # Early kernel formatted output
│   │   └── panic.c                    # Kernel panic & stack trace reporting
│   ├── src/                           # Rust kernel components
│   │   ├── lib.rs                     # Rust kernel core library
│   │   ├── scheduler.rs               # Process / thread scheduler
│   │   ├── task.rs                    # Process Control Block & states
│   │   ├── sync.rs                    # Spinlocks & synchronization primitives
│   │   └── ipc.rs                     # Microkernel message queues & channels
│   └── Cargo.toml
│
├── mm/
│   ├── pmm.c / pmm.h                  # Physical Memory Manager (bitmap frame allocator)
│   ├── vmm.c / vmm.h                  # Virtual Memory Manager (PML4, PDPT, PD, PT paging)
│   ├── heap.c / heap.h                # Kernel heap allocator (kmalloc/kfree)
│   └── mem.c                          # Bare-metal memset, memcpy, memmove
│
├── drivers/
│   ├── char/
│   │   └── serial.c / serial.h        # 16550 UART serial driver for debugging
│   ├── video/
│   │   └── vga.c / vga.h              # VGA text-mode / Linear Framebuffer console
│   ├── timer/
│   │   └── pit.c / pit.h              # 8254 Programmable Interval Timer
│   └── input/
│       └── ps2_keyboard.c             # PS/2 keyboard driver
│
├── fs/
│   ├── vfs.h / vfs.c                  # Virtual Filesystem Switch (mount, open, read, write)
│   └── ramfs.c                        # Memory-backed initramfs filesystem
│
├── include/
│   ├── pyxis/
│   │   ├── types.h                    # Fixed-width integer definitions (uint32_t, etc.)
│   │   ├── kernel.h                   # Core kernel declarations
│   │   ├── mm.h                       # Memory management declarations
│   │   ├── drivers.h                  # Device driver interfaces
│   │   └── fs.h                       # Filesystem interfaces
│   └── uapi/                          # Userspace API & syscall numbers
│
├── userspace/
│   ├── init/                          # First userspace initialization task
│   └── shell/                         # Minimal interactive kernel shell
│
├── tools/
│   └── python/
│       ├── generate_iso.py            # Automated bootable ISO / image generator
│       ├── analyze_memory.py          # ELF symbol & memory layout analyzer
│       └── language_metrics.py        # Exact repository language composition calculator
│
├── scripts/
│   ├── build.sh                       # Complete automated build pipeline
│   ├── run.sh                         # QEMU launcher
│   ├── debug.sh                       # QEMU + GDB stub launcher
│   ├── test.sh                        # Automated test runner
│   ├── toolchain.sh                   # Dependency and compiler verification
│   └── clean.sh                       # Build artifact cleanup
│
├── tests/
│   ├── unit/                          # Memory, math, and data-structure unit tests
│   ├── boot/                          # Boot validation test
│   └── qemu/                          # Headless QEMU integration tests
│
├── docs/
│   ├── architecture/                  # Full architectural specifications
│   │   ├── reference-decisions.md     # Reference analysis of linux-kernelsrctree
│   │   ├── linux-concept-map.md       # Comparative concept map (Adopt/Adapt/Inspire/Ignore)
│   │   ├── boot-flow.md               # Hardware boot flow diagram & documentation
│   │   ├── language-boundaries.md     # C, Rust, ASM, and FFI rules
│   │   └── language-composition.md    # Empirical language statistics report
│   ├── milestones/                    # Detailed milestone tracking (M0 through M14)
│   └── development/                   # Toolchain, build, and debug documentation
│
└── research/                          # Preserved research prototypes (Consensus, Nebula)
    ├── consensus/
    └── nebula/
```
