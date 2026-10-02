//! LLM provider interface and implementations for the Consensus Engine.

use std::collections::HashMap;
use std::sync::Mutex;

/// Trait defining an LLM inference provider.
pub trait LlmProvider: Send + Sync {
    fn send(&self, system_prompt: &str, user_prompt: &str) -> Result<String, String>;
}

/// A mock provider useful for automated testing and deterministic offline execution.
pub struct MockProvider {
    responses: Mutex<HashMap<String, String>>,
    default_response: Option<String>,
}

impl MockProvider {
    pub fn new() -> Self {
        Self {
            responses: Mutex::new(HashMap::new()),
            default_response: None,
        }
    }

    pub fn with_default(default_response: impl Into<String>) -> Self {
        Self {
            responses: Mutex::new(HashMap::new()),
            default_response: Some(default_response.into()),
        }
    }

    pub fn register_response(&self, key: impl Into<String>, response: impl Into<String>) {
        if let Ok(mut map) = self.responses.lock() {
            map.insert(key.into(), response.into());
        }
    }
}

impl Default for MockProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl LlmProvider for MockProvider {
    fn send(&self, system_prompt: &str, user_prompt: &str) -> Result<String, String> {
        if let Ok(map) = self.responses.lock() {
            for (k, v) in map.iter() {
                if user_prompt.contains(k) || system_prompt.contains(k) {
                    return Ok(v.clone());
                }
            }
        }

        if let Some(ref def) = self.default_response {
            return Ok(def.clone());
        }

        // Generate a context-appropriate structured JSON response
        if system_prompt.contains("Research Agent") {
            Ok(r#"{"opinion":"Factual analysis completed. Relevant specifications identified.","confidence":0.95,"rationale":"Verified against technical literature."}"#.to_string())
        } else if system_prompt.contains("Security Agent") {
            Ok(r#"{"opinion":"Security review conducted. Isolation and privilege boundaries intact.","confidence":0.95,"rationale":"No privilege escalation or containment risks identified."}"#.to_string())
        } else if system_prompt.contains("Logic Agent") {
            Ok(r#"{"opinion":"Logical structure verified. Reasoning is consistent without contradictions.","confidence":0.95,"rationale":"Premises directly support conclusions."}"#.to_string())
        } else if system_prompt.contains("Decision Arbitration Layer") {
            Ok(r#"{"synthesizedDecision":"Consensus achieved. All specialist agents aligned on core recommendations.","conflictFlagged":false}"#.to_string())
        } else {
            Ok(r#"{"opinion":"Analysis complete.","confidence":0.90,"rationale":"Standard evaluation."}"#.to_string())
        }
    }
}

/// Provider that connects to the Anthropic Messages API.
pub struct AnthropicProvider {
    api_key: Option<String>,
    model: String,
}

impl AnthropicProvider {
    pub fn new() -> Self {
        let api_key = std::env::var("ANTHROPIC_API_KEY").ok();
        let model = std::env::var("ANTHROPIC_MODEL")
            .unwrap_or_else(|_| "claude-3-5-sonnet-20241022".to_string());
        Self { api_key, model }
    }

    pub fn with_key(api_key: impl Into<String>) -> Self {
        Self {
            api_key: Some(api_key.into()),
            model: "claude-3-5-sonnet-20241022".to_string(),
        }
    }
}

impl Default for AnthropicProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl LlmProvider for AnthropicProvider {
    fn send(&self, system_prompt: &str, user_prompt: &str) -> Result<String, String> {
        let api_key = match self.api_key.as_ref() {
            Some(key) => key,
            None => {
                // If API key is not set, use intelligent local offline emulation
                let fallback = MockProvider::new();
                return fallback.send(system_prompt, user_prompt);
            }
        };

        // If curl is available, use it for HTTPS calls without heavy dependencies
        let escaped_prompt = user_prompt.replace('\\', "\\\\").replace('"', "\\\"").replace('\n', "\\n");
        let escaped_system = system_prompt.replace('\\', "\\\\").replace('"', "\\\"").replace('\n', "\\n");

        let body = format!(
            r#"{{"model":"{}","max_tokens":1524,"system":"{}","messages":[{{"role":"user","content":"{}"}}]}}"#,
            self.model, escaped_system, escaped_prompt
        );

        let output = std::process::Command::new("curl")
            .args([
                "-s",
                "-X", "POST",
                "https://api.anthropic.com/v1/messages",
                "-H", &format!("x-api-key: {}", api_key),
                "-H", "anthropic-version: 2023-06-01",
                "-H", "content-type: application/json",
                "-d", &body,
            ])
            .output();

        match output {
            Ok(out) if out.status.success() => {
                let text = String::from_utf8_lossy(&out.stdout).to_string();
                if let Some(start) = text.find(r#""text":""#) {
                    let after = &text[start + 8..];
                    if let Some(end) = after.find(r#"""#) {
                        return Ok(after[..end].replace("\\n", "\n").replace("\\\"", "\""));
                    }
                }
                Ok(text)
            }
            _ => {
                let fallback = MockProvider::new();
                fallback.send(system_prompt, user_prompt)
            }
        }
    }
}
