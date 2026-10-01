#![no_std]

pub mod sync;
pub mod task;
pub mod scheduler;
pub mod ipc;
pub mod ffi;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {
        core::hint::spin_loop();
    }
}
