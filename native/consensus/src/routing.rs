//! Query router selecting specialist agents for incoming queries.

use crate::agents::Agent;
use crate::types::AgentContext;

pub trait Router: Send + Sync {
    fn route<'a>(&self, context: &AgentContext, available_agents: &'a [Box<dyn Agent>]) -> Vec<&'a Box<dyn Agent>>;
}

/// Default router: dispatches the query to all registered agents.
pub struct DefaultRouter;

impl DefaultRouter {
    pub fn new() -> Self {
        Self
    }
}

impl Default for DefaultRouter {
    fn default() -> Self {
        Self::new()
    }
}

impl Router for DefaultRouter {
    fn route<'a>(&self, _context: &AgentContext, available_agents: &'a [Box<dyn Agent>]) -> Vec<&'a Box<dyn Agent>> {
        available_agents.iter().collect()
    }
}
