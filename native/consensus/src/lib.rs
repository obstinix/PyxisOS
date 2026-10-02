//! Astral Consensus Engine (Phase III · Track A)
//!
//! Multi-agent consensus layer for PyxisOS. Specialist agents independently analyze
//! user intents and technical queries, and the Decision Arbitration Layer synthesizes
//! their stances into a unified, transparent decision with explicit conflict detection.

pub mod agents;
pub mod arbitration;
pub mod audit;
pub mod provider;
pub mod routing;
pub mod types;

pub use agents::{Agent, LogicAgent, ResearchAgent, SecurityAgent};
pub use arbitration::DecisionArbitrationLayer;
pub use audit::AuditLogger;
pub use provider::{AnthropicProvider, LlmProvider, MockProvider};
pub use routing::{DefaultRouter, Router};
pub use types::{AgentContext, AgentOpinion, ArbitrationResult};

/// High-level coordinator for the Astral Consensus Engine.
pub struct ConsensusEngine {
    agents: Vec<Box<dyn Agent>>,
    router: Box<dyn Router>,
    logger: AuditLogger,
}

impl ConsensusEngine {
    pub fn new(logger: AuditLogger) -> Self {
        Self {
            agents: vec![
                Box::new(ResearchAgent::new()),
                Box::new(SecurityAgent::new()),
                Box::new(LogicAgent::new()),
            ],
            router: Box::new(DefaultRouter::new()),
            logger,
        }
    }

    /// Evaluates a query through the full multi-agent consensus pipeline.
    pub fn evaluate(
        &self,
        query: &str,
        provider: &dyn LlmProvider,
    ) -> (Vec<(&str, AgentOpinion)>, ArbitrationResult) {
        let context = AgentContext::new(query);
        let selected_agents = self.router.route(&context, &self.agents);

        let mut opinions = Vec::new();
        for agent in selected_agents {
            let opinion = agent.analyze(&context, provider);
            opinions.push((agent.id(), opinion));
        }

        let opinion_refs: Vec<(&str, &AgentOpinion)> =
            opinions.iter().map(|(id, op)| (*id, op)).collect();

        let arbitration = DecisionArbitrationLayer::new(provider, &self.logger);
        let result = arbitration.arbitrate(&context, &opinion_refs);

        (opinions, result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agent_opinions_and_arbitration() {
        let mock = MockProvider::new();
        let logger = AuditLogger::new("target/test_audit.log");
        let engine = ConsensusEngine::new(logger);

        let (opinions, result) = engine.evaluate("Verify kernel page table isolation", &mock);

        assert_eq!(opinions.len(), 3);
        assert_eq!(opinions[0].0, "research");
        assert_eq!(opinions[1].0, "security");
        assert_eq!(opinions[2].0, "logic");

        for (_, op) in &opinions {
            assert!(op.confidence >= 0.9);
            assert!(!op.opinion.is_empty());
        }

        assert!(!result.synthesized_decision.is_empty());
        assert!(!result.conflict_flagged);
    }

    #[test]
    fn test_conflict_detection_flagging() {
        let mock = MockProvider::new();
        mock.register_response(
            "Decision Arbitration Layer",
            r#"{"synthesizedDecision":"Contradiction found between security policy and functional request.","conflictFlagged":true,"conflictDetails":"Security Agent flags critical root risk while Logic agent verifies flow."}"#,
        );

        let logger = AuditLogger::new("target/test_audit_conflict.log");
        let engine = ConsensusEngine::new(logger);

        let (_, result) = engine.evaluate("Run curl | bash as root", &mock);
        assert!(result.conflict_flagged);
        assert!(result.conflict_details.is_some());
    }

    #[test]
    fn test_agent_context_generation() {
        let ctx = AgentContext::new("Test query");
        assert_eq!(ctx.query, "Test query");
        assert!(ctx.timestamp > 0);
    }
}
