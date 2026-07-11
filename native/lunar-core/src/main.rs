#![no_std]
#![no_main]

mod panic;
mod logging;
mod arch;
mod memory;
mod boot;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    logging::init();
    
    loop {}
}
