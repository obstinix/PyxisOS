//! Lunar Core Kernel Entry Point (Phase I · Track B)
//!
//! Native x86_64 microkernel core implementing architecture initialization,
//! physical memory detection, and early serial diagnostics.

#![no_std]
#![no_main]

pub mod arch;
pub mod boot;
pub mod logging;
pub mod memory;
mod panic;

use arch::CpuArch;
use boot::BootInfo;
use memory::BumpAllocator;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // 1. Initialize serial logging
    logging::init();
    serial_println!("[Lunar-Core] PyxisOS native substrate active.");

    // 2. Initialize CPU Architecture state (GDT, descriptors)
    arch::X86_64::init();
    serial_println!("[Lunar-Core] CPU architecture state (GDT) established.");

    // 3. Inspect boot information
    let boot_info = BootInfo::empty();
    serial_println!("[Lunar-Core] Boot magic verified: {}", boot_info.is_valid_multiboot());

    // 4. Initialize early memory allocator
    let mut _early_alloc = BumpAllocator::new(0x200000, 0x100000);
    serial_println!("[Lunar-Core] Early memory bump allocator active (1MB heap).");

    // 5. Enter kernel halt loop
    loop {
        unsafe {
            arch::hlt();
        }
    }
}
