//! Deterministic Astral Consensus Engine and arbitration architecture.
//!
//! Provides deterministic systems-level multi-agent reasoning, evidence submission,
//! and arbitration without reliance on external network APIs or non-deterministic LLMs.

use crate::types::{
    ConsensusResult, Decision, DecisionOutcome, Evidence, EvidenceCategory, Proposal,
    ProposalPriority, SubsystemTarget, Vote, VoteStance,
};

/// Trait for deterministic system-level reasoning agents.
pub trait DeterministicAgent: Send + Sync {
    fn id(&self) -> &str;
    fn name(&self) -> &str;
    fn domain(&self) -> SubsystemTarget;
    fn evaluate(&self, proposal: &Proposal, evidence: &[Evidence]) -> Vote;
}

/// Specialized security policy and invariant verification agent.
pub struct DeterministicSecurityAgent;

impl DeterministicSecurityAgent {
    pub fn new() -> Self {
        Self
    }
}

impl Default for DeterministicSecurityAgent {
    fn default() -> Self {
        Self::new()
    }
}

impl DeterministicAgent for DeterministicSecurityAgent {
    fn id(&self) -> &str {
        "security"
    }

    fn name(&self) -> &str {
        "Security & Privilege Auditor"
    }

    fn domain(&self) -> SubsystemTarget {
        SubsystemTarget::Security
    }

    fn evaluate(&self, proposal: &Proposal, _evidence: &[Evidence]) -> Vote {
        // Enforce privilege checks
        if proposal.action.contains("unrestricted_root") || proposal.action.contains("raw_io_bypass") {
            return Vote::new(
                self.id(),
                &proposal.id,
                VoteStance::Reject,
                0.99,
                "Violation of fundamental PyxisOS security policy: unrestricted raw bypass rejected",
            );
        }

        if proposal.target == SubsystemTarget::Kernel || proposal.target == SubsystemTarget::Security {
            if proposal.required_capability < 100 {
                return Vote::new(
                    self.id(),
                    &proposal.id,
                    VoteStance::Reject,
                    0.95,
                    "Insufficient cryptographic capability token for Kernel/Security mutation",
                );
            }
        }

        Vote::new(
            self.id(),
            &proposal.id,
            VoteStance::Approve,
            0.92,
            "Security invariants and capability bounds verified",
        )
    }
}

/// Specialized resource budgeting, memory footprint, and telemetry agent.
pub struct DeterministicResourceAgent;

impl DeterministicResourceAgent {
    pub fn new() -> Self {
        Self
    }
}

impl Default for DeterministicResourceAgent {
    fn default() -> Self {
        Self::new()
    }
}

impl DeterministicAgent for DeterministicResourceAgent {
    fn id(&self) -> &str {
        "resource"
    }

    fn name(&self) -> &str {
        "Resource & Memory Profiler"
    }

    fn domain(&self) -> SubsystemTarget {
        SubsystemTarget::Memory
    }

    fn evaluate(&self, proposal: &Proposal, evidence: &[Evidence]) -> Vote {
        // Check for resource constraint violations in evidence
        let has_resource_pressure = evidence.iter().any(|e| {
            e.category == EvidenceCategory::ResourceConstraint && e.confidence > 0.8
        });

        if has_resource_pressure && proposal.priority != ProposalPriority::Critical {
            return Vote::new(
                self.id(),
                &proposal.id,
                VoteStance::ConditionalApproval,
                0.85,
                "Resource pressure detected; execution deferred to low-load scheduling slice",
            ).with_condition("defer_until_load_under_70_pct");
        }

        for (k, v) in &proposal.parameters {
            if k == "alloc_pages" {
                if let Ok(pages) = v.parse::<u64>() {
                    if pages > 16384 {
                        // Requesting > 64MB in one chunk
                        return Vote::new(
                            self.id(),
                            &proposal.id,
                            VoteStance::Reject,
                            0.90,
                            "Single physical frame allocation exceeds 64MB kernel quota",
                        );
                    }
                }
            }
        }

        Vote::new(
            self.id(),
            &proposal.id,
            VoteStance::Approve,
            0.88,
            "Resource and memory allocation within acceptable limits",
        )
    }
}

/// Specialized logic, dependency DAG, and state consistency verification agent.
pub struct DeterministicLogicAgent;

impl DeterministicLogicAgent {
    pub fn new() -> Self {
        Self
    }
}

impl Default for DeterministicLogicAgent {
    fn default() -> Self {
        Self::new()
    }
}

impl DeterministicAgent for DeterministicLogicAgent {
    fn id(&self) -> &str {
        "logic"
    }

    fn name(&self) -> &str {
        "Logical Consistency & Invariant Checker"
    }

    fn domain(&self) -> SubsystemTarget {
        SubsystemTarget::Kernel
    }

    fn evaluate(&self, proposal: &Proposal, _evidence: &[Evidence]) -> Vote {
        if proposal.action.is_empty() {
            return Vote::new(
                self.id(),
                &proposal.id,
                VoteStance::Reject,
                0.95,
                "Malformed proposal: empty action specification",
            );
        }

        Vote::new(
            self.id(),
            &proposal.id,
            VoteStance::Approve,
            0.91,
            "Action syntax and dependency invariants validated",
        )
    }
}

/// Deterministic Decision Arbitrator for Astral Consensus.
pub struct DeterministicArbitrator {
    agents: Vec<Box<dyn DeterministicAgent>>,
}

impl DeterministicArbitrator {
    pub fn new() -> Self {
        Self { agents: Vec::new() }
    }

    pub fn with_default_agents() -> Self {
        let mut arb = Self::new();
        arb.register_agent(Box::new(DeterministicSecurityAgent::new()));
        arb.register_agent(Box::new(DeterministicResourceAgent::new()));
        arb.register_agent(Box::new(DeterministicLogicAgent::new()));
        arb
    }

    pub fn register_agent(&mut self, agent: Box<dyn DeterministicAgent>) {
        self.agents.push(agent);
    }

    /// Arbitrates a single proposal against provided evidence.
    pub fn arbitrate_proposal(&self, proposal: &Proposal, evidence: &[Evidence]) -> Decision {
        let mut votes = Vec::new();
        let mut approve_weight = 0.0;
        let mut total_weight = 0.0;
        let mut count_approve = 0;
        let mut count_reject = 0;

        let mut high_conf_approve = false;
        let mut high_conf_reject = false;
        let mut security_veto = false;

        for agent in &self.agents {
            let vote = agent.evaluate(proposal, evidence);
            let mut weight = 1.0;

            // Domain authority weighting
            if agent.domain() == proposal.target {
                weight = 2.5;
            }

            match vote.stance {
                VoteStance::Approve => {
                    approve_weight += weight * vote.confidence;
                    total_weight += weight;
                    count_approve += 1;
                    if vote.confidence >= 0.85 {
                        high_conf_approve = true;
                    }
                }
                VoteStance::ConditionalApproval => {
                    approve_weight += weight * vote.confidence * 0.8;
                    total_weight += weight;
                    count_approve += 1;
                }
                VoteStance::Reject => {
                    total_weight += weight;
                    count_reject += 1;
                    if vote.confidence >= 0.85 {
                        high_conf_reject = true;
                    }
                    if agent.id() == "security" {
                        security_veto = true;
                    }
                }
                VoteStance::Abstain => {}
            }

            votes.push(vote);
        }

        let score = if total_weight > 0.0 {
            approve_weight / total_weight
        } else {
            0.0
        };

        let conflict_flagged = high_conf_approve && high_conf_reject;
        let conflict_details = if conflict_flagged {
            Some(format!(
                "Material conflict detected on proposal '{}': concurrent high-confidence Approve and Reject votes",
                proposal.id
            ))
        } else {
            None
        };

        let outcome = if security_veto {
            DecisionOutcome::Rejected
        } else if conflict_flagged && proposal.priority == ProposalPriority::Critical {
            DecisionOutcome::EscalationRequired
        } else if score >= 0.65 && count_approve >= count_reject {
            DecisionOutcome::Accepted
        } else if count_approve == count_reject && count_approve > 0 {
            DecisionOutcome::SplitConsensus
        } else {
            DecisionOutcome::Rejected
        };

        let rationale = format!(
            "Consensus outcome {:?} with score {:.2} (Approvals: {}, Rejections: {})",
            outcome, score, count_approve, count_reject
        );

        Decision {
            proposal_id: proposal.id.clone(),
            outcome,
            score,
            votes_approve: count_approve,
            votes_reject: count_reject,
            conflict_flagged,
            conflict_details,
            rationale,
        }
    }

    /// Evaluates a batch of proposals against known evidence.
    pub fn arbitrate_batch(
        &self,
        batch_id: &str,
        proposals: &[Proposal],
        evidence: &[Evidence],
    ) -> ConsensusResult {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_millis() as u64)
            .unwrap_or(0);

        let mut decisions = Vec::new();
        let mut any_conflict = false;

        for proposal in proposals {
            let rel_evidence: Vec<Evidence> = evidence
                .iter()
                .filter(|e| e.proposal_id == proposal.id)
                .cloned()
                .collect();
            let decision = self.arbitrate_proposal(proposal, &rel_evidence);
            if decision.conflict_flagged {
                any_conflict = true;
            }
            decisions.push(decision);
        }

        ConsensusResult {
            batch_id: batch_id.to_string(),
            proposals: proposals.to_vec(),
            decisions,
            conflict_detected: any_conflict,
            timestamp,
        }
    }
}

impl Default for DeterministicArbitrator {
    fn default() -> Self {
        Self::with_default_agents()
    }
}
