# System Architecture Overview

This document describes the high-level architecture of PyxisOS, expanding on the core layers and boundaries defined in [PRD.md](file:///c:/ProjectsPP/Pyxis/files/PyxisOS-repo-scaffold/PyxisOS/docs/PRD.md).

## Core Architecture Layers

PyxisOS operates through a five-layer execution model:

1. **User (Intent / Input)**
   - The user interacts with the system through the Stellar Canvas (desktop shell).
   - They specify high-level intents, ask questions, or configure automations.
2. **Astral Council (Consensus Layer)**
   - The primary reasoning component comprising eight specialized agents (Research, Security, Developer, Planner, Creative, Logic, Ethics, Statistical).
   - Resolves intent into structured, auditable plans through consensus.
3. **Celestial Automation (Execution / Workflow Layer)**
   - Decomposes consensus-derived plans into task graphs (DAGs).
   - Executes multi-step workflows with user-visible approval checkpoints.
4. **Operating System (Runtime & Sandbox Layer)**
   - Ensures execution integrity and manages process virtualization.
5. **Hardware (Physical Layer)**
   - Executes code on the target platform (QEMU/VM first, bare metal later).

## The Track A → Track B Migration Path

To facilitate immediate deployment and parallel development, PyxisOS uses a dual-track approach:

### Track A (PyxisOS Shell)
- Runs as a desktop environment on top of a **proven Linux host substrate**.
- Uses KVM or Firecracker microVMs as a stopgap sandboxing layer for executing untrusted autonomous scripts.
- Communication with host resources is performed through standard Linux system calls (APIs).

### Track B (PyxisOS Native)
- Replaces the Linux host substrate with a purpose-built kernel (**Lunar Core**) and a custom Type-1 hypervisor (**Aegis**).
- Transitioning from Track A to Track B requires no refactoring of the Astral Council or Celestial Automation logic, as long as the boundary between the shell and the host substrate remains cleanly abstracted behind generic system interfaces.

```
┌───────────────────────────────────────────────────────────┐
│                 PyxisOS Shell  —  Track A                  │
│                                                             │
│   Stellar Canvas (Desktop/UX)  ⇄  Orbital Intelligence     │
│                          │                                 │
│           Astral Council (Multi-Agent Consensus)           │
│                          │                                 │
│             Celestial Automation (Workflow Engine)         │
│                          │                                 │
│           Constellation Network (Device Federation)        │
└────────────────────────────┬────────────────────────────────┘
                             │  syscalls / APIs
┌────────────────────────────▼────────────────────────────────┐
│  Host substrate                                              │
│   v1 (Track A):  Linux kernel  +  KVM / Firecracker sandbox  │
│   v2+ (Track B): Lunar Core (custom kernel) + Aegis Hypervisor│
└────────────────────────────┬────────────────────────────────┘
                        Physical Hardware
```
