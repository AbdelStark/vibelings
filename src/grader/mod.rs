//! Grading engine.
//!
//! Supports multiple grading patterns:
//! - **Schema validation**: JSON Schema validation for structured output
//! - **Invariant checking**: Shell scripts that verify conditions
//! - **Multi-run reliability**: K runs, pass if ≥N succeed
//! - **LLM-as-judge**: Last resort, rubric-based evaluation

mod invariant;
mod schema;

use crate::error::GradingError;
use crate::exercise::{Exercise, GraderType};
use crate::{Error, Result};
use serde::{Deserialize, Serialize};

/// Result of grading an exercise.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GradingResult {
    /// Whether the exercise passed
    pub passed: bool,

    /// Human-readable message about the grading
    pub message: String,

    /// Detailed breakdown (e.g., which invariants passed/failed)
    pub details: Vec<GradingDetail>,

    /// Schema validation errors if applicable
    pub schema_errors: Vec<String>,
}

/// A single detail item from grading.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GradingDetail {
    /// Name of the check
    pub name: String,

    /// Whether it passed
    pub passed: bool,

    /// Description or error message
    pub message: String,
}

/// The grading engine.
pub struct Grader {
    // Configuration could go here
}

impl Grader {
    /// Create a new grader.
    pub fn new() -> Result<Self> {
        Ok(Self {})
    }

    /// Grade an exercise based on the model output.
    pub fn grade(&self, exercise: &Exercise, output: &str) -> Result<GradingResult> {
        match exercise.manifest.grader.grader_type {
            GraderType::Schema => self.grade_schema(exercise, output),
            GraderType::Invariants => self.grade_invariants(exercise, output),
            GraderType::Combined => self.grade_combined(exercise, output),
            GraderType::Sandbox => self.grade_sandbox(exercise, output),
            GraderType::Reliability => self.grade_reliability(exercise, output),
            GraderType::LlmJudge => self.grade_llm_judge(exercise, output),
        }
    }

    /// Grade using JSON Schema validation.
    fn grade_schema(&self, exercise: &Exercise, output: &str) -> Result<GradingResult> {
        let schema_path = exercise
            .manifest
            .grader
            .schema_path
            .as_ref()
            .ok_or_else(|| {
                Error::Grading(GradingError::InvalidConfig(
                    "Schema grader requires schema_path".to_string(),
                ))
            })?;

        let full_path = exercise.grader_path.join(schema_path);
        if !full_path.exists() {
            return Err(Error::Grading(GradingError::SchemaNotFound(full_path)));
        }

        let schema_content = std::fs::read_to_string(&full_path)?;
        let schema: serde_json::Value = serde_json::from_str(&schema_content)?;

        // Try to parse output as JSON
        let output_json: serde_json::Value = match serde_json::from_str(output) {
            Ok(v) => v,
            Err(e) => {
                return Ok(GradingResult {
                    passed: false,
                    message: format!("Output is not valid JSON: {}", e),
                    details: vec![],
                    schema_errors: vec![e.to_string()],
                });
            }
        };

        // Validate against schema
        match schema::validate_json(&output_json, &schema) {
            Ok(()) => Ok(GradingResult {
                passed: true,
                message: "Schema validation passed".to_string(),
                details: vec![GradingDetail {
                    name: "schema".to_string(),
                    passed: true,
                    message: "Output matches schema".to_string(),
                }],
                schema_errors: vec![],
            }),
            Err(errors) => Ok(GradingResult {
                passed: false,
                message: format!("Schema validation failed: {} errors", errors.len()),
                details: vec![GradingDetail {
                    name: "schema".to_string(),
                    passed: false,
                    message: errors.join("; "),
                }],
                schema_errors: errors,
            }),
        }
    }

    /// Grade using invariant scripts.
    fn grade_invariants(&self, exercise: &Exercise, output: &str) -> Result<GradingResult> {
        let invariants = &exercise.manifest.grader.invariants;
        if invariants.is_empty() {
            return Err(Error::Grading(GradingError::InvalidConfig(
                "Invariants grader requires at least one invariant".to_string(),
            )));
        }

        let mut details = Vec::new();
        let mut all_passed = true;

        for invariant_path in invariants {
            let full_path = exercise.grader_path.join(invariant_path);
            if !full_path.exists() {
                return Err(Error::Grading(GradingError::InvariantScriptNotFound(
                    full_path,
                )));
            }

            let result = invariant::run_invariant(&full_path, output)?;
            if !result.passed {
                all_passed = false;
            }
            details.push(GradingDetail {
                name: invariant_path.clone(),
                passed: result.passed,
                message: result.message,
            });
        }

        let passed_count = details.iter().filter(|d| d.passed).count();
        let total = details.len();

        Ok(GradingResult {
            passed: all_passed,
            message: format!("Invariants: {}/{} passed", passed_count, total),
            details,
            schema_errors: vec![],
        })
    }

    /// Grade using both schema and invariants.
    fn grade_combined(&self, exercise: &Exercise, output: &str) -> Result<GradingResult> {
        let schema_result = self.grade_schema(exercise, output)?;
        if !schema_result.passed {
            return Ok(schema_result);
        }

        let invariant_result = self.grade_invariants(exercise, output)?;

        let mut details = schema_result.details;
        details.extend(invariant_result.details);

        Ok(GradingResult {
            passed: invariant_result.passed,
            message: format!("Schema: ✓ | {}", invariant_result.message),
            details,
            schema_errors: schema_result.schema_errors,
        })
    }

    /// Grade based on sandbox state.
    fn grade_sandbox(&self, _exercise: &Exercise, _output: &str) -> Result<GradingResult> {
        // TODO: Implement sandbox grading
        Ok(GradingResult {
            passed: true,
            message: "Sandbox grading not yet implemented".to_string(),
            details: vec![],
            schema_errors: vec![],
        })
    }

    /// Grade based on reliability (multi-run).
    fn grade_reliability(&self, _exercise: &Exercise, _output: &str) -> Result<GradingResult> {
        // TODO: Implement reliability grading
        Ok(GradingResult {
            passed: true,
            message: "Reliability grading not yet implemented".to_string(),
            details: vec![],
            schema_errors: vec![],
        })
    }

    /// Grade using LLM-as-judge (last resort).
    fn grade_llm_judge(&self, _exercise: &Exercise, _output: &str) -> Result<GradingResult> {
        // TODO: Implement LLM-as-judge grading
        Ok(GradingResult {
            passed: true,
            message: "LLM-as-judge grading not yet implemented".to_string(),
            details: vec![],
            schema_errors: vec![],
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grader_creation() {
        let grader = Grader::new();
        assert!(grader.is_ok());
    }
}
