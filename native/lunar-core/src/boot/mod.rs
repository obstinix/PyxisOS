/// Boot interface parameters.
/// 
/// NOTE: Open Decision #1 (build-from-scratch vs. adopt-seL4) is unresolved.
/// This module provides the abstract interface for boot parameters.
pub struct BootInfo {
    /// Physical address of the memory map table
    pub memory_map_addr: u64,
    /// Number of elements in the memory map table
    pub memory_map_len: u64,
}
