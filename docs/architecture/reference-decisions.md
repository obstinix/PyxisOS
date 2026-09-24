# PyxisOS Architectural Reference Decisions: Linux Kernel Analysis

## 1. Purpose and Philosophy

PyxisOS uses the educational reference repository [`linux-kernelsrctree`](https://github.com/obstinix/linux-kernelsrctree) to study battle-tested kernel source-tree organization, subsystem boundaries, hardware abstraction patterns, and build modularity.

**Core Principle:**
PyxisOS is **not** a Linux clone and will not copy Linux source code, subsystem implementations, or unneeded complexity. Instead, every architectural pattern examined from the Linux reference tree is explicitly evaluated, classified, and justified.

---

## 2. Classification Criteria

To prevent architectural cargo culting, each reference concept is categorized into one of four explicit postures:

| Classification | Meaning | PyxisOS Application Rule |
|---|---|---|
| **ADOPT** | Use directly as standard engineering practice | Concept is universal to x86_64 systems programming (e.g., IDT vector layout, multiboot headers, standard fixed-width types). |
| **ADAPT** | Modify significantly to fit PyxisOS's hybrid/microkernel model | Concept is valuable, but stripped of Linux's monolithic legacy overhead (e.g., VFS, paging, interrupt frames). |
| **INSPIRE** | Borrow the high-level design idea; implement independently | Design pattern serves as an architectural beacon, but implemented with modern C and Rust abstractions (e.g., device driver registration, scheduler traits). |
| **IGNORE** | Deliberately reject | Mechanism introduces unnecessary complexity, legacy workarounds, or violates PyxisOS's goals (e.g., eBPF, cgroups, complex NUMA zones, monolithic complexity). |

---

## 3. Subsystem Evaluation and Reference Decisions

### 3.1 Architecture Separation (`arch/x86/`)
- **Reference Pattern:** Linux strictly isolates all architecture-dependent assembly, CPU-specific headers, and page table bootstrap code under `arch/<architecture>/`.
- **Classification:** **`ADOPT`**
- **Decision:** PyxisOS establishes `arch/x86_64/` with distinct directories for `boot/`, `cpu/`, `interrupts/`, `context/`, `syscall/`, and `linker/`. Architecture-specific assembly will never be scattered across generic kernel subsystems.

### 3.2 Interrupt Handling & Context State
- **Reference Pattern:** Assembly ISR stubs push vector numbers and error codes, construct an interrupt context frame (`pt_regs`), switch stacks, and invoke generic C handlers.
- **Classification:** **`ADOPT`**
- **Decision:** PyxisOS adopts an identical register-save frame (`struct interrupt_frame`) in `arch/x86_64/interrupts/isr.S`. This frame is passed by pointer to C/Rust dispatchers, ensuring predictable ABI transitions and safe state restoration on `iretq`.

### 3.3 Virtual Filesystem (VFS)
- **Reference Pattern:** Heavyweight VFS with dentry caches, inode caches, superblock management, and complex RCU-based lockless path lookup.
- **Classification:** **`ADAPT`**
- **Decision:** PyxisOS adapts the inode and file operations abstraction (`vfs_node`, `vfs_ops` with `open`, `read`, `write`, `close`, `readdir`), but replaces the complex dcache/RCU hierarchy with a deterministic, lightweight in-memory directory tree suitable for an initial `ramfs` and initramfs.

### 3.4 Physical & Virtual Memory Management
- **Reference Pattern:** Buddy allocator for page frames, SLUB allocator for kernel objects, multi-zone memory (DMA, Normal, HighMem), and 4/5-level page table management.
- **Classification:** **`ADAPT`**
- **Decision:** PyxisOS adapts standard x86_64 4-level paging (PML4, PDPT, PD, PT) with 4KB pages and 2MB huge-page support. For physical memory, a bitmap frame allocator is implemented for simplicity and verification. For kernel heap, a clean, boundary-tagged chunk allocator (`kmalloc`/`kfree`) is used, avoiding Linux's NUMA and zone complexity.

### 3.5 Device Driver Architecture
- **Reference Pattern:** Monolithic `struct device`, `struct bus_type`, kobject hierarchies, and dynamic sysfs nodes.
- **Classification:** **`INSPIRE`**
- **Decision:** PyxisOS implements a clean, modular hardware abstraction interface in C. Device drivers (serial UART, VGA text/linear framebuffer, PIT timer, PS/2 keyboard) expose standardized function pointers (`init`, `read`, `write`, `ioctl`) without dynamic kobject overhead.

### 3.6 Process Scheduling & Task Control
- **Reference Pattern:** Completely Fair Scheduler (CFS) and Earliest Eligible Virtual Deadline First (EEVDF) with hundreds of fields in `struct task_struct`.
- **Classification:** **`INSPIRE`**
- **Decision:** PyxisOS rejects CFS/EEVDF in favor of a clean, priority-based cooperative and preemptive round-robin scheduler implemented in memory-safe Rust. The Process Control Block (`Task`) contains only essential architectural state, registers, memory context, and scheduling state.

### 3.7 Inter-Process Communication (IPC)
- **Reference Pattern:** A collection of legacy subsystems: System V IPC (semaphores, shm, msg), POSIX message queues, Unix domain sockets, and pipes.
- **Classification:** **`ADAPT`**
- **Decision:** PyxisOS unifies IPC into a microkernel-inspired message-passing model in Rust, providing typed synchronous/asynchronous channels (`Port` and `Channel`) and fast shared-memory buffers.

### 3.8 Build Infrastructure & Tooling
- **Reference Pattern:** Complex Kbuild framework using nested Makefiles, Kconfig parser, and recursive expansion.
- **Classification:** **`ADAPT`**
- **Decision:** PyxisOS provides a single, readable top-level GNU `Makefile` supported by clean Shell and Python scripts under `scripts/` and `tools/`. Build targets (`build`, `run`, `debug`, `test`, `check`, `clean`) are transparent and reproducible.

### 3.9 Monolithic In-Kernel Virtualization (eBPF, Cgroups, Namespaces)
- **Reference Pattern:** In-kernel bytecode verification (eBPF), complex resource controllers (cgroups v2), and multi-tenant namespaces.
- **Classification:** **`IGNORE`**
- **Decision:** Deliberately omitted. PyxisOS maintains security through language-level memory safety (Rust), hardware privilege levels (Ring 0 / Ring 3), and clean address-space isolation, avoiding in-kernel bytecode interpreters.
