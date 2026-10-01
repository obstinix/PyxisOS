# PyxisOS x86_64 Boot Flow Specification

This document details the actual hardware-to-shell boot architecture of PyxisOS.

---

## 1. High-Level Boot Sequence Diagram

```
                 +-----------------------------------------+
                 |    Firmware (BIOS / UEFI CSM / QEMU)    |
                 +-----------------------------------------+
                                      |
                                      v
                 +-----------------------------------------+
                 |      Multiboot Bootloader (e.g. GRUB)   |
                 +-----------------------------------------+
                                      |
                                      v (Loads ELF at 1MB, 32-bit Protected Mode)
                 +-----------------------------------------+
                 |    arch/x86_64/boot/header.S + boot.S   |
                 |    • Verify CPUID & Long Mode (EFER.LME)|
                 |    • Setup Early Identity Paging (PML4) |
                 |    • Enable PAE (CR4.PAE = 1)           |
                 |    • Enable Paging (CR0.PG = 1)         |
                 |    • Load Early 64-bit GDT              |
                 |    • Far Jump to 64-bit Long Mode       |
                 +-----------------------------------------+
                                      |
                                      v (Calls kernel_main with Multiboot info)
                 +-----------------------------------------+
                 |          kernel/core/main.c             |
                 +-----------------------------------------+
                                      |
         +----------------------------+----------------------------+
         |                            |                            |
         v                            v                            v
+------------------+         +------------------+         +------------------+
| drivers/char/    |         | drivers/video/   |         | arch/x86_64/cpu/ |
| serial_init()    |         | vga_init()       |         | gdt_init() +     |
| (COM1 16550 UART)|         | (VGA 0xB8000)    |         | idt_init()       |
+------------------+         +------------------+         +------------------+
         |                            |                            |
         +----------------------------+----------------------------+
                                      |
                                      v
                 +-----------------------------------------+
                 |    arch/x86_64/interrupts/pic_init()    |
                 |    (Remap 8259 PIC: IRQ0-15 -> 32-47)   |
                 +-----------------------------------------+
                                      |
                                      v
                 +-----------------------------------------+
                 |             mm/pmm_init()               |
                 |    (Physical Frame Bitmap Allocator)    |
                 +-----------------------------------------+
                                      |
                                      v
                 +-----------------------------------------+
                 |             mm/vmm_init()               |
                 |    (4-Level x86_64 Paging Setup)        |
                 +-----------------------------------------+
                                      |
                                      v
                 +-----------------------------------------+
                 |             mm/heap_init()              |
                 |    (Kernel Heap: kmalloc / kfree)       |
                 +-----------------------------------------+
                                      |
                                      v
                 +-----------------------------------------+
                 |           Device Drivers Init           |
                 |    • pit_init(100 Hz timer on IRQ0)     |
                 |    • keyboard_init(PS/2 on IRQ1)        |
                 +-----------------------------------------+
                                      |
                                      v
                 +-----------------------------------------+
                 |       Virtual Filesystem & RamFS        |
                 |    • vfs_init()                         |
                 |    • ramfs_init() (/os-release, /README)|
                 +-----------------------------------------+
                                      |
                                      v
                 +-----------------------------------------+
                 |     Rust Kernel Core Initialization     |
                 |    • rust_kernel_init() via C-FFI       |
                 |    • Task PCB & Priority Scheduler      |
                 |    • IPC Channels & Message Queues      |
                 +-----------------------------------------+
                                      |
                                      v
                 +-----------------------------------------+
                 |         Enable CPU Interrupts           |
                 |                 (sti)                   |
                 +-----------------------------------------+
                                      |
                                      v
                 +-----------------------------------------+
                 |     Userspace / Pyxis Interactive Shell |
                 |    • shell_init() & shell_run()         |
                 |    • 'help', 'info', 'mem', 'uptime'    |
                 +-----------------------------------------+
```

---

## 2. Transitioning to Long Mode (Step-by-Step)

1. **Firmware Handover:** Bootloader jumps to `_start` in `arch/x86_64/boot/boot.S` with `eax = 0x2BADB002` (Multiboot magic) and `ebx` pointing to physical multiboot info.
2. **CPUID Validation:** Checks if the processor supports the CPUID instruction by toggling bit 21 of `EFLAGS`.
3. **Long Mode Probe:** Executes `cpuid` with function `0x80000001` and tests bit 29 of `edx` (`LM` bit). If unsupported, prints `'E'` on VGA buffer and halts.
4. **Early Paging Initialization:** Statically creates PML4, PDPT, and PD tables identity-mapping the lower 64MB of physical memory using 32 x 2MB huge pages (`PTE_HUGE | PTE_PRESENT | PTE_WRITABLE`).
5. **Enabling PAE & LME:**
   - Sets bit 5 (`PAE`) in `CR4`.
   - Writes `boot_pml4` physical address into `CR3`.
   - Reads `IA32_EFER` MSR (`0xC0000080`), sets bit 8 (`LME`), and writes back via `wrmsr`.
6. **Enabling Paging:** Sets bit 31 (`PG`) and bit 0 (`PE`) in `CR0`.
7. **64-bit GDT & Long Jump:** Loads 64-bit GDT (`lgdt gdt64_ptr`) and executes far jump `ljmp $0x08, $long_mode_start` to enter 64-bit submode.
8. **Stack & ABI Call:** Sets up 32KB kernel stack (`stack_top`), loads multiboot parameters into `rdi` and `rsi` according to the System V AMD64 ABI, and calls `kernel_main`.
