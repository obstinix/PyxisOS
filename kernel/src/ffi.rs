use crate::scheduler::SCHEDULER;
use crate::ipc::{GLOBAL_CHANNEL, Message, MAX_PAYLOAD_SIZE};

extern "C" {
    fn kprintf(fmt: *const u8, ...);
}

#[no_mangle]
pub extern "C" fn rust_kernel_init() {
    let msg = b"[RUST] Pyxis Rust Core initialized (Scheduler, Sync, IPC)\n\0";
    unsafe {
        kprintf(msg.as_ptr());
    }
}

#[no_mangle]
pub extern "C" fn rust_schedule() {
    let mut sched = SCHEDULER.lock();
    sched.schedule_next();
}

#[no_mangle]
pub extern "C" fn rust_ipc_send(sender: u32, receiver: u32, msg_type: u32, data: *const u8, len: usize) -> i32 {
    let mut msg = Message::empty();
    msg.sender = sender;
    msg.receiver = receiver;
    msg.msg_type = msg_type;
    msg.length = core::cmp::min(len, MAX_PAYLOAD_SIZE);

    if !data.is_null() && msg.length > 0 {
        unsafe {
            core::ptr::copy_nonoverlapping(data, msg.payload.as_mut_ptr(), msg.length);
        }
    }

    let mut chan = GLOBAL_CHANNEL.lock();
    if chan.send(&msg) { 0 } else { -1 }
}

#[no_mangle]
pub extern "C" fn rust_ipc_recv(out_sender: *mut u32, out_type: *mut u32, out_data: *mut u8, max_len: usize) -> i32 {
    let mut msg = Message::empty();
    let mut chan = GLOBAL_CHANNEL.lock();

    if chan.receive(&mut msg) {
        unsafe {
            if !out_sender.is_null() { *out_sender = msg.sender; }
            if !out_type.is_null() { *out_type = msg.msg_type; }
            if !out_data.is_null() && max_len > 0 {
                let to_copy = core::cmp::min(max_len, msg.length);
                core::ptr::copy_nonoverlapping(msg.payload.as_ptr(), out_data, to_copy);
            }
        }
        msg.length as i32
    } else {
        -1
    }
}
