//! Trace capture and replay.
//!
//! Records all model interactions and tool calls for debugging and replay.

mod store;

pub use store::TraceStore;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// A recorded trace of an exercise run.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trace {
    /// Unique trace ID
    pub id: String,

    /// Exercise ID
    pub exercise_id: String,

    /// Timestamp of the run
    pub timestamp: String,

    /// Duration in seconds
    pub duration_secs: f64,

    /// Input messages
    pub messages: Vec<TraceMessage>,

    /// Model response
    pub response: Option<String>,

    /// Tool calls made during the run
    pub tool_calls: Vec<TraceToolCall>,

    /// Whether the exercise passed
    pub passed: bool,

    /// Input token count.
    pub tokens_in: u32,
    /// Output token count.
    pub tokens_out: u32,

    /// Estimated cost in USD.
    pub cost_usd: f64,
}

/// A message in the trace.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraceMessage {
    /// Role (system, user, assistant)
    pub role: String,

    /// Content
    pub content: String,
}

/// A tool call in the trace.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraceToolCall {
    /// Tool name
    pub name: String,

    /// Arguments (JSON)
    pub arguments: String,

    /// Result
    pub result: String,

    /// Duration in milliseconds
    pub duration_ms: u64,
}

impl Trace {
    /// Create a new trace.
    pub fn new(exercise_id: &str) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            exercise_id: exercise_id.to_string(),
            timestamp: chrono::Utc::now().to_rfc3339(),
            duration_secs: 0.0,
            messages: Vec::new(),
            response: None,
            tool_calls: Vec::new(),
            passed: false,
            tokens_in: 0,
            tokens_out: 0,
            cost_usd: 0.0,
        }
    }

    /// Add a message to the trace.
    pub fn add_message(&mut self, role: &str, content: &str) {
        self.messages.push(TraceMessage {
            role: role.to_string(),
            content: content.to_string(),
        });
    }

    /// Add a tool call to the trace.
    pub fn add_tool_call(&mut self, name: &str, arguments: &str, result: &str, duration_ms: u64) {
        self.tool_calls.push(TraceToolCall {
            name: name.to_string(),
            arguments: arguments.to_string(),
            result: result.to_string(),
            duration_ms,
        });
    }

    /// Set the response.
    pub fn set_response(&mut self, response: &str) {
        self.response = Some(response.to_string());
    }

    /// Mark as completed.
    pub fn complete(&mut self, passed: bool, duration_secs: f64) {
        self.passed = passed;
        self.duration_secs = duration_secs;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trace_creation() {
        let trace = Trace::new("test/exercise");
        assert!(!trace.id.is_empty());
        assert_eq!(trace.exercise_id, "test/exercise");
    }

    #[test]
    fn test_trace_messages() {
        let mut trace = Trace::new("test/exercise");
        trace.add_message("user", "Hello");
        trace.add_message("assistant", "Hi there");

        assert_eq!(trace.messages.len(), 2);
        assert_eq!(trace.messages[0].role, "user");
    }
}
