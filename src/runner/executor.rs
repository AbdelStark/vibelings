//! Exercise execution.

use serde::{Deserialize, Serialize};

/// Result of running an exercise.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunResult {
    /// Whether the exercise passed
    pub passed: bool,

    /// Error message if failed
    pub error_message: Option<String>,

    /// Duration in seconds
    pub duration_secs: f64,

    /// Cost in USD
    pub cost_usd: f64,

    /// Number of tool calls made
    pub tool_calls: u32,

    /// Input tokens used
    pub tokens_in: u32,

    /// Output tokens used
    pub tokens_out: u32,

    /// Grading details
    pub grading_details: Option<String>,

    /// Trace ID for this run
    pub trace_id: Option<String>,
}

impl Default for RunResult {
    fn default() -> Self {
        Self {
            passed: false,
            error_message: None,
            duration_secs: 0.0,
            cost_usd: 0.0,
            tool_calls: 0,
            tokens_in: 0,
            tokens_out: 0,
            grading_details: None,
            trace_id: None,
        }
    }
}
