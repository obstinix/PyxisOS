# Scheduling Subsystem

This document describes the planned process and scheduling subsystem for PyxisOS Native (Track B).

> [!NOTE]
> The scheduling subsystem is currently in the **Design Stage**. No implementation details are finalized, pending the resolution of Open Decision #1.

## Scope of Scheduling (PRD Phase I)

According to the [PRD.md](file:///c:/ProjectsPP/Pyxis/files/PyxisOS-repo-scaffold/PyxisOS/docs/PRD.md), the process scheduler must support:
- Process Control Block (PCB) or thread state tracking.
- Preemptive execution scheduling (e.g., Round Robin, Priority-based, or Multi-Level Feedback Queue).
- Context switching between executing environments.

## Open Decision #1 Impact

- **Build from scratch**: We will implement custom timer interrupt handlers, configure the CPU interrupt state, build a run queue, and write assembly context switches to save and restore registers.
- **Adopt seL4**: Scheduling is largely managed by the seL4 microkernel using its scheduling model (priorities and time slices). Pyxis OS services will focus on managing and mapping the capability nodes of the scheduled threads.

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
