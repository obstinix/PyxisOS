use core::panic::PanicInfo;
use crate::serial_println;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("\n[LUNAR-CORE PANIC] {}", info);
    loop {
        unsafe {
            crate::arch::hlt();
        }
    }
}
