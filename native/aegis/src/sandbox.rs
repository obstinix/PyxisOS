//! Execution sandbox context and lifecycle management for Aegis.

use crate::capability::CapabilitySpace;

/// Operational lifecycle state of an execution sandbox.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SandboxState {
    Created,
    Running,
    Suspended,
    Terminated,
}

/// Configuration parameters defining sandbox limits.
#[derive(Debug, Clone)]
pub struct SandboxConfig {
    pub name: String,
    pub memory_limit_bytes: usize,
    pub max_active_channels: usize,
}

impl SandboxConfig {
    pub fn new(name: impl Into<String>, memory_limit_bytes: usize) -> Self {
        Self {
            name: name.into(),
            memory_limit_bytes,
            max_active_channels: 16,
        }
    }
}

/// Isolated execution sandbox context containing memory bounds and capability space.
pub struct SandboxContext {
    pub id: u64,
    pub config: SandboxConfig,
    pub state: SandboxState,
    pub capability_space: CapabilitySpace,
    pub allocated_memory_bytes: usize,
}

impl SandboxContext {
    pub fn new(id: u64, config: SandboxConfig) -> Self {
        Self {
            id,
            config,
            state: SandboxState::Created,
            capability_space: CapabilitySpace::new(),
            allocated_memory_bytes: 0,
        }
    }

    pub fn start(&mut self) -> Result<(), &'static str> {
        if self.state != SandboxState::Created && self.state != SandboxState::Suspended {
            return Err("Cannot start sandbox in its current state");
        }
        self.state = SandboxState::Running;
        Ok(())
    }

    pub fn suspend(&mut self) -> Result<(), &'static str> {
        if self.state != SandboxState::Running {
            return Err("Cannot suspend a sandbox that is not running");
        }
        self.state = SandboxState::Suspended;
        Ok(())
    }

    pub fn terminate(&mut self) {
        self.state = SandboxState::Terminated;
    }

    pub fn allocate_memory(&mut self, bytes: usize) -> Result<(), &'static str> {
        if self.allocated_memory_bytes + bytes > self.config.memory_limit_bytes {
            return Err("Sandbox memory quota exceeded");
        }
        self.allocated_memory_bytes += bytes;
        Ok(())
    }

    pub fn release_memory(&mut self, bytes: usize) {
        self.allocated_memory_bytes = self.allocated_memory_bytes.saturating_sub(bytes);
    }
}
