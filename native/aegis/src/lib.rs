//! Aegis Capability Isolation and Security Architecture (Phase 4 · Track B)
//!
//! Aegis provides capability tokens, execution sandboxing, and isolation boundaries
//! to protect kernel state and physical memory from untrusted autonomous agent workloads.

pub mod capability;
pub mod isolation;
pub mod sandbox;

pub use capability::{CapabilityRights, CapabilitySpace, CapabilityToken};
pub use isolation::{HardwareVirtualizationInterface, IsolationBoundary, SoftwareIsolation};
pub use sandbox::{SandboxConfig, SandboxContext, SandboxState};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_capability_rights_masking() {
        let r1 = CapabilityRights::READ.union(CapabilityRights::WRITE);
        assert!(r1.contains(CapabilityRights::READ));
        assert!(r1.contains(CapabilityRights::WRITE));
        assert!(!r1.contains(CapabilityRights::EXECUTE));

        let r2 = r1.intersect(CapabilityRights::READ);
        assert_eq!(r2, CapabilityRights::READ);
    }

    #[test]
    fn test_capability_delegation_and_revocation() {
        let mut space = CapabilitySpace::new();
        let master = CapabilityToken::new(
            1,
            "security-daemon",
            "/dev/mem",
            CapabilityRights::ALL,
            true,
        );

        let delegated = master.delegate(
            2,
            "agent-worker",
            CapabilityRights::READ.union(CapabilityRights::MAP_PHYSICAL),
        ).expect("Delegation should succeed");

        assert_eq!(delegated.owner_id, "agent-worker");
        assert!(delegated.rights.contains(CapabilityRights::READ));
        assert!(!delegated.rights.contains(CapabilityRights::WRITE));

        space.insert(delegated);
        assert_eq!(space.token_count(), 1);
        assert!(space.has_permission("/dev/mem", CapabilityRights::READ));
        assert!(!space.has_permission("/dev/mem", CapabilityRights::WRITE));

        assert!(space.revoke(2));
        assert_eq!(space.token_count(), 0);
    }

    #[test]
    fn test_sandbox_lifecycle_and_memory_limit() {
        let config = SandboxConfig::new("agent-sandbox-01", 1024 * 1024); // 1MB
        let mut sandbox = SandboxContext::new(100, config);

        assert_eq!(sandbox.state, SandboxState::Created);
        assert!(sandbox.start().is_ok());
        assert_eq!(sandbox.state, SandboxState::Running);

        assert!(sandbox.allocate_memory(512 * 1024).is_ok());
        assert_eq!(sandbox.allocated_memory_bytes, 512 * 1024);

        // Exceed limit
        assert!(sandbox.allocate_memory(600 * 1024).is_err());

        sandbox.release_memory(256 * 1024);
        assert_eq!(sandbox.allocated_memory_bytes, 256 * 1024);

        sandbox.terminate();
        assert_eq!(sandbox.state, SandboxState::Terminated);
    }

    #[test]
    fn test_isolation_boundary_enforcement() {
        let config = SandboxConfig::new("eval-sandbox", 2 * 1024 * 1024);
        let mut sandbox = SandboxContext::new(101, config);
        let isolation = SoftwareIsolation::new();

        // Not running yet -> access denied
        assert!(!isolation.verify_resource_access(&sandbox, "/sys/kernel", CapabilityRights::READ));

        sandbox.start().unwrap();

        // No capability token -> access denied
        assert!(!isolation.verify_resource_access(&sandbox, "/sys/kernel", CapabilityRights::READ));

        // Grant read token
        let token = CapabilityToken::new(1, "eval", "/sys/kernel", CapabilityRights::READ, false);
        sandbox.capability_space.insert(token);

        // Read permitted, write denied
        assert!(isolation.verify_resource_access(&sandbox, "/sys/kernel", CapabilityRights::READ));
        assert!(!isolation.verify_resource_access(&sandbox, "/sys/kernel", CapabilityRights::WRITE));
    }
}
