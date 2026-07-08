# native/ — PyxisOS Native (Track B)

The from-scratch systems track: a custom kernel, hypervisor, and (eventually) native spatial engine. This is long-horizon systems-research work, most useful for contributors with Rust/C and OS-internals background.

- `lunar-core/` — Phase I, the kernel.
- `aegis/` — Phase VI, the hypervisor. Note: Track A's actual sandboxing need is served by KVM/Firecracker, not this folder — this is the from-scratch, security-reviewed, long-term version.

See `docs/PRD.md` Section 5 for why this is a separate track from everything else in the repo, and Section 14, Open Decision #1, for the build-vs-adopt question this track should resolve early.
