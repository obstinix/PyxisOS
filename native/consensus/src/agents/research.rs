//! Research Agent implementation for factual investigation and technical context.

use super::{parse_opinion_json, Agent};
use crate::provider::LlmProvider;
use crate::types::{AgentContext, AgentOpinion};

pub struct ResearchAgent;

impl ResearchAgent {
    pub const ID: &'static str = "research";
    pub const NAME: &'static str = "Research Agent";
    pub const DESCRIPTION: &'static str =
        "Gathers factual context, compiles technical specifications, and verifies details.";

    pub fn new() -> Self {
        Self
    }
}

impl Default for ResearchAgent {
    fn default() -> Self {
        Self::new()
    }
}

impl Agent for ResearchAgent {
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
        let system_prompt = "You are the Research Agent for PyxisOS. Your role is context-gathering, \
compiling technical specifications, and factual research.\n\
Analyze the user query, focus on factual correctness, relevant background details, and reference materials.\n\n\
You MUST respond ONLY with a raw JSON object matching the following structure:\n\
{\n  \"opinion\": \"your detailed findings and context summary\",\n  \"confidence\": 0.95,\n  \"rationale\": \"why this confidence level was chosen based on current knowledge\"\n}\n\n\
Do NOT wrap the JSON in markdown code blocks. Do NOT include any explanations outside the JSON object.";

        let user_prompt = format!("Query: \"{}\"\nTimestamp: {}", context.query, context.timestamp);

        match provider.send(system_prompt, &user_prompt) {
            Ok(resp) => parse_opinion_json(&resp, self.name()),
            Err(e) => AgentOpinion::new(
                format!("Factual research failed: {}", e),
                0.0,
                format!("Failed to compile research: {}", e),
            ),
        }
    }
}
