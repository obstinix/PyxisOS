//! Capability-based security model for Aegis.
//!
//! Replaces ambient authority with unforgeable, fine-grained capability tokens
//! governing access to memory, hardware ports, devices, and system IPC channels.

/// Bitflags representing permissions granted by a capability token.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CapabilityRights(pub u32);

impl CapabilityRights {
    pub const NONE: Self = Self(0);
    pub const READ: Self = Self(1 << 0);
    pub const WRITE: Self = Self(1 << 1);
    pub const EXECUTE: Self = Self(1 << 2);
    pub const MAP_PHYSICAL: Self = Self(1 << 3);
    pub const DEVICE_IO: Self = Self(1 << 4);
    pub const INTERRUPT_CONTROL: Self = Self(1 << 5);
    pub const TASK_SPAWN: Self = Self(1 << 6);
    pub const DELEGATE: Self = Self(1 << 7);
    pub const ALL: Self = Self(0xFF);

    pub const fn contains(&self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }

    pub const fn union(&self, other: Self) -> Self {
        Self(self.0 | other.0)
    }

    pub const fn intersect(&self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}

/// Unforgeable token granting explicit rights to an owner.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CapabilityToken {
    pub id: u64,
    pub owner_id: String,
    pub resource: String,
    pub rights: CapabilityRights,
    pub revocable: bool,
}

impl CapabilityToken {
    pub fn new(
        id: u64,
        owner_id: impl Into<String>,
        resource: impl Into<String>,
        rights: CapabilityRights,
        revocable: bool,
    ) -> Self {
        Self {
            id,
            owner_id: owner_id.into(),
            resource: resource.into(),
            rights,
            revocable,
        }
    }

    /// Derives a sub-capability token with equal or reduced rights.
    pub fn delegate(
        &self,
        new_id: u64,
        new_owner: impl Into<String>,
        reduced_rights: CapabilityRights,
    ) -> Result<Self, &'static str> {
        if !self.rights.contains(CapabilityRights::DELEGATE) {
            return Err("Capability does not permit delegation");
        }
        if !self.rights.contains(reduced_rights) {
            return Err("Cannot delegate rights exceeding parent capability");
        }

        Ok(Self {
            id: new_id,
            owner_id: new_owner.into(),
            resource: self.resource.clone(),
            rights: reduced_rights,
            revocable: true,
        })
    }
}

/// Local capability space tracking tokens held by a context.
#[derive(Debug, Default)]
pub struct CapabilitySpace {
    tokens: Vec<CapabilityToken>,
}

impl CapabilitySpace {
    pub fn new() -> Self {
        Self { tokens: Vec::new() }
    }

    pub fn insert(&mut self, token: CapabilityToken) {
        self.tokens.push(token);
    }

    pub fn revoke(&mut self, token_id: u64) -> bool {
        if let Some(pos) = self.tokens.iter().position(|t| t.id == token_id) {
            if self.tokens[pos].revocable {
                self.tokens.remove(pos);
                return true;
            }
        }
        false
    }

    pub fn has_permission(&self, resource: &str, required_right: CapabilityRights) -> bool {
        self.tokens
            .iter()
            .any(|t| t.resource == resource && t.rights.contains(required_right))
    }

    pub fn token_count(&self) -> usize {
        self.tokens.len()
    }
}
