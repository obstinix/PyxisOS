# native/lunar-core/ — Lunar Core

**Phase I · Track B**

The foundational kernel: bootloader, memory management, process scheduler, interrupt handling, filesystem, terminal.

**Status:** not started. First open decision (PRD Section 14): build fully from scratch, or on top of an existing minimal microkernel (e.g. seL4) to de-risk the timeline? Resolve this before writing boot code.

**Target for v0.1:** boots to a shell prompt in QEMU, runs a "hello world" userspace binary, smoke tests pass in CI.
