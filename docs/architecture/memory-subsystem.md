# Memory Subsystem

This document describes the planned memory management subsystem for PyxisOS Native (Track B).

> [!NOTE]
> The memory management subsystem is implemented in freestanding C and x86_64 assembly in `mm/` (bitmap physical frame allocator, 4-level PML4 paging, heap chunk allocator), with corresponding Rust memory abstraction traits in `native/lunar-core/src/memory/`.

## Scope of Memory Management (PRD Phase I)

According to the [PRD.md](../PRD.md), the memory manager supports:
- Physical Memory Management: Tracking free/allocated page frames using a bitmap frame allocator (`mm/pmm.c`).
- Virtual Memory & Paging: 4-level PML4 address space mapping (`mm/vmm.c`).
- Kernel Heap Allocator: Boundary-tagged chunk allocator providing `kmalloc()` and `kfree()` (`mm/heap.c`).

## Architectural Implementation

- **Physical Frame Allocator (`mm/pmm.c`):** Tracks 4KB physical pages across detected Multiboot memory zones via a memory bitmap.
- **Virtual Memory Manager (`mm/vmm.c`):** Implements x86_64 4-level paging (PML4, PDPT, PD, PT) with support for 4KB pages and 2MB huge pages.
- **Kernel Heap (`mm/heap.c`):** Manages dynamic kernel allocation with header chunk validation and zero-fill guarantees.

## Design Interfaces

To remain microkernel-agnostic, the `lunar-core` defines the memory allocation interface using abstract traits:

```rust
pub trait MemoryAllocator {
    fn allocate(&self, layout: core::alloc::Layout) -> Result<*mut u8, ()>;
    fn deallocate(&self, ptr: *mut u8, layout: core::alloc::Layout);
}
```
This interface separates the allocator's implementation details from the rest of the kernel modules.
