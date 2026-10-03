//! Astral Consensus Engine (Phase III · Track A)
//!
//! Multi-agent consensus layer for PyxisOS. Specialist agents independently analyze
//! user intents and technical queries, and the Decision Arbitration Layer synthesizes
//! their stances into a unified, transparent decision with explicit conflict detection.

pub mod agents;
pub mod arbitration;
pub mod audit;
pub mod deterministic;
pub mod provider;
pub mod routing;
pub mod types;

pub use agents::{Agent, LogicAgent, ResearchAgent, SecurityAgent};
pub use arbitration::DecisionArbitrationLayer;
pub use audit::AuditLogger;
pub use deterministic::{
    DeterministicAgent, DeterministicArbitrator, DeterministicLogicAgent,
    DeterministicResourceAgent, DeterministicSecurityAgent,
};
pub use provider::{AnthropicProvider, LlmProvider, MockProvider};
pub use routing::{DefaultRouter, Router};
pub use types::{
    AgentContext, AgentOpinion, ArbitrationResult, ConsensusResult, Decision, DecisionOutcome,
    Evidence, EvidenceCategory, Proposal, ProposalPriority, SubsystemTarget, Vote, VoteStance,
};

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

    #[test]
    fn test_deterministic_safe_proposal_acceptance() {
        let arbitrator = DeterministicArbitrator::new();
        let mut arbitrator = arbitrator;
        arbitrator.register_agent(Box::new(DeterministicSecurityAgent::new()));
        arbitrator.register_agent(Box::new(DeterministicResourceAgent::new()));
        arbitrator.register_agent(Box::new(DeterministicLogicAgent::new()));

        let proposal = Proposal::new(
            "prop-001",
            "kernel-optimizer",
            SubsystemTarget::Memory,
            "compact_heap_arenas",
            50,
            ProposalPriority::Normal,
        ).with_param("target_arena", "kernel_pool");

        let decision = arbitrator.arbitrate_proposal(&proposal, &[]);
        assert_eq!(decision.outcome, DecisionOutcome::Accepted);
        assert!(decision.score >= 0.70);
        assert!(!decision.conflict_flagged);
        assert_eq!(decision.votes_approve, 3);
        assert_eq!(decision.votes_reject, 0);
    }

    #[test]
    fn test_deterministic_security_veto_unrestricted_root() {
        let arbitrator = DeterministicArbitrator::with_default_agents();

        let proposal = Proposal::new(
            "prop-002",
            "compromised-subagent",
            SubsystemTarget::Security,
            "escalate_unrestricted_root_privilege",
            10,
            ProposalPriority::High,
        );

        let decision = arbitrator.arbitrate_proposal(&proposal, &[]);
        assert_eq!(decision.outcome, DecisionOutcome::Rejected);
        assert!(decision.votes_reject >= 1);
    }

    #[test]
    fn test_deterministic_resource_constraint_handling() {
        let arbitrator = DeterministicArbitrator::with_default_agents();

        let proposal = Proposal::new(
            "prop-003",
            "heavy-workload",
            SubsystemTarget::Memory,
            "allocate_buffer",
            50,
            ProposalPriority::Low,
        );

        let evidence = vec![Evidence::new(
            "ev-001",
            "prop-003",
            "telemetry-daemon",
            EvidenceCategory::ResourceConstraint,
            "RAM utilization above 92%",
            0.88,
        )];

        let decision = arbitrator.arbitrate_proposal(&proposal, &evidence);
        // Under resource pressure on Low priority, ResourceAgent returns ConditionalApproval
        assert_eq!(decision.outcome, DecisionOutcome::Accepted);
        assert!(decision.score > 0.0);
    }

    #[test]
    fn test_deterministic_batch_arbitration() {
        let arbitrator = DeterministicArbitrator::with_default_agents();

        let p1 = Proposal::new(
            "p1",
            "agent-a",
            SubsystemTarget::Memory,
            "defrag",
            50,
            ProposalPriority::Normal,
        );
        let p2 = Proposal::new(
            "p2",
            "agent-b",
            SubsystemTarget::Kernel,
            "raw_io_bypass",
            0,
            ProposalPriority::Critical,
        );

        let result = arbitrator.arbitrate_batch("batch-01", &[p1, p2], &[]);
        assert_eq!(result.proposals.len(), 2);
        assert_eq!(result.decisions.len(), 2);
        assert_eq!(result.decisions[0].outcome, DecisionOutcome::Accepted);
        assert_eq!(result.decisions[1].outcome, DecisionOutcome::Rejected);
    }
}
