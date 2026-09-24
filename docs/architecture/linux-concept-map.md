# PyxisOS vs Linux Reference Architectural Concept Map

This document maps concepts from the reference Linux kernel source tree ([`linux-kernelsrctree`](https://github.com/obstinix/linux-kernelsrctree)) to PyxisOS, detailing the rationale behind every architectural adoption, adaptation, inspiration, or rejection.

---

## Concept Comparison Matrix

| # | Linux Kernel Concept | PyxisOS Equivalent | Posture | Architectural Rationale |
|---|---|---|---|---|
| **01** | `arch/x86/boot/header.S` (bzImage setup header) | `arch/x86_64/boot/header.S` & `boot.S` | **ADAPT** | PyxisOS targets standard Multiboot / Multiboot2 specification, avoiding Linux-specific real-mode setup protocol while preserving clean multiboot compliance. |
| **02** | `arch/x86/kernel/head_64.S` (Long mode entry) | `arch/x86_64/boot/boot.S` | **ADOPT** | Standard x86_64 requirement: switch to 32-bit protected mode, setup initial PML4 page tables, set PAE and EFER.LME, enable CR0.PG, and jump to 64-bit long mode. |
| **03** | `arch/x86/entry/entry_64.S` (ISR stubs & `pt_regs`) | `arch/x86_64/interrupts/isr.S` (`interrupt_frame`) | **ADOPT** | Push interrupt vector and error code, save all GPRs (`r15`..`rax`), pass pointer to C dispatcher, restore state, and execute `iretq`. Essential for x86_64 stability. |
| **04** | `arch/x86/kernel/cpu/common.c` (GDT / TSS) | `arch/x86_64/cpu/gdt.c` & `gdt.S` | **ADOPT** | Flat 64-bit kernel code/data segments, user code/data segments, and a valid Task State Segment (TSS) holding the Interrupt Stack Table (IST). |
| **05** | `arch/x86/kernel/traps.c` & `irq.c` | `arch/x86_64/cpu/idt.c` & `irq.c` | **ADOPT** | IDT vector table initialization (vectors 0–31 exceptions, 32–47 PIC IRQs, 0x80 syscalls). |
| **06** | `drivers/tty/serial/8250/` (UART driver) | `drivers/char/serial.c` (16550 UART) | **ADAPT** | Essential early kernel debug output. Reimplemented as a clean, polling/interrupt-capable 16550 UART driver at COM1 (`0x3F8`) without the heavy TTY line discipline layer. |
| **07** | `drivers/video/fbdev/` / VGA text mode | `drivers/video/vga.c` (VGA & Framebuffer) | **ADAPT** | Clean text-mode video buffer (`0xB8000`) and linear framebuffer plotting primitives with cursor tracking and ANSI color styling. |
| **08** | `drivers/clocksource/` (PIT 8254 & HPET) | `drivers/timer/pit.c` (8254 PIT) | **ADAPT** | 8254 timer programmed to 100 Hz / 1000 Hz IRQ0 ticks for preemptive scheduling and kernel uptime tracking. |
| **09** | `mm/page_alloc.c` (Buddy Allocator) | `mm/pmm.c` (Bitmap Page Frame Allocator) | **ADAPT** | PyxisOS uses a deterministic physical memory bitmap tracking 4KB page frames reported by Multiboot memory maps, offering $O(1)$ allocation without zone complexity. |
| **10** | `arch/x86/mm/init_64.c` (PML4 4-Level Paging) | `mm/vmm.c` (Virtual Memory Manager) | **ADOPT** | Standard x86_64 4-level paging structures: PML4 (Page Map Level 4), PDPT (Page Directory Pointer Table), PD (Page Directory), and PT (Page Table). |
| **11** | `mm/slub.c` (SLUB object allocator) | `mm/heap.c` (`kmalloc` / `kfree`) | **ADAPT** | A boundary-tag linked-chunk heap allocator for dynamic kernel memory, providing safety checks and zero-fill guarantees. |
| **12** | `kernel/sched/core.c` (CFS / EEVDF Scheduler) | `kernel/src/scheduler.rs` | **INSPIRE** | Reject Linux CFS complexity. Implement a safe, priority-based cooperative and preemptive round-robin scheduler in Rust using a clean Process Control Block (`Task`). |
| **13** | `ipc/` (SysV, POSIX queues, domain sockets) | `kernel/src/ipc.rs` (Message-Passing Channels) | **INSPIRE** | Reject fragmented legacy IPC. Implement clean, microkernel-inspired typed synchronous and asynchronous message channels in Rust. |
| **14** | `fs/` (Virtual Filesystem Switch & Dentry Cache) | `fs/vfs.c` & `fs/ramfs.c` | **ADAPT** | Lightweight `vfs_node` with standard function pointers (`read`, `write`, `open`, `close`, `readdir`) backed by an in-memory root filesystem (`ramfs`), omitting complex RCU dcache locks. |
| **15** | `arch/x86/entry/syscall_64.c` (`syscall` instruction)| `arch/x86_64/syscall/syscall.S` | **ADAPT** | Fast MSR-based system call handling via `IA32_LSTAR` and `IA32_STAR`, backed by legacy software interrupt `int 0x80` compatibility. |
| **16** | `kernel/bpf/` (Extended Berkeley Packet Filter) | *Omitted* | **IGNORE** | Linux eBPF introduces huge attack surface and in-kernel JIT compiler complexity. PyxisOS achieves safety through Rust's type system and hardware isolation. |
| **17** | `kernel/cgroup/` & `kernel/nsproxy.c` (Containers) | *Omitted* | **IGNORE** | Unnecessary for core OS primitives. Process isolation is managed cleanly via distinct CR3 address spaces and user privilege ring 3. |
| **18** | `include/linux/` & `include/uapi/` | `include/pyxis/` & `include/uapi/` | **ADOPT** | Strict separation between internal kernel APIs (`include/pyxis/`) and user-facing syscall/type definitions (`include/uapi/`). |
| **19** | Kbuild (`scripts/Kbuild.include`) | Root `Makefile` + `scripts/` + `tools/` | **ADAPT** | A transparent top-level GNU Makefile orchestrating C, ASM, Rust, and linker scripts with support scripts in Shell and Python. |
