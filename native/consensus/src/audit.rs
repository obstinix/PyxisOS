//! Audit logger for tracking consensus decisions and agent reasoning.

use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};

pub struct AuditLogger {
    log_path: PathBuf,
}

impl AuditLogger {
    pub fn new(log_path: impl AsRef<Path>) -> Self {
        Self {
            log_path: log_path.as_ref().to_path_buf(),
        }
    }

    pub fn default_path() -> Self {
        Self::new("audit.log")
    }

    /// Logs a generic structured event to the audit log.
    pub fn log_event(&self, event: &str, payload: &[(&str, &str)]) {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_millis())
            .unwrap_or(0);

        let mut entry = format!(r#"{{"timestamp":{},"event":"{}""#, timestamp, event);
        for &(k, v) in payload {
            let escaped = v.replace('\\', "\\\\").replace('"', "\\\"").replace('\n', "\\n");
            entry.push_str(&format!(r#","{}":"{}""#, k, escaped));
        }
        entry.push_str("}\n");

        if let Ok(mut file) = OpenOptions::new().create(true).append(true).open(&self.log_path) {
            let _ = file.write_all(entry.as_bytes());
        }
    }

    /// Logs an agent's individual opinion.
    pub fn log_agent_call(&self, agent_id: &str, query: &str, opinion: &str, confidence: f64) {
        let conf_str = format!("{:.2}", confidence);
        self.log_event(
            "agent_call",
            &[
                ("agent_id", agent_id),
                ("query", query),
                ("opinion", opinion),
                ("confidence", &conf_str),
            ],
        );
    }

    /// Logs the final synthesized decision from the arbitration layer.
    pub fn log_arbitration(
        &self,
        query: &str,
        decision: &str,
        conflict_flagged: bool,
        conflict_details: Option<&str>,
    ) {
        let conflict_str = if conflict_flagged { "true" } else { "false" };
        let details = conflict_details.unwrap_or("");
        self.log_event(
            "arbitration_decision",
            &[
                ("query", query),
                ("decision", decision),
                ("conflict_flagged", conflict_str),
                ("conflict_details", details),
            ],
        );
    }
}
