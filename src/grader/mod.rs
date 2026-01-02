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

    /// Grade based on sandbox state (tool call validation).
    ///
    /// Sandbox grading validates that the model output contains valid tool calls
    /// that conform to expected schemas and constraints.
    ///
    /// Expected output format:
    /// ```json
    /// {
    ///   "tool_calls": [
    ///     {"name": "tool_name", "arguments": {...}}
    ///   ],
    ///   "result": "optional final result"
    /// }
    /// ```
    fn grade_sandbox(&self, exercise: &Exercise, output: &str) -> Result<GradingResult> {
        // Parse output as JSON
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

        // Extract tool_calls array
        let tool_calls = match output_json.get("tool_calls") {
            Some(serde_json::Value::Array(calls)) => calls,
            Some(_) => {
                return Ok(GradingResult {
                    passed: false,
                    message: "tool_calls must be an array".to_string(),
                    details: vec![],
                    schema_errors: vec!["tool_calls is not an array".to_string()],
                });
            }
            None => {
                return Ok(GradingResult {
                    passed: false,
                    message: "Output must contain a tool_calls array".to_string(),
                    details: vec![],
                    schema_errors: vec!["Missing tool_calls field".to_string()],
                });
            }
        };

        let mut details = Vec::new();
        let mut all_passed = true;
        let mut schema_errors = Vec::new();

        // Load tools schema if specified
        let tools_schema = if let Some(ref schema_path) = exercise.manifest.grader.schema_path {
            let full_path = exercise.grader_path.join(schema_path);
            if full_path.exists() {
                let content = std::fs::read_to_string(&full_path)?;
                Some(serde_json::from_str::<serde_json::Value>(&content)?)
            } else {
                None
            }
        } else {
            None
        };

        // Validate each tool call
        for (i, call) in tool_calls.iter().enumerate() {
            let call_name = format!("tool_call_{}", i);

            // Check required fields
            let tool_name = match call.get("name") {
                Some(serde_json::Value::String(n)) => n.clone(),
                _ => {
                    all_passed = false;
                    details.push(GradingDetail {
                        name: call_name,
                        passed: false,
                        message: "Tool call missing 'name' field".to_string(),
                    });
                    continue;
                }
            };

            let arguments = match call.get("arguments") {
                Some(args) => args,
                None => {
                    all_passed = false;
                    details.push(GradingDetail {
                        name: call_name,
                        passed: false,
                        message: format!("Tool call '{}' missing 'arguments' field", tool_name),
                    });
                    continue;
                }
            };

            // Validate against tools schema if available
            if let Some(ref tools) = tools_schema {
                if let Some(tools_array) = tools.get("tools").and_then(|t| t.as_array()) {
                    let tool_def = tools_array.iter().find(|t| {
                        t.get("name").and_then(|n| n.as_str()) == Some(&tool_name)
                    });

                    match tool_def {
                        Some(def) => {
                            if let Some(params_schema) = def.get("parameters") {
                                match schema::validate_json(arguments, params_schema) {
                                    Ok(()) => {
                                        details.push(GradingDetail {
                                            name: call_name,
                                            passed: true,
                                            message: format!(
                                                "Tool '{}' arguments valid",
                                                tool_name
                                            ),
                                        });
                                    }
                                    Err(errors) => {
                                        all_passed = false;
                                        schema_errors.extend(errors.clone());
                                        details.push(GradingDetail {
                                            name: call_name,
                                            passed: false,
                                            message: format!(
                                                "Tool '{}' arguments invalid: {}",
                                                tool_name,
                                                errors.join("; ")
                                            ),
                                        });
                                    }
                                }
                            } else {
                                details.push(GradingDetail {
                                    name: call_name,
                                    passed: true,
                                    message: format!("Tool '{}' called (no schema)", tool_name),
                                });
                            }
                        }
                        None => {
                            all_passed = false;
                            schema_errors.push(format!("Unknown tool: {}", tool_name));
                            details.push(GradingDetail {
                                name: call_name,
                                passed: false,
                                message: format!("Unknown tool '{}'", tool_name),
                            });
                        }
                    }
                } else {
                    details.push(GradingDetail {
                        name: call_name,
                        passed: true,
                        message: format!("Tool '{}' called", tool_name),
                    });
                }
            } else {
                details.push(GradingDetail {
                    name: call_name,
                    passed: true,
                    message: format!("Tool '{}' called", tool_name),
                });
            }
        }

        // Run invariant scripts if defined
        if !exercise.manifest.grader.invariants.is_empty() {
            let tool_calls_json = serde_json::to_string_pretty(tool_calls)?;

            for invariant_path in &exercise.manifest.grader.invariants {
                let full_path = exercise.grader_path.join(invariant_path);
                if !full_path.exists() {
                    return Err(Error::Grading(GradingError::InvariantScriptNotFound(
                        full_path,
                    )));
                }

                let result = invariant::run_invariant(&full_path, &tool_calls_json)?;
                if !result.passed {
                    all_passed = false;
                }
                details.push(GradingDetail {
                    name: invariant_path.clone(),
                    passed: result.passed,
                    message: result.message,
                });
            }
        }

        let passed_count = details.iter().filter(|d| d.passed).count();
        let total = details.len();

        Ok(GradingResult {
            passed: all_passed,
            message: if all_passed {
                format!("Sandbox validation passed: {}/{} checks", passed_count, total)
            } else {
                format!("Sandbox validation failed: {}/{} checks passed", passed_count, total)
            },
            details,
            schema_errors,
        })
    }

    /// Grade based on reliability (multi-run).
    ///
    /// NOTE: This grader type is deprecated. Multi-run reliability should be
    /// configured via `run.runs` and `run.required_passes` in the exercise
    /// manifest, with a concrete grader type (Schema, Sandbox, etc.).
    /// The ExerciseRunner handles multi-run orchestration.
    fn grade_reliability(&self, _exercise: &Exercise, _output: &str) -> Result<GradingResult> {
        Err(Error::Grading(GradingError::NotImplemented(
            "GraderType::Reliability is deprecated. Use a concrete grader type (schema, sandbox, \
             combined, invariants) with run.runs > 1 in manifest.toml for multi-run reliability. \
             The runner handles multi-run orchestration automatically."
                .to_string(),
        )))
    }

    /// Grade using LLM-as-judge (last resort).
    ///
    /// This grading method uses an LLM to evaluate the output against a rubric.
    /// It should only be used when deterministic grading is not possible.
    fn grade_llm_judge(&self, _exercise: &Exercise, _output: &str) -> Result<GradingResult> {
        Err(Error::Grading(GradingError::NotImplemented(
            "LLM-as-judge grading is not yet implemented. This is intentionally a low priority \
             as deterministic grading (schema, sandbox, invariants) is preferred. If you need \
             this feature, please open an issue on the vibelings repository."
                .to_string(),
        )))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::exercise::{
        Exercise, ExerciseManifest, ExerciseMetadata, ExerciseRequirements, ExerciseRunConfig,
        ExerciseStatus, GraderConfig, GraderType, Track,
    };
    use tempfile::TempDir;

    #[test]
    fn test_grader_creation() {
        let grader = Grader::new();
        assert!(grader.is_ok());
    }

    fn create_test_exercise(grader_type: GraderType, temp_dir: &TempDir) -> Exercise {
        let path = temp_dir.path().to_path_buf();
        let grader_path = path.join("grader");
        std::fs::create_dir_all(&grader_path).unwrap();

        Exercise {
            manifest: ExerciseManifest {
                exercise: ExerciseMetadata {
                    id: "test_exercise".to_string(),
                    title: "Test Exercise".to_string(),
                    track: Track::Fundamentals,
                    prerequisites: vec![],
                    description: None,
                    difficulty: 1,
                },
                requirements: ExerciseRequirements::default(),
                run: ExerciseRunConfig::default(),
                grader: GraderConfig {
                    grader_type,
                    schema_path: None,
                    invariants: vec![],
                    rubric_path: None,
                },
            },
            path: path.clone(),
            status: ExerciseStatus::Pending,
            readme_path: path.join("README.md"),
            starter_path: path.join("starter"),
            grader_path,
            fixtures_path: None,
        }
    }

    #[test]
    fn test_sandbox_grading_valid_tool_calls() {
        let grader = Grader::new().unwrap();
        let temp_dir = TempDir::new().unwrap();
        let exercise = create_test_exercise(GraderType::Sandbox, &temp_dir);

        let output = r#"{
            "tool_calls": [
                {
                    "name": "get_weather",
                    "arguments": {"location": "San Francisco"}
                }
            ]
        }"#;

        let result = grader.grade(&exercise, output).unwrap();
        assert!(result.passed);
        assert!(result.message.contains("passed"));
    }

    #[test]
    fn test_sandbox_grading_missing_tool_calls() {
        let grader = Grader::new().unwrap();
        let temp_dir = TempDir::new().unwrap();
        let exercise = create_test_exercise(GraderType::Sandbox, &temp_dir);

        let output = r#"{"result": "no tools here"}"#;

        let result = grader.grade(&exercise, output).unwrap();
        assert!(!result.passed);
        assert!(result.message.contains("tool_calls"));
    }

    #[test]
    fn test_sandbox_grading_invalid_json() {
        let grader = Grader::new().unwrap();
        let temp_dir = TempDir::new().unwrap();
        let exercise = create_test_exercise(GraderType::Sandbox, &temp_dir);

        let output = "not valid json at all";

        let result = grader.grade(&exercise, output).unwrap();
        assert!(!result.passed);
        assert!(result.message.contains("not valid JSON"));
    }

    #[test]
    fn test_sandbox_grading_tool_calls_not_array() {
        let grader = Grader::new().unwrap();
        let temp_dir = TempDir::new().unwrap();
        let exercise = create_test_exercise(GraderType::Sandbox, &temp_dir);

        let output = r#"{"tool_calls": "not an array"}"#;

        let result = grader.grade(&exercise, output).unwrap();
        assert!(!result.passed);
        assert!(result.message.contains("must be an array"));
    }

    #[test]
    fn test_sandbox_grading_missing_tool_name() {
        let grader = Grader::new().unwrap();
        let temp_dir = TempDir::new().unwrap();
        let exercise = create_test_exercise(GraderType::Sandbox, &temp_dir);

        let output = r#"{
            "tool_calls": [
                {"arguments": {"location": "NYC"}}
            ]
        }"#;

        let result = grader.grade(&exercise, output).unwrap();
        assert!(!result.passed);
        assert!(result.details.iter().any(|d| d.message.contains("missing 'name'")));
    }

    #[test]
    fn test_sandbox_grading_missing_arguments() {
        let grader = Grader::new().unwrap();
        let temp_dir = TempDir::new().unwrap();
        let exercise = create_test_exercise(GraderType::Sandbox, &temp_dir);

        let output = r#"{
            "tool_calls": [
                {"name": "get_weather"}
            ]
        }"#;

        let result = grader.grade(&exercise, output).unwrap();
        assert!(!result.passed);
        assert!(result.details.iter().any(|d| d.message.contains("missing 'arguments'")));
    }

    #[test]
    fn test_sandbox_grading_with_schema_validation() {
        let grader = Grader::new().unwrap();
        let temp_dir = TempDir::new().unwrap();

        // Create exercise with schema
        let path = temp_dir.path().to_path_buf();
        let grader_path = path.join("grader");
        std::fs::create_dir_all(&grader_path).unwrap();

        // Write a tools schema file
        let schema = r#"{
            "tools": [
                {
                    "name": "get_weather",
                    "parameters": {
                        "type": "object",
                        "properties": {
                            "location": {"type": "string", "minLength": 1}
                        },
                        "required": ["location"]
                    }
                }
            ]
        }"#;
        std::fs::write(grader_path.join("tools.json"), schema).unwrap();

        let exercise = Exercise {
            manifest: ExerciseManifest {
                exercise: ExerciseMetadata {
                    id: "test_exercise".to_string(),
                    title: "Test Exercise".to_string(),
                    track: Track::Fundamentals,
                    prerequisites: vec![],
                    description: None,
                    difficulty: 1,
                },
                requirements: ExerciseRequirements::default(),
                run: ExerciseRunConfig::default(),
                grader: GraderConfig {
                    grader_type: GraderType::Sandbox,
                    schema_path: Some("tools.json".to_string()),
                    invariants: vec![],
                    rubric_path: None,
                },
            },
            path: path.clone(),
            status: ExerciseStatus::Pending,
            readme_path: path.join("README.md"),
            starter_path: path.join("starter"),
            grader_path,
            fixtures_path: None,
        };

        // Valid tool call
        let output = r#"{
            "tool_calls": [
                {"name": "get_weather", "arguments": {"location": "Boston"}}
            ]
        }"#;
        let result = grader.grade(&exercise, output).unwrap();
        assert!(result.passed);

        // Invalid tool (unknown)
        let output_unknown = r#"{
            "tool_calls": [
                {"name": "unknown_tool", "arguments": {"x": 1}}
            ]
        }"#;
        let result_unknown = grader.grade(&exercise, output_unknown).unwrap();
        assert!(!result_unknown.passed);
        assert!(result_unknown.details.iter().any(|d| d.message.contains("Unknown tool")));

        // Invalid arguments (missing required)
        let output_missing = r#"{
            "tool_calls": [
                {"name": "get_weather", "arguments": {}}
            ]
        }"#;
        let result_missing = grader.grade(&exercise, output_missing).unwrap();
        assert!(!result_missing.passed);
    }

    #[test]
    fn test_sandbox_grading_empty_tool_calls() {
        let grader = Grader::new().unwrap();
        let temp_dir = TempDir::new().unwrap();
        let exercise = create_test_exercise(GraderType::Sandbox, &temp_dir);

        let output = r#"{"tool_calls": []}"#;

        let result = grader.grade(&exercise, output).unwrap();
        // Empty tool_calls is valid - just no operations
        assert!(result.passed);
        assert!(result.details.is_empty());
    }

    #[test]
    fn test_sandbox_grading_multiple_valid_tools() {
        let grader = Grader::new().unwrap();
        let temp_dir = TempDir::new().unwrap();
        let exercise = create_test_exercise(GraderType::Sandbox, &temp_dir);

        let output = r#"{
            "tool_calls": [
                {"name": "read_file", "arguments": {"path": "/tmp/test.txt"}},
                {"name": "write_file", "arguments": {"path": "/tmp/out.txt", "content": "hello"}},
                {"name": "run_tests", "arguments": {}}
            ]
        }"#;

        let result = grader.grade(&exercise, output).unwrap();
        assert!(result.passed);
        assert_eq!(result.details.len(), 3);
    }

    #[test]
    fn test_reliability_grader_returns_error() {
        let grader = Grader::new().unwrap();
        let temp_dir = TempDir::new().unwrap();
        let exercise = create_test_exercise(GraderType::Reliability, &temp_dir);

        let result = grader.grade(&exercise, "any output");
        assert!(result.is_err());

        let err = result.unwrap_err();
        let err_msg = err.to_string();
        assert!(err_msg.contains("not implemented") || err_msg.contains("deprecated"));
    }

    #[test]
    fn test_llm_judge_grader_returns_error() {
        let grader = Grader::new().unwrap();
        let temp_dir = TempDir::new().unwrap();
        let exercise = create_test_exercise(GraderType::LlmJudge, &temp_dir);

        let result = grader.grade(&exercise, "any output");
        assert!(result.is_err());

        let err = result.unwrap_err();
        let err_msg = err.to_string();
        assert!(err_msg.contains("not implemented") || err_msg.contains("not yet implemented"));
    }
}
