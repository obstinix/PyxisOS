//! Logic Agent implementation for consistency verification, paradox resolution, and reasoning flow.

use super::{parse_opinion_json, Agent};
use crate::provider::LlmProvider;
use crate::types::{AgentContext, AgentOpinion};

pub struct LogicAgent;

impl LogicAgent {
    pub const ID: &'static str = "logic";
    pub const NAME: &'static str = "Logic Agent";
    pub const DESCRIPTION: &'static str =
        "Verifies logical consistency, identifies contradictions, and checks reasoning flow.";

    pub fn new() -> Self {
        Self
    }
}

impl Default for LogicAgent {
    fn default() -> Self {
        Self::new()
    }
}

impl Agent for LogicAgent {
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
        let system_prompt = "You are the Logic Agent for PyxisOS. Your role is logical verification, \
consistency validation, and identifying contradictions in user queries or plan drafts.\n\
Analyze the user query, focus on structural logic, consistency, paradoxes, causal links, and overall feasibility.\n\n\
You MUST respond ONLY with a raw JSON object matching the following structure:\n\
{\n  \"opinion\": \"your detailed logical analysis, contradiction filtering, and consistency checks\",\n  \"confidence\": 0.95,\n  \"rationale\": \"why this confidence level was chosen based on the logical coherence of the request\"\n}\n\n\
Do NOT wrap the JSON in markdown code blocks. Do NOT include any explanations outside the JSON object.";

        let user_prompt = format!("Query: \"{}\"\nTimestamp: {}", context.query, context.timestamp);

        match provider.send(system_prompt, &user_prompt) {
            Ok(resp) => parse_opinion_json(&resp, self.name()),
            Err(e) => AgentOpinion::new(
                format!("Logical screening failed: {}", e),
                0.0,
                format!("Failed to execute logical check: {}", e),
            ),
        }
    }
}
