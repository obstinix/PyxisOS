# PyxisOS — Product Requirements Document

| | |
|---|---|
| **Project** | PyxisOS — Autonomous AI-Native Operating System |
| **Repository** | https://github.com/obstinix/PyxisOS |
| **License** | Apache-2.0 (already in repo ✅) |
| **Status** | Draft v0.3 |
| **Last updated** | July 12, 2026 |

**Changelog**
- **v0.3** — Modernized architecture documentation (added system overview, boot sequence, memory, and scheduling subsystems), formalized research paper outline, migrated dev environment setup to Rust toolchain, and established Rust workspace in `native/`.
- **v0.2** — Nebula Engine (Phase VII) expanded with a concrete tech decision, a scoped v0.1 milestone, and a real starter prototype now in the repo at `nebula/prototype/`. Repo structure (Section 9) synced to the actual scaffold. README.md, CONTRIBUTING.md, and issue templates added.
- **v0.1** — Initial draft.

> **TL;DR / key recommendation:** The original 8-phase vision is preserved in full below — same codenames, same phases, same architecture. The one thing this PRD adds is a **two-track delivery strategy** (Section 5): split the plan into an **AI/UX track** that can ship in weeks on top of proven components, and a **from-scratch systems track** (kernel, hypervisor, spatial engine) that is a genuine multi-year systems-research effort. Running them as one linear roadmap risks the project stalling at Phase I before anyone ever sees the part that makes PyxisOS different. Running them in parallel tracks lets you demonstrate the AI-native experience immediately while the deep systems work proceeds at its own pace.

---

## 1. Overview & How to Use This Document

This PRD turns the PyxisOS vision doc into something buildable: scoped requirements, acceptance criteria, a suggested repo structure, and a roadmap with honest effort estimates. It's meant to live at `docs/PRD.md` in the repo and evolve as decisions get made — treat the "Open Decisions" in Section 14 as the first thing to resolve as a team/community.

**Assumptions made in this draft** (flag any of these to change the plan):
- No team size was specified, so estimates are given as ranges and explicitly scaled for both a small dedicated team (2–5 people) and a solo/part-time contributor.
- The repo is public and the license is already Apache-2.0, so this assumes an open-source, contributor-friendly development model.
- Initial target platform is x86_64 (desktop + VM); mobile/ARM is out of scope for v1.
- The project has two goals in tension — *ship something real* and *produce research-paper-worthy results* (both are explicit in the source vision doc) — this PRD tries to serve both rather than picking one.

---

## 2. Background & Problem Statement

Modern operating systems are built around a fixed pipeline:

> User → Application → Operating System → Hardware

AI shows up only at the application layer — a chatbot window, a copilot plugin — bolted onto a system that has no native concept of intent, delegation, or multi-agent reasoning. This creates real, specific problems worth solving:

- **Fragmented automation.** Every app has its own bot; none of them share context or coordinate.
- **Context switching.** Users move between the OS, apps, and AI assistants as separate silos.
- **Single-model bias.** One assistant, one perspective, no adversarial or specialist review of its own answers.
- **No OS-level trust model for autonomous action.** Existing permission systems (file access, notifications) were never designed for an agent that plans and executes multi-step tasks on a user's behalf.

## 3. Vision

PyxisOS reframes the pipeline as:

> User → Astral Consensus Engine → Celestial Automation → Operating System → Hardware

The OS becomes an intent-coordinator, not just a resource manager: it interprets what the user is trying to do, routes work to specialized agents, executes it through secured, sandboxed automation, and continuously adapts — while keeping the user in control. Applications become services in a larger ecosystem rather than isolated silos.

## 4. Goals & Non-Goals

**Goals**

| ID | Goal |
|---|---|
| G1 | Ship a working multi-agent **Astral Consensus Engine** that gives a user request independent, specialist analysis and a transparent, synthesized answer. |
| G2 | Provide autonomous multi-step workflow execution (**Celestial Automation**) with user oversight and a clear audit trail. |
| G3 | Build a distinctive desktop shell (**Stellar Canvas**) as the primary surface for the above. |
| G4 | Establish PyxisOS as a credible open research platform for human-AI collaborative computing — generating the data the companion research paper needs. |
| G5 *(long-horizon)* | Deliver a purpose-built AI-native kernel, hypervisor, and spatial computing environment (**Lunar Core**, **Aegis**, **Nebula Engine**). |

**Non-Goals (v1)** — explicit, to protect the roadmap from scope collapse:

- **NG1** — Not attempting to replace Linux/Windows/macOS as anyone's daily-driver OS in year one.
- **NG2** — Not writing GPU/Wi-Fi/USB drivers from scratch initially — v1 rides on an existing kernel's driver stack.
- **NG3** — Not targeting arbitrary consumer hardware early — target QEMU/VM and a short list of well-supported machines first.
- **NG4** — Not building a from-scratch Type-1 hypervisor before it can be security-reviewed — v1 sandboxing rides on KVM/Firecracker.

## 5. Recommended Delivery Strategy: Two Tracks

This is the main structural change this PRD makes to the original plan, so it's worth stating plainly: **a bootloader-up kernel, a custom hypervisor, and a from-scratch 3D compositor are, individually, projects on the scale of Linux, Xen, and a game engine.** Those took large communities years to decades. Sequencing PyxisOS's 8 phases *literally in order* means nothing user-visible ships until after Phase I (a custom kernel) is done — historically the point where most from-scratch OS projects stall.

The fix isn't to shrink the ambition — it's to separate what's genuinely novel here (the AI layer) from what's genuinely hard-but-solved-elsewhere (kernels, hypervisors, 3D engines):

- **Track A — "PyxisOS Shell"** (Phases II, III, IV, V, VIII, and prototypes of VI/VII): the Astral Consensus Engine, Celestial Automation, Stellar Canvas, and friends, built as an application/desktop-shell layer on a proven Linux kernel with KVM/Firecracker for sandboxing. This is where the actual novelty lives, and it can be demonstrated, tested, and published on in **weeks to months**, not years.
- **Track B — "PyxisOS Native"** (Phase I in full, plus native Phase VI/VII once Track A validates the concepts): the from-scratch kernel, custom hypervisor, and native spatial engine, run as a long-horizon systems-research effort — ideally by contributors who specialize in kernels/security/graphics, once Track A has proven the AI-native UX is worth the investment (and, realistically, once the project has attracted the specialist contributors or funding that this kind of work requires).

Every phase below is tagged with its track. Nothing about the original phases, codenames, or architecture changes — this just tells you what can start **this month** versus what's a **multi-year parallel effort**.

## 6. Target Users

| Persona | Needs |
|---|---|
| **Alex, the power user** | Wants an AI-augmented desktop that automates repetitive workflows without babysitting eight different app-specific bots. |
| **Dr. Sam, the researcher** | Wants PyxisOS's Consensus Engine as a platform to run and publish experiments on. |
| **Jordan, the contributor** | Open-source dev or OS hobbyist who wants to contribute — either to the AI/shell layer now, or to kernel/hypervisor work later. |
| **(Later) IT/Ops teams** | Long-horizon: autonomous, auditable IT operations. Not a v1 target; noted for roadmap context. |

## 7. Success Metrics

**Track A**
- Consensus Engine answer quality vs. a single-agent baseline, on a fixed evaluation set (this is the single most important number for the whole project — see risk note in Section 10, Phase III).
- % of automation workflows completed without user correction; % of destructive actions correctly gated for approval.
- Contributor/user growth: GitHub stars/forks/issues, weekly active shell users.

**Track B**
- Kernel boots and reaches a shell prompt in QEMU; syscall/driver coverage; hypervisor isolation test pass rate; overhead vs. KVM/Xen baselines.

**Research**
- Measured reduction in hallucination/bias from multi-agent consensus vs. single-model baseline.
- Latency/cost overhead of consensus vs. single-agent response.

## 8. System Architecture

```
┌──────────────────────────────────────────────────────────────┐
│                   PyxisOS Shell — Track A                    │
│                                                              │
│      Stellar Canvas (Desktop/UX) ⇄ Orbital Intelligence      │
│                                  │                           │
│       Astral Consensus Engine (Multi-Agent Consensus)        │
│    Research · Security · Developer · Planner · Creative ·    │
│  Logic · Ethics · Statistical → Decision Arbitration Layer   │
│                                  │                           │
│            Celestial Automation (Workflow Engine)            │
│                                  │                           │
│           Constellation Network (Device Federation)          │
└──────────────────────────────────┬───────────────────────────┘
                                   │  syscalls / APIs
┌──────────────────────────────────▼───────────────────────────┐
│                        Host substrate                        │
│   v1 (Track A):  Linux kernel  +  KVM / Firecracker sandbox  │
│ v2+ (Track B): Lunar Core (custom kernel) + Aegis Hypervisor │
└──────────────────────────────────┬───────────────────────────┘
                       Physical Hardware
```

The Track A shell talks to the host through ordinary syscalls/APIs today, and migrates to native Lunar Core/Aegis interfaces once Track B matures — the Consensus Engine, Automation engine, and Shell code shouldn't need a rewrite to make that jump if the interface boundary is kept clean from day one.

## 9. Repository & Engineering Setup

The scaffold below now exists in the repo (previously just had a LICENSE file — Apache-2.0, good, that decision's already made):

```
PyxisOS/
├── README.md                    ← quick pitch + phase table + getting started
├── CONTRIBUTING.md              ← how to contribute (Track A vs Track B)
├── LICENSE                      (Apache-2.0, already in repo)
├── docs/
│   ├── PRD.md                   ← this document
│   ├── architecture/
│   └── research-paper/          ← paper drafts, benchmark write-ups
├── consensus/                   ← Astral Consensus Engine agents + Decision Arbitration Layer
├── automation/                  ← Celestial Automation workflow engine
├── shell/                       ← Stellar Canvas (Track A, built on Linux)
├── intelligence/                 ← Orbital Intelligence adaptive layer
├── nebula/                       ← Nebula Engine — spatial/3D computing
│   └── prototype/                  ← Three.js/WebXR starter (scaffolded, runnable today)
├── network/                      ← Constellation Network federation
├── native/                       ← Track B workspace: Lunar Core, Aegis
│   ├── lunar-core/
│   └── aegis/
├── research/                     ← eval harnesses, benchmark data
└── .github/
    └── ISSUE_TEMPLATE/            ← bug report, feature request, new-agent proposal
```

**Suggestions:**
- Issue labels by phase (`phase:lunar-core`, `phase:astral-consensus`, …) and by track (`track:A`, `track:B`) so contributors can self-select by interest and time commitment.
- Given `native/` will eventually need very different CI (QEMU boot tests, kernel-specific tooling) than `consensus/`/`shell/`/`nebula/` (standard app/web CI), keep them decoupled enough to split into a separate repo later without much pain — don't let them share build tooling more than necessary.
- A CI workflow (`.github/workflows/`) is deliberately not scaffolded yet — add it once `consensus/` or `shell/` has real code to lint/test against, rather than shipping an empty pipeline.

## 10. Phased Requirements

*Effort ranges below assume a small dedicated team (2–5 people); multiply by roughly 3–5x for a solo or part-time effort. They're deliberately ranges, not commitments — treat them as planning inputs, not deadlines.*

### 🌑 Phase I — New Moon: **Lunar Core** — *Track B*
**Objective:** Foundational kernel — bootstrapping, memory management, scheduling, interrupts, filesystem, terminal.

- **Requirements:** boots x86_64 via a custom or GRUB-compatible loader · paging/allocator supporting concurrent processes · preemptive scheduler · CPU exception + hardware IRQ handling (timer, keyboard minimum) · filesystem support (recommend read support for an existing format like FAT32/ext2 before inventing a custom one) · minimal kernel-level debug shell.
- **Acceptance criteria:** boots to a shell prompt in QEMU; runs a "hello world" userspace binary; smoke-test suite passes in CI on every merge.
- **Tech notes:** Rust is worth strong consideration for memory safety (see Redox OS, Theseus as precedent). The highest-leverage open decision here is **build vs. adopt**: a 100%-from-scratch kernel vs. building on an existing minimal microkernel (e.g., seL4) to de-risk the timeline. Flagged in Section 14 — don't default silently on this one.
- **Dependencies:** none — foundational, but *not* a blocker for Track A.
- **Effort:** a genuinely from-scratch minimal kernel at this scope is realistically **low tens of person-months**, even before broad hardware support. This is exactly why it's a parallel track, not a gate.
- **Risks:** driver complexity is where hobby kernels historically stall; keep the target surface (QEMU + a short hardware list) narrow on purpose.

### 🌒 Phase II — Waxing Crescent: **Stellar Canvas** — *Track A now, Track B later*
**Objective:** Custom graphical desktop environment — compositor, window manager, theming, widgets, animation.

- **Requirements:** compositor/window manager (Track A: Wayland/wlroots on Linux; Track B: native once Lunar Core has a GPU/framebuffer path) · theme engine (color/type/motion tokens) · widget toolkit for shell + third-party surfaces · compositor-level animation framework · developer/inspector mode · a shell/launcher built around Consensus Engine + Automation entry points rather than a traditional taskbar.
- **Acceptance criteria:** a running desktop session where a user launches apps, invokes the Consensus Engine via a global shortcut, and sees live automation status.
- **Tech notes:** wlroots + Rust/C for a "real OS shell" feel, or Tauri/Electron if raw prototyping speed matters more early on — worth deciding explicitly rather than defaulting.
- **Dependencies:** Track A version has none; native version depends on Phase I graphics support.
- **Effort:** a Track A prototype is **weeks to ~2 months**; a polished DE is a sustained, ongoing effort.
- **Risks:** compositor/driver bugs are notoriously time-consuming to debug; scope "polish" explicitly or it never ends.

### 🌓 Phase III — First Quarter: **Astral Consensus Engine** — *Track A*
**Objective:** Native multi-agent layer — specialized agents independently analyze requests, converge on a consensus response.

This is the heart of what makes PyxisOS different, and the fastest phase to get real evidence on — prioritize it.

- **Requirements:** a defined agent contract (input: request + context; output: opinion + confidence + rationale) · the eight named agents (Research, Security, Developer, Planner, Creative, Logic, Ethics, Statistical) implemented as pluggable experts — differently-prompted models, fine-tunes, or rules engines · a Decision Arbitration Layer (start with a simple "judge" pass that reviews all agent outputs and writes the final answer — fastest to ship and evaluate — then iterate toward voting/debate mechanisms) · a transparency UI showing each agent's independent opinion, not just the synthesis · a conflict-resolution policy for strong disagreement (escalate to user, flag low confidence) · a routing layer so routine requests don't invoke all eight agents (cost/latency control).
- **Acceptance criteria:** on a fixed benchmark set, the Consensus Engine produces synthesized answers with visible per-agent rationale, scored against a single-agent baseline on a defined rubric.
- **Tech notes:** the fastest path to v1 is API calls with distinct system prompts per agent persona rather than eight separate models — parallelize/cache calls to manage latency.
- **Dependencies:** none blocking — can be prototyped as a CLI before any shell integration exists.
- **Effort:** a 2–3-agent MVP with simple consensus is **achievable in weeks**; the full 8-agent system with UI is **a couple of months**.
- **Risks:** cost and latency of running multiple model calls per request; and, more fundamentally, **proving consensus actually beats a single well-prompted agent** — this needs a real evaluation harness, not intuition, and it's worth building that harness before writing much agent code.

### 🌔 Phase IV — Waxing Gibbous: **Celestial Automation** — *Track A*
**Objective:** Autonomous execution of long-running, multi-step workflows via cooperative agents.

- **Requirements:** a Master Agent/orchestrator decomposing goals into task graphs · a planner with explicit approval checkpoints for risky actions (deletion, network calls, purchases) · a task scheduler + async background executors · a persisted, resumable, inspectable workflow graph · long-term memory with user-visible view/delete controls · an event system for triggers (file changes, schedule, notifications).
- **Acceptance criteria:** a request like "organize my downloads folder by file type weekly" gets scheduled, executed, and reported on, with an approval gate before any destructive step.
- **Tech notes:** look to existing DAG-based orchestration patterns rather than inventing task-graph semantics from scratch.
- **Dependencies:** builds on Astral Consensus Engine (Phase III) for planning intelligence.
- **Effort:** a core scheduler+executor MVP is **a few weeks**; safe, trustworthy automation with good approval UX is **months of ongoing hardening**, not a one-time milestone.
- **Risks:** this is the highest-trust-risk surface in the whole project — an "autonomous" feature that takes real actions on real files/accounts. Default to confirm-before-destructive and full audit logging from the very first version, not as a later hardening pass.

### 🌕 Phase V — Full Moon: **Orbital Intelligence** — *Track A, ongoing*
**Objective:** Unify subsystems into an adaptive environment — layouts, resource allocation, and automation strategy adapt to the user, with oversight retained.

- **Requirements:** opt-in, local-first usage telemetry · adaptive layout *suggestions* (not silently auto-applied, at least initially) · predictive app/resource pre-loading · search spanning files, conversations, and automation history · shared context across Consensus Engine/Automation/Shell.
- **Acceptance criteria:** measurable reduction in time-to-task for repeat workflows in a user study.
- **Dependencies:** needs real usage data from Phases III/IV — sequence this *after* those mature, not in parallel from day one.
- **Effort:** genuinely ongoing/iterative rather than a single milestone.
- **Risks:** telemetry-driven personalization raises real privacy questions; local-first and opt-in by default, clearly disclosed.

### 🌖 Phase VI — Waning Gibbous: **Aegis Hypervisor** — *Track A stopgap now, Track B long-term*
**Objective:** Hypervisor-based isolation, secure execution, and defensive hardening.

- **Requirements:** isolated VM/container workloads · memory protection between guests · a sandboxing policy engine (what can an agent/automation task touch?) · secure boot · encrypted storage for images and user data · policy-based access control — nicely, the **Security agent from Astral Consensus Engine is a natural author/reviewer of these policies**, worth designing for explicitly.
- **Acceptance criteria:** passes a defined isolation test suite (no cross-VM memory leakage; sandboxed processes can't escape declared policy).
- **Tech notes:** strongly recommend Track A's actual sandboxing need (containing what an autonomous agent can do) rides on **KVM/Firecracker** — battle-tested, used in production elsewhere — rather than a hand-rolled hypervisor. Reserve a genuinely custom Type-1 hypervisor for Track B, once Lunar Core exists and dedicated security engineering is available.
- **Dependencies:** native version depends on Lunar Core; the Track A sandboxing baseline doesn't, and should ship much earlier.
- **Effort:** a custom Type-1 hypervisor is arguably the single hardest item on this entire roadmap — realistically a multi-person-year, specialist effort that needs external security review before it ever touches real user data.
- **Risks:** isolation bugs are severe-by-definition; don't self-certify this one — budget for outside review.

### 🌗 Phase VII — Last Quarter: **Nebula Engine** — *Track A prototype, Track B native*
**Objective:** Immersive spatial computing — 3D desktop, spatial windows, VR/AR support.

**Decision made:** prototype on **Three.js + WebXR** (Track A) rather than a custom renderer, for the same reason the rest of Track A rides on proven components — it's the fastest way to get a real, demoable spatial desktop in front of people. **Bevy (Rust)** is the recommended Track B target once the desktop metaphor is validated and Lunar Core has a graphics story — its ECS model fits "windows as entities" well and keeps the stack Rust-consistent with the kernel, but its VR/XR tooling is less mature than Three.js/WebXR today, which is exactly why it's the *later* native step, not the first prototype.

- **Requirements:** 3D scene graph representing windows as spatial objects · GPU-accelerated rendering (WebGL2 for the prototype) · physics-based window interaction (inertia, snapping) · gesture/controller navigation · WebXR session support (VR/AR) · multiple fast-switching spatial workspaces.
- **v0.1 milestone (scoped small on purpose):** a scene with several floating "windows" (textured panels) arranged in space, orbit-style camera control on desktop, and a working "Enter VR" button via the WebXR Device API. **This now exists** at `nebula/prototype/index.html` — open it in a browser to see the starting point (WebXR needs HTTPS or `localhost`, and a connected headset to actually enter VR; the desktop orbit/zoom view works anywhere).
- **Acceptance criteria:** the prototype holds a comfortable frame rate for VR (commonly-cited comfort thresholds start around 72fps+) with multiple floating windows, and a controller/gesture can select and move a window in 3D space (not yet in v0.1 — camera navigation only so far).
- **Tech notes:** a `THREE.Group` per window is a reasonable starting abstraction, with real app content rendered to a `CanvasTexture` or an `<iframe>`-in-3D approach later. The iframe approach reopens the sandboxing questions from Phase VI (an arbitrary app rendering inside a 3D window still needs the same permission boundaries as a 2D one) — worth treating as a security question, not just a rendering one, before it's built. The v0.1 prototype deliberately hand-rolls minimal orbit-controls and VR-button logic directly against the WebXR API instead of pulling in the standard three.js addon modules, so it stays a single dependency-free file — swap in the real `OrbitControls`/`VRButton` addons once the project has an actual build step.
- **Dependencies:** benefits from Stellar Canvas's design system for consistent theming; otherwise independent and runnable today.
- **Effort:** the scaffolded v0.1 is real but intentionally minimal — expect **weeks, not months**, to get from it to something worth demoing publicly; a comfortable, daily-usable VR desktop remains a long, iterative UX research effort regardless of engine choice.
- **Risks:** VR comfort/motion sickness is a genuinely hard UX problem — keep this opt-in and experimental until user-tested; rendering arbitrary app content inside a 3D window is a security question (see Phase VI) as much as a graphics one.

### 🌘 Phase VIII — Waning Crescent: **Constellation Network** — *Track A, grows over time*
**Objective:** Multi-device federation — distributed agents, synchronized state, secure decentralized communication.

- **Requirements:** device discovery/pairing (local network + relay) · end-to-end encrypted state/preference/automation sync · distributed agent delegation (offload heavy compute to a more capable paired device) · local-inference fallback when offline.
- **Acceptance criteria:** two PyxisOS Shell instances on separate machines share automation state and can hand a running task from one to the other.
- **Tech notes:** CRDT-based sync (e.g., Automerge/Yjs-style approaches) is a much safer starting point than inventing a new distributed consensus protocol.
- **Dependencies:** benefits from mature Automation (Phase IV); doesn't require the custom kernel, so it can grow incrementally on Track A.
- **Effort:** basic two-device sync is **a few weeks**; robust mesh federation with offline/edge support is **months, ongoing**.
- **Risks:** sync/partition bugs are subtle — budget for dedicated chaos/partition testing, not just happy-path tests.

## 11. Cross-Cutting Requirements

- **Security & privacy:** a permission model for what agents/automation can touch by default, full audit logging, clear data retention/deletion controls, opt-in telemetry only.
- **Observability:** structured logs/traces for agent decisions and automation runs — this doubles as both a debugging tool and the transparency data the research paper needs.
- **Testing:** unit tests per module; consensus stability tests for the Consensus Engine against a fixed prompt set; QEMU-based boot/regression CI once `native/` has content; fuzz testing anywhere untrusted input is parsed (filesystem, network) ahead of Aegis.
- **Documentation:** architecture docs, a contributor guide, an "adding a new Consensus agent" guide, and ADRs (architecture decision records) for the big forks-in-the-road like build-vs-adopt on the kernel.
- **Accessibility:** keyboard navigation and screen-reader support for Stellar Canvas, called out explicitly so it doesn't get dropped under deadline pressure.

## 12. Risks & Mitigations (Project-Level)

| Risk | Mitigation |
|---|---|
| Scope collapse ("boil the ocean") | Two-track strategy + explicit non-goals (Section 4). |
| Solo/small-team burnout on a multi-year systems effort | Ship Track A wins early — sustains motivation and attracts contributors. |
| LLM cost/latency for the Consensus Engine | Agent routing (not all 8 every time), caching, cheaper models for low-stakes agents. |
| Security incidents from autonomous automation | Default confirm-before-destructive, full audit log, graduate to auto-execute only after a track record on low-risk actions. |
| Steep onboarding curve for kernel/hypervisor work | Strong docs, "good first issue" labels, consider a separate community channel for Track B specialists. |
| Naming/trademark | Worth a quick trademark/namespace check on "Pyxis"/"PyxisOS" before any wider public launch — not urgent now. |

## 13. Illustrative Timeline

*Ranges assume a small dedicated team (2–5 contributors). Solo/part-time: multiply by 3–5x. A funded team with systems specialists could compress Track B significantly — this table is a planning aid, not a commitment.*

| Milestone | Track | Rough timeframe |
|---|---|---|
| Astral Consensus Engine MVP (2–3 agents, CLI) | A | Month 1–2 |
| Aegis sandboxing via KVM/Firecracker | A | Month 2–4 |
| Stellar Canvas prototype shell (Linux-based) | A | Month 2–4 |
| Celestial Automation MVP w/ approval gates | A | Month 3–5 |
| Public v0.1 "PyxisOS Shell" release | A | Month 5–6 |
| Orbital Intelligence personalization v1 | A | Month 6–9 |
| Constellation Network (2-device sync) | A | Month 7–10 |
| Nebula Engine spatial prototype (engine-based) | A | Month 8–12 |
| Lunar Core: minimal kernel boots in QEMU | B (parallel) | Month 6–18+ |
| Aegis: full custom Type-1 hypervisor | B | 18+ months, post-Lunar Core, security-reviewed |
| Native Nebula / Stellar Canvas on Lunar Core | B | Post-Lunar Core, TBD |

## 14. Open Decisions

These are the things worth resolving deliberately rather than by default:

1. **Build vs. adopt for Lunar Core** — fully from scratch, or build on an existing minimal kernel (e.g., seL4) to de-risk the timeline?
2. **Is Track B an active, staffed goal for v1, or a stated long-term north star** mentioned in docs but not yet resourced? (This changes almost everything else about sequencing.)
3. **Primary near-term objective** — research paper, usable product, portfolio/learning project, or genuinely all three? Affects what "done" means for Phase III especially.
4. **Team model** — solo, open community, or a funded effort? Directly rescales every estimate in Section 13.
5. **LLM backend for Consensus agents** — hosted API (fast to start, ongoing cost/dependency) vs. local/open-weight models (more autonomous and private, more setup work)?

## 15. Appendix A — Codename Glossary

| Codename | Phase | Track | One-liner |
|---|---|---|---|
| Lunar Core | I | B | The kernel |
| Stellar Canvas | II | A → B | The desktop environment |
| Astral Consensus Engine | III | A | Multi-agent consensus |
| Celestial Automation | IV | A | Autonomous workflows |
| Orbital Intelligence | V | A | Adaptive, learning UX |
| Aegis Hypervisor | VI | A (stopgap) → B | Isolation & security |
| Nebula Engine | VII | A (prototype) → B | Spatial/3D computing |
| Constellation Network | VIII | A | Multi-device federation |

## 16. Appendix B — Mapping to the Research Paper Outline

The original vision doc's proposed paper structure (Intro → Problem Statement → Architecture → Multi-Agent Consensus Framework → Autonomous Workflow Engine → Hypervisor Security → Adaptive UX → Experimental Evaluation → Future Work → Conclusion) lines up directly with Sections 2–3, 8, and 10 (Phases III/IV/VI/VII) of this PRD. Practically: the evaluation harness recommended in Phase III's risk note *is* the "Experimental Evaluation" section of the paper — building it early serves both the product and the publication.
