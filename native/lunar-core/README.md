# native/lunar-core/ — Lunar Core Microkernel

**Phase 2 · Track B · Native Systems Research**

Lunar Core is PyxisOS's experimental, freestanding `#![no_std]` Rust microkernel architecture.

## Implementation Details

- **Target Architecture:** `x86_64-pyxis.json` (freestanding x86_64, soft-float / no SSE, kernel code model, no red zone).
- **Core Modules:**
  - `src/arch/`: 64-bit GDT descriptor entries and IDT gate setups in safe Rust.
  - `src/boot/`: Multiboot 1 and Multiboot 2 header tags and memory map parsers.
  - `src/memory/`: 4-level PML4 paging table structures, page frame flags, and physical frame allocators.
  - `src/logging.rs`: COM1 (0x3F8) 16550 UART serial logger for early kernel debugging.
  - `src/panic.rs`: Freestanding `#[panic_handler]` reporting CPU architectural state over serial UART.
- **Kernel Integration:** Interoperates with the freestanding C/Assembly kernel core (`arch/`, `kernel/`, `mm/`, `fs/`, `drivers/`).

## Architectural Status

- **Status:** Experimental prototype in active development.
- **Target:** Boots to a serial/VGA prompt in QEMU, validates page tables and task context switches, and prepares for bare-metal testing on physical x86_64 hardware.
- For the full system vision and roadmap, see [`docs/PROJECT_VISION.md`](../../docs/PROJECT_VISION.md) and [`docs/ROADMAP.md`](../../docs/ROADMAP.md).
