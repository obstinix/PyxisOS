//! Core data types for the Astral Consensus Engine.

/// Execution context for an agent query.
#[derive(Debug, Clone)]
pub struct AgentContext {
    pub query: String,
    pub timestamp: u64,
}

impl AgentContext {
    pub fn new(query: impl Into<String>) -> Self {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_millis() as u64)
            .unwrap_or(0);
        Self {
            query: query.into(),
            timestamp,
        }
    }
}

/// An individual opinion returned by a specialist agent.
#[derive(Debug, Clone, PartialEq)]
pub struct AgentOpinion {
    pub opinion: String,
    pub confidence: f64,
    pub rationale: String,
}

impl AgentOpinion {
    pub fn new(opinion: impl Into<String>, confidence: f64, rationale: impl Into<String>) -> Self {
        Self {
            opinion: opinion.into(),
            confidence: confidence.clamp(0.0, 1.0),
            rationale: rationale.into(),
        }
    }
}

/// The synthesized decision returned by the arbitration layer.
#[derive(Debug, Clone, PartialEq)]
pub struct ArbitrationResult {
    pub synthesized_decision: String,
    pub conflict_flagged: bool,
    pub conflict_details: Option<String>,
}

impl ArbitrationResult {
    pub fn new(
        synthesized_decision: impl Into<String>,
        conflict_flagged: bool,
        conflict_details: Option<String>,
    ) -> Self {
        Self {
            synthesized_decision: synthesized_decision.into(),
            conflict_flagged,
            conflict_details,
        }
    }
}

/// Target operating system subsystem for a proposal.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SubsystemTarget {
    Kernel,
    Memory,
    Security,
    Storage,
    Drivers,
    Network,
    Userspace,
}

/// Priority tier of an agent proposal.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ProposalPriority {
    Low,
    Normal,
    High,
    Critical,
}

/// A structured proposal submitted by an agent to modify or inspect OS state.
#[derive(Debug, Clone, PartialEq)]
pub struct Proposal {
    pub id: String,
    pub author_agent: String,
    pub target: SubsystemTarget,
    pub action: String,
    pub parameters: Vec<(String, String)>,
    pub required_capability: u64,
    pub priority: ProposalPriority,
    pub timestamp: u64,
}

impl Proposal {
    pub fn new(
        id: impl Into<String>,
        author_agent: impl Into<String>,
        target: SubsystemTarget,
        action: impl Into<String>,
        required_capability: u64,
        priority: ProposalPriority,
    ) -> Self {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_millis() as u64)
            .unwrap_or(0);
        Self {
            id: id.into(),
            author_agent: author_agent.into(),
            target,
            action: action.into(),
            parameters: Vec::new(),
            required_capability,
            priority,
            timestamp,
        }
    }

    pub fn with_param(mut self, key: impl Into<String>, val: impl Into<String>) -> Self {
        self.parameters.push((key.into(), val.into()));
        self
    }
}

/// Categorization of verifiable evidence submitted with or against a proposal.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EvidenceCategory {
    Telemetry,
    SafetyInvariant,
    ResourceConstraint,
    AuditLog,
    Benchmark,
}

/// An evidentiary item supporting or challenging a proposal.
#[derive(Debug, Clone, PartialEq)]
pub struct Evidence {
    pub id: String,
    pub proposal_id: String,
    pub provider_agent: String,
    pub category: EvidenceCategory,
    pub summary: String,
    pub confidence: f64,
    pub verifiable_hash: Option<u64>,
}

impl Evidence {
    pub fn new(
        id: impl Into<String>,
        proposal_id: impl Into<String>,
        provider_agent: impl Into<String>,
        category: EvidenceCategory,
        summary: impl Into<String>,
        confidence: f64,
    ) -> Self {
        Self {
            id: id.into(),
            proposal_id: proposal_id.into(),
            provider_agent: provider_agent.into(),
            category,
            summary: summary.into(),
            confidence: confidence.clamp(0.0, 1.0),
            verifiable_hash: None,
        }
    }
}

/// The stance of an agent vote on a proposal.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VoteStance {
    Approve,
    Reject,
    Abstain,
    ConditionalApproval,
}

/// An individual vote cast by a specialized agent during arbitration.
#[derive(Debug, Clone, PartialEq)]
pub struct Vote {
    pub agent_id: String,
    pub proposal_id: String,
    pub stance: VoteStance,
    pub confidence: f64,
    pub rationale: String,
    pub conditions: Vec<String>,
}

impl Vote {
    pub fn new(
        agent_id: impl Into<String>,
        proposal_id: impl Into<String>,
        stance: VoteStance,
        confidence: f64,
        rationale: impl Into<String>,
    ) -> Self {
        Self {
            agent_id: agent_id.into(),
            proposal_id: proposal_id.into(),
            stance,
            confidence: confidence.clamp(0.0, 1.0),
            rationale: rationale.into(),
            conditions: Vec::new(),
        }
    }

    pub fn with_condition(mut self, condition: impl Into<String>) -> Self {
        self.conditions.push(condition.into());
        self
    }
}

/// Final outcome reached by the decision arbitration layer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DecisionOutcome {
    Accepted,
    Rejected,
    SplitConsensus,
    QuorumFailed,
    EscalationRequired,
}

/// The synthesized decision for a specific proposal.
#[derive(Debug, Clone, PartialEq)]
pub struct Decision {
    pub proposal_id: String,
    pub outcome: DecisionOutcome,
    pub score: f64,
    pub votes_approve: usize,
    pub votes_reject: usize,
    pub conflict_flagged: bool,
    pub conflict_details: Option<String>,
    pub rationale: String,
}

/// The overall batch consensus result containing all evaluated proposals and decisions.
#[derive(Debug, Clone, PartialEq)]
pub struct ConsensusResult {
    pub batch_id: String,
    pub proposals: Vec<Proposal>,
    pub decisions: Vec<Decision>,
    pub conflict_detected: bool,
    pub timestamp: u64,
}
