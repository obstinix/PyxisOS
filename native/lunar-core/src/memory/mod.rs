/// Memory allocator trait.
/// 
/// NOTE: Open Decision #1 (build-from-scratch vs. adopt-seL4) is unresolved.
/// This trait outlines the interface for memory allocation without prescribing a design.
pub trait MemoryAllocator {
    /// Allocate memory block
    fn allocate(&self, layout: core::alloc::Layout) -> Result<*mut u8, ()>;
    /// Deallocate memory block
    fn deallocate(&self, ptr: *mut u8, layout: core::alloc::Layout);
}
