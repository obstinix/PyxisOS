//! Security Agent implementation for risk assessment, privilege bounds, and isolation policies.

use super::{parse_opinion_json, Agent};
use crate::provider::LlmProvider;
use crate::types::{AgentContext, AgentOpinion};

pub struct SecurityAgent;

impl SecurityAgent {
    pub const ID: &'static str = "security";
    pub const NAME: &'static str = "Security Agent";
    pub const DESCRIPTION: &'static str =
        "Identifies safety risks, maps privilege boundaries, and validates containment policies.";

    pub fn new() -> Self {
        Self
    }
}

impl Default for SecurityAgent {
    fn default() -> Self {
        Self::new()
    }
}

impl Agent for SecurityAgent {
    fn id(&self) -> &str {
        Self::ID
    }

    fn name(&self) -> &str {
        Self::NAME
    }

    fn description(&self) -> &str {
        Self::DESCRIPTION
    }

    fn analyze(&self, context: &AgentContext, provider: &dyn LlmProvider) -> AgentOpinion {
        let system_prompt = "You are the Security Agent for PyxisOS. Your role is safety-framing, \
identifying security risks, privilege boundaries, and containment policies.\n\
Analyze the user query, focus on potential execution risks (e.g., shell command execution, container escapes, \
unauthorized host filesystem access, untrusted network operations) and outline mitigation requirements.\n\n\
You MUST respond ONLY with a raw JSON object matching the following structure:\n\
{\n  \"opinion\": \"your detailed security assessment, vulnerability notes, and risk constraints\",\n  \"confidence\": 0.95,\n  \"rationale\": \"why this confidence level was chosen based on the risk profile of the request\"\n}\n\n\
Do NOT wrap the JSON in markdown code blocks. Do NOT include any explanations outside the JSON object.";

        let user_prompt = format!("Query: \"{}\"\nTimestamp: {}", context.query, context.timestamp);

        match provider.send(system_prompt, &user_prompt) {
            Ok(resp) => parse_opinion_json(&resp, self.name()),
            Err(e) => AgentOpinion::new(
                format!("Security screening failed: {}", e),
                0.0,
                format!("Failed to execute security screen: {}", e),
            ),
        }
    }
}
