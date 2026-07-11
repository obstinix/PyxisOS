# Boot Sequence

This document describes the planned boot sequence for PyxisOS Native (Track B).

> [!NOTE]
> Lunar Core (Phase I) is currently not started. The bootloader and kernel bootstrapping details remain stubs pending the resolution of Open Decision #1.

## Target Platform & Hardware Configuration

In alignment with **Non-Goal NG3**, the target platform is strictly **x86_64** virtual machines, with **QEMU** as the primary emulator.
- Firmware: BIOS or UEFI (TBD based on Open Decision #1)
- Boot loader: Multiboot2 (GRUB-compatible) or specialized Rust-based bootloaders (like the `bootloader` crate).

## Open Decision #1: Build vs. Adopt

The exact boot process depends on whether we:
1. **Build a custom microkernel from scratch**:
   - The bootloader loads the `lunar-core` ELF binary, sets up basic paging, switches the processor from real mode to 32-bit protected mode, then to 64-bit long mode, and jumps to the entry point `_start`.
2. **Adopt an existing microkernel (e.g., seL4)**:
   - The bootloader initializes the seL4 microkernel.
   - seL4 boots and initializes the root task, which subsequently spawns the Pyxis OS system services.

## Current Scaffolding

A minimal nightly Rust binary is defined under `native/lunar-core/` with a standard entry point:

```rust
#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}
```

This ensures we can build the target compiler target spec and check the toolchain before resolving the boot code.
