# native/aegis/ — Aegis Hypervisor

**Phase VI · Track B**

Custom Type-1 hypervisor: VM isolation, memory protection, secure boot, encrypted storage, policy-based access control (natural integration point: the Security agent from `consensus/` as policy author/reviewer).

**Status:** scaffolded — see [`native/Cargo.toml`](../Cargo.toml). Not started — and shouldn't be, until `native/lunar-core/` exists and dedicated security engineering is available. This is arguably the single hardest item on the whole roadmap; budget for external security review before it ever touches real user data. Track A's near-term sandboxing need is met by KVM/Firecracker, not this folder.
