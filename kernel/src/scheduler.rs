use crate::task::{Task, TaskState, TaskPriority};
use crate::sync::Spinlock;

pub const MAX_TASKS: usize = 16;

extern "C" {
    fn pyxis_context_switch(old_rsp: *mut u64, new_rsp: u64);
}

pub struct Scheduler {
    tasks: [Task; MAX_TASKS],
    current_idx: usize,
    task_count: usize,
}

impl Scheduler {
    pub const fn new() -> Self {
        Self {
            tasks: [const { Task::empty() }; MAX_TASKS],
            current_idx: 0,
            task_count: 0,
        }
    }

    pub fn spawn(&mut self, name: &[u8], rsp: u64, stack_base: u64, stack_size: usize) -> Option<u32> {
        for (i, task) in self.tasks.iter_mut().enumerate() {
            if task.state == TaskState::Terminated {
                task.id = (i + 1) as u32;
                task.state = TaskState::Ready;
                task.priority = TaskPriority::Normal;
                task.rsp = rsp;
                task.stack_base = stack_base;
                task.stack_size = stack_size;

                let len = core::cmp::min(name.len(), 31);
                task.name[..len].copy_from_slice(&name[..len]);
                task.name[len] = 0;

                self.task_count += 1;
                return Some(task.id);
            }
        }
        None
    }

    pub fn schedule_next(&mut self) {
        if self.task_count <= 1 {
            return;
        }

        let old_idx = self.current_idx;
        let mut next_idx = (old_idx + 1) % MAX_TASKS;

        for _ in 0..MAX_TASKS {
            if self.tasks[next_idx].state == TaskState::Ready {
                break;
            }
            next_idx = (next_idx + 1) % MAX_TASKS;
        }

        if next_idx == old_idx || self.tasks[next_idx].state != TaskState::Ready {
            return;
        }

        if self.tasks[old_idx].state == TaskState::Running {
            self.tasks[old_idx].state = TaskState::Ready;
        }

        self.tasks[next_idx].state = TaskState::Running;
        self.current_idx = next_idx;

        unsafe {
            let old_rsp_ptr = &mut self.tasks[old_idx].rsp as *mut u64;
            let new_rsp = self.tasks[next_idx].rsp;
            pyxis_context_switch(old_rsp_ptr, new_rsp);
        }
    }

    pub fn task_count(&self) -> usize {
        self.task_count
    }
}

pub static SCHEDULER: Spinlock<Scheduler> = Spinlock::new(Scheduler::new());
