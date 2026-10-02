//! Memory management, paging descriptors, and frame allocation for Lunar Core.

/// Standard page sizes on x86_64.
pub const PAGE_SIZE_4K: u64 = 4096;
pub const PAGE_SIZE_2M: u64 = 2 * 1024 * 1024;
pub const PAGE_SIZE_1G: u64 = 1024 * 1024 * 1024;

/// Strongly-typed wrapper for physical addresses.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct PhysicalAddress(pub u64);

impl PhysicalAddress {
    pub const fn new(addr: u64) -> Self {
        Self(addr)
    }

    pub const fn as_u64(&self) -> u64 {
        self.0
    }

    pub const fn is_aligned(&self, alignment: u64) -> bool {
        self.0 % alignment == 0
    }
}

/// Strongly-typed wrapper for virtual addresses.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct VirtualAddress(pub u64);

impl VirtualAddress {
    pub const fn new(addr: u64) -> Self {
        Self(addr)
    }

    pub const fn as_u64(&self) -> u64 {
        self.0
    }

    pub const fn pml4_index(&self) -> usize {
        ((self.0 >> 39) & 0x1FF) as usize
    }

    pub const fn pdpt_index(&self) -> usize {
        ((self.0 >> 30) & 0x1FF) as usize
    }

    pub const fn pd_index(&self) -> usize {
        ((self.0 >> 21) & 0x1FF) as usize
    }

    pub const fn pt_index(&self) -> usize {
        ((self.0 >> 12) & 0x1FF) as usize
    }
}

/// 64-bit page table flags for PML4, PDPT, PD, and PT entries.
pub struct PageTableFlags;

impl PageTableFlags {
    pub const PRESENT: u64 = 1 << 0;
    pub const WRITABLE: u64 = 1 << 1;
    pub const USER_ACCESSIBLE: u64 = 1 << 2;
    pub const WRITE_THROUGH: u64 = 1 << 3;
    pub const NO_CACHE: u64 = 1 << 4;
    pub const ACCESSED: u64 = 1 << 5;
    pub const DIRTY: u64 = 1 << 6;
    pub const HUGE_PAGE: u64 = 1 << 7;
    pub const GLOBAL: u64 = 1 << 8;
    pub const NO_EXECUTE: u64 = 1 << 63;
}

/// Single 64-bit page table entry.
#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct PageTableEntry(pub u64);

impl PageTableEntry {
    pub const fn empty() -> Self {
        Self(0)
    }

    pub const fn is_present(&self) -> bool {
        (self.0 & PageTableFlags::PRESENT) != 0
    }

    pub const fn is_writable(&self) -> bool {
        (self.0 & PageTableFlags::WRITABLE) != 0
    }

    pub const fn physical_address(&self) -> PhysicalAddress {
        PhysicalAddress(self.0 & 0x000F_FFFF_FFFF_F000)
    }

    pub fn set(&mut self, addr: PhysicalAddress, flags: u64) {
        self.0 = (addr.as_u64() & 0x000F_FFFF_FFFF_F000) | (flags & !0x000F_FFFF_FFFF_F000);
    }
}

/// Abstract memory allocator trait.
pub trait MemoryAllocator {
    fn allocate(&mut self, layout: core::alloc::Layout) -> Result<*mut u8, ()>;
    fn deallocate(&mut self, ptr: *mut u8, layout: core::alloc::Layout);
}

/// Simple bump allocator for early boot stages.
pub struct BumpAllocator {
    next: usize,
    limit: usize,
}

impl BumpAllocator {
    pub const fn new(start: usize, size: usize) -> Self {
        Self {
            next: start,
            limit: start + size,
        }
    }
}

impl MemoryAllocator for BumpAllocator {
    fn allocate(&mut self, layout: core::alloc::Layout) -> Result<*mut u8, ()> {
        let align = layout.align();
        let alloc_start = (self.next + align - 1) & !(align - 1);
        let alloc_end = alloc_start.checked_add(layout.size()).ok_or(())?;

        if alloc_end <= self.limit {
            self.next = alloc_end;
            Ok(alloc_start as *mut u8)
        } else {
            Err(())
        }
    }

    fn deallocate(&mut self, _ptr: *mut u8, _layout: core::alloc::Layout) {
        // Bump allocator does not reclaim individual allocations
    }
}
