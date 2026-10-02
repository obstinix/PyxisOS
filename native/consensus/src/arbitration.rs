//! Decision Arbitration Layer synthesizing multi-agent inputs into consensus decisions.

use crate::audit::AuditLogger;
use crate::provider::LlmProvider;
use crate::types::{AgentContext, AgentOpinion, ArbitrationResult};

pub struct DecisionArbitrationLayer<'a> {
    provider: &'a dyn LlmProvider,
    logger: &'a AuditLogger,
}

impl<'a> DecisionArbitrationLayer<'a> {
    pub fn new(provider: &'a dyn LlmProvider, logger: &'a AuditLogger) -> Self {
        Self { provider, logger }
    }

    /// Arbitrates the opinions of active agents into a unified, transparent consensus decision.
    pub fn arbitrate(
        &self,
        context: &AgentContext,
        opinions: &[(&str, &AgentOpinion)],
    ) -> ArbitrationResult {
        // 1. Log each agent opinion to audit trail
        for (agent_id, op) in opinions {
            self.logger
                .log_agent_call(agent_id, &context.query, &op.opinion, op.confidence);
        }

        // 2. Format agent opinions
        let mut formatted = String::new();
        for (agent_id, op) in opinions {
            formatted.push_str(&format!(
                "[Agent: {}] (Confidence: {:.2})\nOpinion: {}\nRationale: {}\n\n",
                agent_id, op.confidence, op.opinion, op.rationale
            ));
        }

        // 3. Build arbitration prompt
        let system_prompt = "You are the Decision Arbitration Layer for the PyxisOS Astral Consensus Engine.\n\
Your role is to perform a single \"judge\" pass that synthesizes the opinions of specialized agents into a unified, transparent decision.\n\
You must also analyze the agents' opinions for any material contradictions or disagreements.\n\n\
Conflict Threshold Rule:\n\
A conflict exists if two or more high-confidence agents (confidence >= 0.7) present materially contradictory stances or disagree on the primary resolution.\n\n\
You MUST respond ONLY with a raw JSON object matching the following structure:\n\
{\n  \"synthesizedDecision\": \"your transparent final decision resolving the query, highlighting the rationale of the agents\",\n  \"conflictFlagged\": true/false,\n  \"conflictDetails\": \"explanation of the contradiction if flagged, detailing which agents disagreed and on what points\"\n}\n\n\
Do NOT wrap the JSON in markdown code blocks. Do NOT include any explanations outside the JSON object.";

        let user_prompt = format!("Query: \"{}\"\n\nAgent Opinions:\n{}", context.query, formatted);

        let result = match self.provider.send(system_prompt, &user_prompt) {
            Ok(resp) => self.parse_arbitration_response(&resp),
            Err(e) => ArbitrationResult::new(
                format!("Failed to arbitrate consensus: {}", e),
                true,
                Some(format!("Arbitration system error: {}", e)),
            ),
        };

        // 4. Log the final arbitration decision
        self.logger.log_arbitration(
            &context.query,
            &result.synthesized_decision,
            result.conflict_flagged,
            result.conflict_details.as_deref(),
        );

        result
    }

    fn parse_arbitration_response(&self, raw: &str) -> ArbitrationResult {
        let clean = raw.trim().trim_start_matches("```json").trim_start_matches("```").trim_end_matches("```").trim();

        let decision = if let Some(idx) = clean.find(r#""synthesizedDecision""#) {
            extract_json_string(&clean[idx..])
        } else {
            clean.to_string()
        };

        let conflict_flagged = if let Some(idx) = clean.find(r#""conflictFlagged""#) {
            clean[idx..].contains("true")
        } else {
            false
        };

        let conflict_details = if conflict_flagged {
            if let Some(idx) = clean.find(r#""conflictDetails""#) {
                let det = extract_json_string(&clean[idx..]);
                if det.is_empty() || det == "null" {
                    None
                } else {
                    Some(det)
                }
            } else {
                None
            }
        } else {
            None
        };

        ArbitrationResult::new(decision, conflict_flagged, conflict_details)
    }
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
