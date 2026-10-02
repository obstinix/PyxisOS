//! Boot interface and Multiboot parameter parsing.

pub const MULTIBOOT_BOOTLOADER_MAGIC: u32 = 0x2BADB002;
pub const MULTIBOOT2_BOOTLOADER_MAGIC: u32 = 0x36D76289;

pub const MEMORY_TYPE_AVAILABLE: u32 = 1;
pub const MEMORY_TYPE_RESERVED: u32 = 2;
pub const MEMORY_TYPE_ACPI_RECLAIMABLE: u32 = 3;
pub const MEMORY_TYPE_NVS: u32 = 4;
pub const MEMORY_TYPE_BADRAM: u32 = 5;

/// Physical memory map entry as provided by Multiboot.
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct MultibootMmapEntry {
    pub size: u32,
    pub addr: u64,
    pub len: u64,
    pub entry_type: u32,
}

/// Raw Multiboot info structure passed from bootloader.
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct MultibootRawInfo {
    pub flags: u32,
    pub mem_lower: u32,
    pub mem_upper: u32,
    pub boot_device: u32,
    pub cmdline: u32,
    pub mods_count: u32,
    pub mods_addr: u32,
    pub syms: [u32; 4],
    pub mmap_length: u32,
    pub mmap_addr: u32,
    pub drives_length: u32,
    pub drives_addr: u32,
    pub config_table: u32,
    pub boot_loader_name: u32,
    pub apm_table: u32,
    pub vbe_control_info: u32,
    pub vbe_mode_info: u32,
    pub vbe_mode: u16,
    pub vbe_interface_seg: u16,
    pub vbe_interface_off: u16,
    pub vbe_interface_len: u16,
    pub framebuffer_addr: u64,
    pub framebuffer_pitch: u32,
    pub framebuffer_width: u32,
    pub framebuffer_height: u32,
    pub framebuffer_bpp: u8,
    pub framebuffer_type: u8,
    pub color_info: [u8; 6],
}

/// High-level boot parameters validated and normalized for Lunar Core.
#[derive(Debug, Clone, Copy)]
pub struct BootInfo {
    pub magic: u32,
    pub memory_lower_kb: u64,
    pub memory_upper_kb: u64,
    pub memory_map_addr: u64,
    pub memory_map_len: u64,
    pub cmdline_addr: u64,
}

impl BootInfo {
    pub const fn empty() -> Self {
        Self {
            magic: 0,
            memory_lower_kb: 0,
            memory_upper_kb: 0,
            memory_map_addr: 0,
            memory_map_len: 0,
            cmdline_addr: 0,
        }
    }

    /// Validates the Multiboot magic identifier.
    pub fn is_valid_multiboot(&self) -> bool {
        self.magic == MULTIBOOT_BOOTLOADER_MAGIC || self.magic == MULTIBOOT2_BOOTLOADER_MAGIC
    }

    /// Total detected physical memory in kilobytes.
    pub fn total_memory_kb(&self) -> u64 {
        self.memory_lower_kb + self.memory_upper_kb
    }
}
