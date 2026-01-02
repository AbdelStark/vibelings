//! JSON output types for machine-readable CLI output.
//!
//! These types are used when the `--json` flag is provided to output
//! structured data that can be parsed by scripts and CI systems.

use serde::Serialize;

use crate::runner::RunResult;
use crate::ExerciseStatus;

/// JSON output for the `list` command.
#[derive(Debug, Serialize)]
pub struct ListOutput {
    /// List of exercises
    pub exercises: Vec<ExerciseInfo>,
    /// Summary statistics
    pub summary: ListSummary,
}

/// Information about a single exercise.
#[derive(Debug, Serialize)]
pub struct ExerciseInfo {
    /// Full exercise ID (e.g., "fundamentals/json_01")
    pub id: String,
    /// Human-readable title
    pub title: String,
    /// Track name
    pub track: String,
    /// Current status
    pub status: String,
    /// Whether prerequisites are met
    pub unlocked: bool,
    /// Difficulty level (1-5)
    pub difficulty: u8,
    /// List of prerequisite exercise IDs
    pub prerequisites: Vec<String>,
}

/// Summary statistics for the list command.
#[derive(Debug, Serialize)]
pub struct ListSummary {
    /// Total number of exercises
    pub total: usize,
    /// Number of completed exercises
    pub completed: usize,
    /// Number of exercises in progress
    pub in_progress: usize,
    /// Number of pending exercises
    pub pending: usize,
    /// Completion percentage
    pub completion_percent: f64,
}

/// JSON output for the `run` command.
#[derive(Debug, Serialize)]
pub struct RunOutput {
    /// Exercise ID that was run
    pub exercise: String,
    /// Run result
    #[serde(flatten)]
    pub result: RunResult,
}

/// JSON output for the `doctor` command.
#[derive(Debug, Serialize)]
pub struct DoctorOutput {
    /// Overall health status
    pub healthy: bool,
    /// Individual health checks
    pub checks: Vec<HealthCheck>,
    /// Number of checks passed
    pub passed: usize,
    /// Total number of checks
    pub total: usize,
}

/// A single health check result.
#[derive(Debug, Serialize)]
pub struct HealthCheck {
    /// Name of the check
    pub name: String,
    /// Whether the check passed
    pub passed: bool,
    /// Optional detail message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
    /// Optional warning (for non-critical issues)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warning: Option<String>,
}

/// JSON output for the `cost` command.
#[derive(Debug, Serialize)]
pub struct CostOutput {
    /// Per-exercise cost breakdown
    pub exercises: Vec<ExerciseCost>,
    /// Summary statistics
    pub summary: CostSummary,
}

/// Cost information for a single exercise.
#[derive(Debug, Serialize)]
pub struct ExerciseCost {
    /// Exercise ID
    pub id: String,
    /// Total tokens used
    pub tokens: u64,
    /// Total cost in USD
    pub cost_usd: f64,
}

/// Summary cost statistics.
#[derive(Debug, Serialize)]
pub struct CostSummary {
    /// Total tokens across all exercises
    pub total_tokens: u64,
    /// Total cost in USD
    pub total_cost_usd: f64,
    /// Average tokens per exercise
    pub avg_tokens: u64,
    /// Average cost per exercise
    pub avg_cost_usd: f64,
    /// Number of exercises tracked
    pub exercise_count: usize,
}

/// JSON output for the `verify` command.
#[derive(Debug, Serialize)]
pub struct VerifyOutput {
    /// Overall verification success
    pub success: bool,
    /// Individual exercise results
    pub results: Vec<VerifyResult>,
    /// Summary statistics
    pub summary: VerifySummary,
}

/// Verification result for a single exercise.
#[derive(Debug, Serialize)]
pub struct VerifyResult {
    /// Exercise ID
    pub id: String,
    /// Whether the exercise passed
    pub passed: bool,
    /// Error message if failed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// Summary statistics for verification.
#[derive(Debug, Serialize)]
pub struct VerifySummary {
    /// Number of exercises verified
    pub total: usize,
    /// Number that passed
    pub passed: usize,
    /// Number that failed
    pub failed: usize,
}

/// Convert ExerciseStatus to a string for JSON output.
pub fn status_to_string(status: &ExerciseStatus) -> String {
    match status {
        ExerciseStatus::Pending => "pending".to_string(),
        ExerciseStatus::InProgress => "in_progress".to_string(),
        ExerciseStatus::Completed => "completed".to_string(),
        ExerciseStatus::Flaky => "flaky".to_string(),
        ExerciseStatus::NeedsReruns => "needs_reruns".to_string(),
        ExerciseStatus::Experimental => "experimental".to_string(),
    }
}

/// Print JSON output to stdout.
pub fn print_json<T: Serialize>(value: &T) -> crate::Result<()> {
    let json = serde_json::to_string_pretty(value)?;
    println!("{}", json);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_status_to_string_all_variants() {
        assert_eq!(status_to_string(&ExerciseStatus::Pending), "pending");
        assert_eq!(status_to_string(&ExerciseStatus::InProgress), "in_progress");
        assert_eq!(status_to_string(&ExerciseStatus::Completed), "completed");
        assert_eq!(status_to_string(&ExerciseStatus::Flaky), "flaky");
        assert_eq!(
            status_to_string(&ExerciseStatus::NeedsReruns),
            "needs_reruns"
        );
        assert_eq!(
            status_to_string(&ExerciseStatus::Experimental),
            "experimental"
        );
    }

    #[test]
    fn test_list_output_serialization() {
        let output = ListOutput {
            exercises: vec![ExerciseInfo {
                id: "fundamentals/json_01".to_string(),
                title: "JSON Basics".to_string(),
                track: "fundamentals".to_string(),
                status: "completed".to_string(),
                unlocked: true,
                difficulty: 1,
                prerequisites: vec![],
            }],
            summary: ListSummary {
                total: 1,
                completed: 1,
                in_progress: 0,
                pending: 0,
                completion_percent: 100.0,
            },
        };

        let json = serde_json::to_string(&output).unwrap();
        assert!(json.contains("fundamentals/json_01"));
        assert!(json.contains("JSON Basics"));
    }

    #[test]
    fn test_doctor_output_serialization() {
        let output = DoctorOutput {
            healthy: true,
            checks: vec![
                HealthCheck {
                    name: "API Key".to_string(),
                    passed: true,
                    detail: Some("Found in environment".to_string()),
                    warning: None,
                },
                HealthCheck {
                    name: "Network".to_string(),
                    passed: false,
                    detail: None,
                    warning: Some("Offline mode".to_string()),
                },
            ],
            passed: 1,
            total: 2,
        };

        let json = serde_json::to_string(&output).unwrap();
        assert!(json.contains("API Key"));
        assert!(json.contains("healthy"));
        // detail should be present for first check
        assert!(json.contains("Found in environment"));
        // warning should be present for second check
        assert!(json.contains("Offline mode"));
    }

    #[test]
    fn test_cost_output_serialization() {
        let output = CostOutput {
            exercises: vec![ExerciseCost {
                id: "json_01".to_string(),
                tokens: 500,
                cost_usd: 0.0025,
            }],
            summary: CostSummary {
                total_tokens: 500,
                total_cost_usd: 0.0025,
                avg_tokens: 500,
                avg_cost_usd: 0.0025,
                exercise_count: 1,
            },
        };

        let json = serde_json::to_string(&output).unwrap();
        assert!(json.contains("json_01"));
        assert!(json.contains("500"));
    }

    #[test]
    fn test_verify_output_serialization() {
        let output = VerifyOutput {
            success: false,
            results: vec![
                VerifyResult {
                    id: "json_01".to_string(),
                    passed: true,
                    error: None,
                },
                VerifyResult {
                    id: "json_02".to_string(),
                    passed: false,
                    error: Some("Schema validation failed".to_string()),
                },
            ],
            summary: VerifySummary {
                total: 2,
                passed: 1,
                failed: 1,
            },
        };

        let json = serde_json::to_string(&output).unwrap();
        assert!(json.contains("json_01"));
        assert!(json.contains("Schema validation failed"));
        // error should not be present for passing exercise (skip_serializing_if)
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        let results = parsed["results"].as_array().unwrap();
        assert!(results[0].get("error").is_none());
    }

    #[test]
    fn test_exercise_info_defaults() {
        let info = ExerciseInfo {
            id: "test".to_string(),
            title: "Test".to_string(),
            track: "test".to_string(),
            status: "pending".to_string(),
            unlocked: false,
            difficulty: 1,
            prerequisites: vec!["prereq".to_string()],
        };

        assert_eq!(info.prerequisites.len(), 1);
        assert!(!info.unlocked);
    }

    #[test]
    fn test_run_output_serialization() {
        let output = RunOutput {
            exercise: "json_01".to_string(),
            result: RunResult {
                passed: true,
                error_message: None,
                duration_secs: 0.15,
                cost_usd: 0.001,
                tool_calls: 0,
                tokens_in: 100,
                tokens_out: 50,
                grading_details: None,
                trace_id: Some("trace-123".to_string()),
            },
        };

        let json = serde_json::to_string(&output).unwrap();
        assert!(json.contains("json_01"));
        assert!(json.contains("passed"));
        assert!(json.contains("0.15"));
    }
}
