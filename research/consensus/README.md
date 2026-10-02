# Astral Consensus Engine (Phase III · Track A)

The multi-agent consensus layer for PyxisOS. Each specialist agent (Research, Security, Logic, Developer, Planner, Creative, Ethics, Statistical) independently analyzes requests; the Decision Arbitration Layer synthesizes their input into a transparent answer with conflict detection.

## Architecture & Implementation

The Astral Consensus Engine has been migrated from early TypeScript prototyping into **native Rust** located in [`native/consensus/`](../../native/consensus/).

- **Native Rust Crate:** `native/consensus`
- **CLI Executable:** `cargo run --manifest-path native/consensus/Cargo.toml -- "query"`
- **Specialist Agents:** `ResearchAgent`, `SecurityAgent`, `LogicAgent` in `native/consensus/src/agents/`
- **Decision Arbitration Layer:** `native/consensus/src/arbitration.rs`
- **Evaluation Suite:** Python research harness in [`research/eval/`](../eval/)

See [`docs/PRD.md`](../../docs/PRD.md), Section 10 (Phase III) for full specifications.
