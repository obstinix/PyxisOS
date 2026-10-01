#[derive(Copy, Clone, PartialEq, Eq)]
pub enum TaskState {
    Ready,
    Running,
    Blocked,
    Terminated,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum TaskPriority {
    High = 2,
    Normal = 1,
    Low = 0,
}

pub struct Task {
    pub id: u32,
    pub name: [u8; 32],
    pub state: TaskState,
    pub priority: TaskPriority,
    pub rsp: u64,
    pub stack_base: u64,
    pub stack_size: usize,
}

impl Task {
    pub const fn empty() -> Self {
        Self {
            id: 0,
            name: [0; 32],
            state: TaskState::Terminated,
            priority: TaskPriority::Normal,
            rsp: 0,
            stack_base: 0,
            stack_size: 0,
        }
    }
}
