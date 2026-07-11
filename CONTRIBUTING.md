# Contributing to PyxisOS

Thanks for your interest — PyxisOS is early-stage, which means there's real room to shape it. This doc covers how contributions are organized and what to expect.

## Two tracks, two kinds of contributions

PyxisOS splits work into two tracks (full reasoning in [`docs/PRD.md`](docs/PRD.md), Section 5):

- **Track A — PyxisOS Shell** (`consensus/`, `automation/`, `shell/`, `intelligence/`, `nebula/`, `network/`): the Astral Consensus Engine, automation engine, desktop shell, and spatial prototype, built on Linux/KVM. No OS-development background required — this is mostly application-level work (agent design/prompting, orchestration logic, frontend, 3D/graphics).
- **Track B — PyxisOS Native** (`native/lunar-core/`, `native/aegis/`, and eventually a native `nebula/`/`shell/`): the from-scratch kernel, hypervisor, and native spatial engine. Systems programming background (Rust/C, OS internals) genuinely helps here — this is long-horizon, harder-to-parallelize work.

Both are welcome and both are real engineering. Pick based on interest and background, not "seniority" — Track A is not "the easy track," it's a differently scoped one, and it's where the project's actual novelty is being proven out.

## Finding something to work on

- Issues are labeled by phase (`phase:astral-consensus`, `phase:lunar-core`, etc.) and by track (`track:A`, `track:B`).
- `good-first-issue` marks a reasonable entry point.
- Each module folder's `README.md` states its status and links to its section of the PRD — start there before opening an issue to check it's not already scoped differently than you expect.
- Nothing fits? Open an issue proposing what you'd like to build. Proposing a new Astral Consensus Engine agent has its own issue template.

## Workflow

1. Fork the repo, branch off `main`: `track-a/<short-description>` or `track-b/<short-description>`.
2. Keep PRs scoped to one phase/module where possible — easier to review, easier to revert.
3. Reference the issue you're addressing in the PR description.
4. CI (once present for a given module — see note in `docs/PRD.md` Section 9) must pass before merge.
5. Anything touching `native/aegis/` (the hypervisor) or a sandboxing/policy engine: flag the PR for security-focused review explicitly. Isolation bugs are treated as high-severity by default, not case-by-case.

## Code style

- **Rust** (Track B, and any Track A Rust components): `rustfmt` + `clippy`, no warnings on merge.
- **JS/TS** (Track A components: `consensus/`, `automation/`, `shell/`, `nebula/prototype/`, and other web/orchestration logic): `prettier` + `eslint`.
- A formal style guide will land in `docs/` as the codebase grows. Until then, match the surrounding code.

## Proposing changes to the PRD itself

`docs/PRD.md` is a living document. Open a PR directly against it for small clarifications. Open an issue first for anything that changes scope, sequencing, or the two-track strategy, so it can be discussed before the doc changes underneath anyone mid-implementation.

## Testing expectations

- **Track A:** unit tests per module. Any change to `consensus/`'s agents or consensus logic should run against the evaluation set in `research/` before merging — this protects against silently regressing consensus quality, which is the single most important property of the whole Council concept.
- **Track B:** changes to `native/lunar-core/` need to boot clean in the QEMU CI job (once it exists); changes to `native/aegis/` need to pass the isolation test suite before merge, no exceptions.

## Security disclosures

If you find an isolation, sandboxing, or privilege-escalation issue — especially in `native/aegis/` or the automation approval-gate logic in `automation/` — please open a private security advisory on the repo rather than a public issue. A dedicated [SECURITY.md](file:///c:/ProjectsPP/Pyxis/files/PyxisOS-repo-scaffold/PyxisOS/SECURITY.md) contains our security policy and disclosure process. Please refer to it before reporting any vulnerability.

## Code of conduct

Not yet formally adopted — the [Contributor Covenant](https://www.contributor-covenant.org/) is a reasonable default. Open an issue if you want to help drive this.

## Questions

Open an issue with the `question` label, or check [`docs/PRD.md`](docs/PRD.md) Section 14 (Open Decisions) first — your question might already be a known open one.
