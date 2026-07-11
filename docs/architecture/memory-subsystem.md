# Memory Subsystem

This document describes the planned memory management subsystem for PyxisOS Native (Track B).

> [!NOTE]
> The memory management subsystem is currently in the **Design Stage**. No implementation details are finalized, pending the resolution of Open Decision #1.

## Scope of Memory Management (PRD Phase I)

According to the [PRD.md](file:///c:/ProjectsPP/Pyxis/files/PyxisOS-repo-scaffold/PyxisOS/docs/PRD.md), the memory manager must support:
- Physical Memory Management: Tracking free/allocated page frames using a bitmap or buddy allocator.
- Virtual Memory & Paging: Mapping virtual addresses to physical frames.
- Kernel Heap Allocator: Supporting dynamic memory allocation inside the kernel (`malloc`/`free` or equivalent Rust allocator traits).

## Open Decision #1 Impact

- **Build from scratch**: We will need to write a custom physical frame allocator, set up a custom page table mapper, and implement a custom heap allocator trait (`core::alloc::GlobalAlloc`) in Rust.
- **Adopt seL4**: Memory management will be delegated to seL4 capabilities. The kernel services will request memory resources by manipulating page capabilities through seL4 system calls.

## Design Interfaces

To remain microkernel-agnostic, the `lunar-core` defines the memory allocation interface using abstract traits:

```rust
pub trait MemoryAllocator {
    fn allocate(&self, layout: core::alloc::Layout) -> Result<*mut u8, ()>;
    fn deallocate(&self, ptr: *mut u8, layout: core::alloc::Layout);
}
```
This interface separates the allocator's implementation details from the rest of the kernel modules.
