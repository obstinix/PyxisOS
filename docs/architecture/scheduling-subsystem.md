# Scheduling Subsystem

This document describes the planned process and scheduling subsystem for PyxisOS Native (Track B).

> [!NOTE]
> The scheduling and context switching subsystem is implemented via freestanding assembly in `arch/x86_64/context/switch.S`, PIT timer interrupts in `drivers/timer/pit.c`, and safe task management and round-robin scheduler traits in `kernel/src/scheduler.rs` and `kernel/src/task.rs`.

## Scope of Scheduling (PRD Phase I)

According to the [PRD.md](../PRD.md), the process scheduler supports:
- Process Control Block (`Task`) tracking register context, thread state, and priority.
- Preemptive execution scheduling driven by 8254 PIT timer ticks (100 Hz).
- Low-level assembly context switching (`switch_context`) saving callee-saved registers (`rbx`, `rsp`, `rbp`, `r12`-`r15`).

## Architectural Implementation

- **Context Switch Assembly (`arch/x86_64/context/switch.S`):** Switches CPU register state between executing task contexts.
- **Timer Tick Dispatcher (`drivers/timer/pit.c`):** Generates 100 Hz interrupts to track kernel uptime and trigger preemptive scheduling slices.
- **Rust Scheduler Core (`kernel/src/scheduler.rs`, `kernel/src/task.rs`):** Implements priority queues, task state transitions (`Ready`, `Running`, `Blocked`, `Terminated`), and safe run-queue management in Rust.

## Design Interfaces

To remain microkernel-agnostic, the scheduler interfaces are defined abstractly in the kernel modules to handle process states:

```rust
pub enum ThreadState {
    Ready,
    Running,
    Blocked,
    Terminated,
}

pub struct ProcessControlBlock {
    pub pid: u64,
    pub state: ThreadState,
    pub priority: u8,
}
```
These structures represent the minimal required context for tracking tasks inside the kernel before a scheduler algorithm is chosen.
