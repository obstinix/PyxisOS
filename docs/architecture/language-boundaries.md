# PyxisOS Language & Subsystem Boundaries

This document defines the ABI contracts, data representations, calling conventions, and foreign function interfaces (FFI) governing interactions across language boundaries in PyxisOS.

---

## 1. Boundary Architecture Overview

```
 ┌────────────────────────────────────────────────────────┐
 │                   PyxisOS User Shell                   │
 └──────────────────────────┬─────────────────────────────┘
                            │ System Call Interface (int 0x80 / syscall)
 ┌──────────────────────────▼─────────────────────────────┐
 │                      C Subsystems                      │
 │    • Memory (PMM / VMM / Heap)                         │
 │    • Drivers (Serial, VGA, PIT, PS/2 Keyboard)         │
 │    • VFS & RamFS                                       │
 └─────────────┬────────────────────────────▲─────────────┘
               │ C-FFI Calls                │ C Callbacks / Primitives
               ▼                            │
 ┌──────────────────────────┐  ┌────────────┴─────────────┐
 │      Rust Kernel Core    │  │     x86_64 Assembly      │
 │  • Scheduler             │  │  • Bootstrap             │
 │  • Task Control Blocks   │  │  • Context Switching     │
 │  • IPC Channels          │  │  • GDT/IDT/TSS Flush     │
 │  • Spinlocks             │  │  • ISR Vector Tables     │
 └──────────────────────────┘  └──────────────────────────┘
```

---

## 2. ABI and Calling Conventions

All subsystems target the **System V AMD64 ABI**:
- **Integer / Pointer Arguments:** Passed in registers `%rdi`, `%rsi`, `%rdx`, `%rcx`, `%r8`, `%r9`.
- **Return Values:** Returned in `%rax` (low 64 bits) and `%rdx` (high 64 bits).
- **Callee-Saved Registers:** `%rbx`, `%rsp`, `%rbp`, `%r12`, `%r13`, `%r14`, `%r15`. These are preserved across function calls and saved during cooperative context switches in `arch/x86_64/context/switch.S`.
- **Red Zone:** Disabled across all C, Rust, and Assembly compilation units using `-mno-red-zone`. This ensures interrupts do not corrupt stack frames below `%rsp`.

---

## 3. Assembly to C / Rust Boundary

- **Entry Point Handover:**
  `arch/x86_64/boot/boot.S` sets up 64-bit segments, loads `%rsp` with `stack_top`, places Multiboot info in `%rdi` and magic in `%rsi`, and invokes `kernel_main`.
- **Interrupt Handling:**
  `arch/x86_64/interrupts/isr.S` saves all 15 general-purpose registers, loads `%rsp` into `%rdi`, and calls `irq_dispatch(struct interrupt_frame *frame)` in `arch/x86_64/interrupts/irq.c`.
- **Context Switch:**
  `pyxis_context_switch(uint64_t *old_rsp_ptr, uint64_t new_rsp)` pushes callee-saved registers, stores current `%rsp` in `*old_rsp_ptr`, loads `new_rsp` into `%rsp`, pops registers, and executes `ret`.

---

## 4. C to Rust FFI Boundary

The Rust kernel core exposes a clean `extern "C"` API linked as a static library (`libpyxis_kernel.a`):

### Exported by Rust:
```rust
#[no_mangle]
pub extern "C" fn rust_kernel_init();

#[no_mangle]
pub extern "C" fn rust_schedule();

#[no_mangle]
pub extern "C" fn rust_ipc_send(sender: u32, receiver: u32, msg_type: u32, data: *const u8, len: usize) -> i32;

#[no_mangle]
pub extern "C" fn rust_ipc_recv(out_sender: *mut u32, out_type: *mut u32, out_data: *mut u8, max_len: usize) -> i32;
```

### Imported by Rust from C:
```rust
extern "C" {
    fn kprintf(fmt: *const u8, ...);
    fn pyxis_context_switch(old_rsp: *mut u64, new_rsp: u64);
}
```

### Safety Principles:
1. **No Panics across FFI:** Rust uses `panic = "abort"`. Any panic inside Rust immediately halts the CPU safely without unwinding into C stack frames.
2. **Buffer Bounds Checking:** All IPC message buffers are clamped to `MAX_PAYLOAD_SIZE` (64 bytes) with explicit bounds verification.
3. **Null Pointer Checks:** Raw pointers passed across the boundary are verified against null before dereferencing.
