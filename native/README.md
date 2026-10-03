# native/ — PyxisOS Native Systems Workspace (Track B)

This directory contains the native Rust workspace for PyxisOS's low-level systems architecture and AI coordination engine:

- `lunar-core/` — Experimental `#![no_std]` Rust microkernel architecture (task scheduling, memory abstraction traits, early logging, and target specifications).
- `consensus/` — Astral Consensus Engine: native Rust multi-agent reasoning, evidence submission, and deterministic decision arbitration.
- `aegis/` — Planned capability-based isolation and execution sandbox. (Track A operational sandboxing is provided by host virtualization/containment).

For comprehensive technical specifications and milestone status, see:
- [Long-Term Project Vision](../docs/PROJECT_VISION.md)
- [Phased Milestone Roadmap](../docs/ROADMAP.md)
- [Canonical Project Status](../docs/PROJECT_STATUS.md)
