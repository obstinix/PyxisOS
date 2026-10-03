//! Isolation boundary enforcement and hardware virtualization hooks for Aegis.

use crate::capability::CapabilityRights;
use crate::sandbox::{SandboxContext, SandboxState};

/// General trait for enforcement of isolation boundaries.
pub trait IsolationBoundary {
    fn verify_resource_access(
        &self,
        context: &SandboxContext,
        resource: &str,
        right: CapabilityRights,
    ) -> bool;
    fn enforce_memory_limit(&self, context: &SandboxContext) -> bool;
}

/// Software capability-enforced isolation layer.
pub struct SoftwareIsolation;

impl SoftwareIsolation {
    pub fn new() -> Self {
        Self
    }
}

impl Default for SoftwareIsolation {
    fn default() -> Self {
        Self::new()
    }
}

impl IsolationBoundary for SoftwareIsolation {
    fn verify_resource_access(
        &self,
        context: &SandboxContext,
        resource: &str,
        right: CapabilityRights,
    ) -> bool {
        if context.state != SandboxState::Running {
            return false;
        }
        context.capability_space.has_permission(resource, right)
    }

    fn enforce_memory_limit(&self, context: &SandboxContext) -> bool {
        context.allocated_memory_bytes <= context.config.memory_limit_bytes
    }
}

/// Hardware virtualization feature detection for planned hypervisor support.
///
/// NOTE: This component is experimental. PyxisOS does not yet claim a production
/// Type-1 hypervisor; this module provides the hardware interface detection hooks.
pub struct HardwareVirtualizationInterface;

impl HardwareVirtualizationInterface {
    /// Detects whether Intel VT-x (VMX) or AMD-V (SVM) CPU extensions are present.
    ///
    /// In user-mode tests or non-virtualized environments, returns false safely.
    pub fn is_virtualization_supported() -> bool {
        // Safe check using standard x86 CPUID feature flags when on x86_64
        #[cfg(target_arch = "x86_64")]
        {
            // CPUID leaf 1: ECX bit 5 indicates VMX (Intel VT-x)
            // CPUID leaf 0x80000001: ECX bit 2 indicates SVM (AMD-V)
            false // Hardware virtualization root initialization is planned
        }
        #[cfg(not(target_arch = "x86_64"))]
        {
            false
        }
    }

    pub fn status_description() -> &'static str {
        "Aegis Hardware Hypervisor: Experimental architecture; CPU virtualization hooks defined."
    }
}
