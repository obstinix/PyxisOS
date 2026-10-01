use crate::sync::Spinlock;

pub const MAX_IPC_MESSAGES: usize = 32;
pub const MAX_PAYLOAD_SIZE: usize = 64;

#[derive(Copy, Clone)]
pub struct Message {
    pub sender: u32,
    pub receiver: u32,
    pub msg_type: u32,
    pub length: usize,
    pub payload: [u8; MAX_PAYLOAD_SIZE],
}

impl Message {
    pub const fn empty() -> Self {
        Self {
            sender: 0,
            receiver: 0,
            msg_type: 0,
            length: 0,
            payload: [0; MAX_PAYLOAD_SIZE],
        }
    }
}

pub struct Channel {
    queue: [Message; MAX_IPC_MESSAGES],
    head: usize,
    tail: usize,
    count: usize,
}

impl Channel {
    pub const fn new() -> Self {
        Self {
            queue: [const { Message::empty() }; MAX_IPC_MESSAGES],
            head: 0,
            tail: 0,
            count: 0,
        }
    }

    pub fn send(&mut self, msg: &Message) -> bool {
        if self.count >= MAX_IPC_MESSAGES {
            return false;
        }
        self.queue[self.tail] = *msg;
        self.tail = (self.tail + 1) % MAX_IPC_MESSAGES;
        self.count += 1;
        true
    }

    pub fn receive(&mut self, dest: &mut Message) -> bool {
        if self.count == 0 {
            return false;
        }
        *dest = self.queue[self.head];
        self.head = (self.head + 1) % MAX_IPC_MESSAGES;
        self.count -= 1;
        true
    }

    pub fn len(&self) -> usize {
        self.count
    }
}

pub static GLOBAL_CHANNEL: Spinlock<Channel> = Spinlock::new(Channel::new());
