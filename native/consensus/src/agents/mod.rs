//! Specialist agents for the Astral Consensus Engine.

pub mod logic;
pub mod research;
pub mod security;

pub use logic::LogicAgent;
pub use research::ResearchAgent;
pub use security::SecurityAgent;

use crate::provider::LlmProvider;
use crate::types::{AgentContext, AgentOpinion};

/// Common trait implemented by all specialist agents.
pub trait Agent: Send + Sync {
    fn id(&self) -> &str;
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn analyze(&self, context: &AgentContext, provider: &dyn LlmProvider) -> AgentOpinion;
}

/// Helper function to parse simple JSON agent responses.
pub(crate) fn parse_opinion_json(raw: &str, fallback_name: &str) -> AgentOpinion {
    let clean = raw.trim().trim_start_matches("```json").trim_start_matches("```").trim_end_matches("```").trim();

    // Extract opinion
    let opinion = if let Some(idx) = clean.find(r#""opinion""#) {
        extract_json_string(&clean[idx..])
    } else {
        clean.to_string()
    };

    // Extract confidence
    let confidence = if let Some(idx) = clean.find(r#""confidence""#) {
        extract_json_number(&clean[idx..]).unwrap_or(0.90)
    } else {
        0.85
    };

    // Extract rationale
    let rationale = if let Some(idx) = clean.find(r#""rationale""#) {
        extract_json_string(&clean[idx..])
    } else {
        format!("Analysis provided by {}.", fallback_name)
    };

    AgentOpinion::new(opinion, confidence, rationale)
}

fn extract_json_string(slice: &str) -> String {
    if let Some(colon) = slice.find(':') {
        let after_colon = slice[colon + 1..].trim_start();
        if let Some(start_quote) = after_colon.find('"') {
            let after_quote = &after_colon[start_quote + 1..];
            let mut result = String::new();
            let mut escape = false;
            for ch in after_quote.chars() {
                if escape {
                    match ch {
                        'n' => result.push('\n'),
                        't' => result.push('\t'),
                        'r' => result.push('\r'),
                        '\\' => result.push('\\'),
                        '"' => result.push('"'),
                        _ => result.push(ch),
                    }
                    escape = false;
                } else if ch == '\\' {
                    escape = true;
                } else if ch == '"' {
                    break;
                } else {
                    result.push(ch);
                }
            }
            return result;
        }
    }
    slice.to_string()
}

fn extract_json_number(slice: &str) -> Option<f64> {
    if let Some(colon) = slice.find(':') {
        let after_colon = slice[colon + 1..].trim_start();
        let num_str: String = after_colon
            .chars()
            .take_while(|c| c.is_ascii_digit() || *c == '.' || *c == '-')
            .collect();
        return num_str.parse::<f64>().ok();
    }
    None
}
