# 🌌 PyxisOS

**The first autonomous, AI-native operating system.**

> User → AI Council → Autonomous Agent Network → Operating System → Hardware

PyxisOS reimagines the OS as an intent-coordinator, not just a resource manager: a native multi-agent "Council" reasons about what you're trying to do, an automation engine executes multi-step work on your behalf with your oversight, and — longer-term — a purpose-built kernel and hypervisor secure all of it.

**Status:** early development. Architecture and scope are actively being defined — see [`docs/PRD.md`](docs/PRD.md) for the full plan.

## Two tracks

Building a kernel, a hypervisor, and a 3D engine from scratch is, individually, a multi-year systems effort — comparable in scope to Linux, Xen, and a game engine. So PyxisOS splits into two tracks that run in parallel rather than one long linear roadmap:

- **Track A — PyxisOS Shell.** The AI Council, automation engine, desktop shell, and spatial (3D) prototype — built on a proven Linux/KVM base. This is where the novelty lives, ships in weeks-to-months, and is where most contributions should start.
- **Track B — PyxisOS Native.** A from-scratch kernel, hypervisor, and native spatial engine — a long-horizon systems-research effort for contributors who want to work at that level.

Full reasoning in [`docs/PRD.md`](docs/PRD.md), Section 5.

## The phases

| | Phase | Codename | Track | What it is |
|---|---|---|---|---|
| 🌑 | I — New Moon | Lunar Core | B | The kernel |
| 🌒 | II — Waxing Crescent | Stellar Canvas | A → B | The desktop environment |
| 🌓 | III — First Quarter | Astral Council | A | Multi-agent consensus |
| 🌔 | IV — Waxing Gibbous | Celestial Automation | A | Autonomous workflows |
| 🌕 | V — Full Moon | Orbital Intelligence | A | Adaptive, learning UX |
| 🌖 | VI — Waning Gibbous | Aegis Hypervisor | A → B | Isolation & security |
| 🌗 | VII — Last Quarter | Nebula Engine | A → B | Spatial / 3D computing |
| 🌘 | VIII — Waning Crescent | Constellation Network | A | Multi-device federation |

Full detail on every phase — requirements, acceptance criteria, effort estimates, risks — lives in [`docs/PRD.md`](docs/PRD.md), Section 10.

## What's actually running right now

Most of this repo is scaffolding and specification — that's normal for a project at this stage. One thing is real and runnable today:

- **[`nebula/prototype/index.html`](nebula/prototype/index.html)** — an early Nebula Engine (Phase VII) demo. Open it in a browser: drag to orbit, scroll to zoom, and (with a connected headset, served over HTTPS or `localhost`) hit "Enter VR."

## Repository layout

```
council/         Astral Council — multi-agent consensus (Phase III)
automation/      Celestial Automation — workflow engine (Phase IV)
shell/           Stellar Canvas — desktop environment (Phase II)
intelligence/    Orbital Intelligence — adaptive UX (Phase V)
nebula/          Nebula Engine — spatial/3D computing (Phase VII)
network/         Constellation Network — device federation (Phase VIII)
native/          Track B: Lunar Core (kernel) + Aegis (hypervisor)
research/        Evaluation harnesses & benchmarks
docs/            PRD, architecture notes, research paper drafts
```

Every module folder has its own `README.md` explaining scope, status, and where to look in the PRD.

## Getting started

There's no build yet beyond the Nebula prototype — the project is at the "define the architecture, start on Track A" stage. The best way in:

1. Read [`docs/PRD.md`](docs/PRD.md) for the full plan, especially Section 14 (Open Decisions) — some of the highest-leverage questions (build-vs-adopt on the kernel, LLM backend for the Council) aren't resolved yet.
2. Check open issues labeled `good-first-issue`, or pick by track: `track:A` for the AI/shell layer, `track:B` for kernel/hypervisor/native work.
3. See [`CONTRIBUTING.md`](CONTRIBUTING.md) for branch/PR conventions and how the two tracks are organized.

## License

Apache-2.0 — see [`LICENSE`](LICENSE).
