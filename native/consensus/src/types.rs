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
