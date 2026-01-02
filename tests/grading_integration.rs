//! Exercise grading integration tests.
//!
//! Tests that verify the grading system works correctly with actual exercises.

use std::fs;
use std::path::PathBuf;

use vibelings::grader::Grader;
use vibelings::{Exercise, ExerciseManifest, ExerciseStatus, GraderType, Track};

/// Helper to get the exercises directory path.
fn exercises_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("exercises")
}

/// Load an exercise from the real exercises directory.
fn load_exercise(track: &str, id: &str) -> Exercise {
    let exercise_path = exercises_dir().join(track).join(id);
    let manifest_path = exercise_path.join("manifest.toml");
    let manifest_content = fs::read_to_string(&manifest_path)
        .unwrap_or_else(|_| panic!("Failed to read manifest at {:?}", manifest_path));
    let manifest: ExerciseManifest = toml::from_str(&manifest_content)
        .unwrap_or_else(|e| panic!("Failed to parse manifest: {}", e));

    Exercise {
        manifest,
        path: exercise_path.clone(),
        status: ExerciseStatus::Pending,
        readme_path: exercise_path.join("README.md"),
        starter_path: exercise_path.join("starter"),
        grader_path: exercise_path.join("grader"),
        fixtures_path: None,
    }
}

// =============================================================================
// JSON_01 Tests - Basic JSON Schema Validation
// =============================================================================

#[test]
fn test_json_01_valid_person() {
    let exercise = load_exercise("fundamentals", "json_01");
    let grader = Grader::new().unwrap();

    let valid_output = r#"{
        "name": "Alice Smith",
        "age": 30,
        "email": "alice@example.com"
    }"#;

    let result = grader.grade(&exercise, valid_output).unwrap();
    assert!(
        result.passed,
        "Expected valid JSON to pass: {}",
        result.message
    );
}

#[test]
fn test_json_01_valid_person_with_occupation() {
    let exercise = load_exercise("fundamentals", "json_01");
    let grader = Grader::new().unwrap();

    let valid_output = r#"{
        "name": "Bob Jones",
        "age": 45,
        "email": "bob@company.org",
        "occupation": "Software Engineer"
    }"#;

    let result = grader.grade(&exercise, valid_output).unwrap();
    assert!(
        result.passed,
        "Expected valid JSON with occupation to pass: {}",
        result.message
    );
}

#[test]
fn test_json_01_missing_required_field() {
    let exercise = load_exercise("fundamentals", "json_01");
    let grader = Grader::new().unwrap();

    // Missing email field
    let invalid_output = r#"{
        "name": "Charlie",
        "age": 25
    }"#;

    let result = grader.grade(&exercise, invalid_output).unwrap();
    assert!(!result.passed, "Expected missing required field to fail");
    assert!(
        result.message.contains("validation failed"),
        "Expected validation error message"
    );
}

#[test]
fn test_json_01_invalid_email_format() {
    let exercise = load_exercise("fundamentals", "json_01");
    let grader = Grader::new().unwrap();

    let invalid_output = r#"{
        "name": "Diana",
        "age": 28,
        "email": "not-an-email"
    }"#;

    let result = grader.grade(&exercise, invalid_output).unwrap();
    assert!(!result.passed, "Expected invalid email format to fail");
}

#[test]
fn test_json_01_invalid_age_type() {
    let exercise = load_exercise("fundamentals", "json_01");
    let grader = Grader::new().unwrap();

    let invalid_output = r#"{
        "name": "Eve",
        "age": "thirty",
        "email": "eve@example.com"
    }"#;

    let result = grader.grade(&exercise, invalid_output).unwrap();
    assert!(!result.passed, "Expected wrong type for age to fail");
}

#[test]
fn test_json_01_age_out_of_range() {
    let exercise = load_exercise("fundamentals", "json_01");
    let grader = Grader::new().unwrap();

    let invalid_output = r#"{
        "name": "Frank",
        "age": 200,
        "email": "frank@example.com"
    }"#;

    let result = grader.grade(&exercise, invalid_output).unwrap();
    assert!(!result.passed, "Expected age > 150 to fail validation");
}

#[test]
fn test_json_01_extra_properties_not_allowed() {
    let exercise = load_exercise("fundamentals", "json_01");
    let grader = Grader::new().unwrap();

    let invalid_output = r#"{
        "name": "Grace",
        "age": 35,
        "email": "grace@example.com",
        "extra_field": "not allowed"
    }"#;

    let result = grader.grade(&exercise, invalid_output).unwrap();
    assert!(
        !result.passed,
        "Expected extra properties to fail (additionalProperties: false)"
    );
}

#[test]
fn test_json_01_not_json() {
    let exercise = load_exercise("fundamentals", "json_01");
    let grader = Grader::new().unwrap();

    let invalid_output = "This is just plain text, not JSON at all.";

    let result = grader.grade(&exercise, invalid_output).unwrap();
    assert!(!result.passed, "Expected non-JSON to fail");
    assert!(
        result.message.contains("not valid JSON"),
        "Expected JSON parse error message"
    );
}

// =============================================================================
// JSON_02 Tests - Nested JSON Structures
// =============================================================================

#[test]
fn test_json_02_valid_team() {
    let exercise = load_exercise("fundamentals", "json_02");
    let grader = Grader::new().unwrap();

    let valid_output = r#"{
        "name": "Platform Team",
        "department": "Engineering",
        "members": [
            {
                "name": "Alice",
                "role": "lead",
                "skills": ["rust", "python"]
            },
            {
                "name": "Bob",
                "role": "senior",
                "skills": ["javascript"]
            }
        ]
    }"#;

    let result = grader.grade(&exercise, valid_output).unwrap();
    assert!(
        result.passed,
        "Expected valid nested JSON to pass: {}",
        result.message
    );
}

#[test]
fn test_json_02_with_current_project() {
    let exercise = load_exercise("fundamentals", "json_02");
    let grader = Grader::new().unwrap();

    let valid_output = r#"{
        "name": "ML Team",
        "department": "Research",
        "members": [
            {
                "name": "Carol",
                "role": "junior",
                "skills": ["python", "tensorflow"]
            }
        ],
        "currentProject": {
            "name": "Model Optimization",
            "deadline": "2024-12-31",
            "status": "active"
        }
    }"#;

    let result = grader.grade(&exercise, valid_output).unwrap();
    assert!(
        result.passed,
        "Expected valid team with project to pass: {}",
        result.message
    );
}

#[test]
fn test_json_02_empty_members_array() {
    let exercise = load_exercise("fundamentals", "json_02");
    let grader = Grader::new().unwrap();

    let invalid_output = r#"{
        "name": "Empty Team",
        "department": "Engineering",
        "members": []
    }"#;

    let result = grader.grade(&exercise, invalid_output).unwrap();
    assert!(
        !result.passed,
        "Expected empty members array to fail (minItems: 1)"
    );
}

#[test]
fn test_json_02_invalid_role() {
    let exercise = load_exercise("fundamentals", "json_02");
    let grader = Grader::new().unwrap();

    let invalid_output = r#"{
        "name": "Test Team",
        "department": "QA",
        "members": [
            {
                "name": "Dave",
                "role": "intern",
                "skills": ["testing"]
            }
        ]
    }"#;

    let result = grader.grade(&exercise, invalid_output).unwrap();
    assert!(!result.passed, "Expected invalid role enum value to fail");
}

#[test]
fn test_json_02_invalid_deadline_format() {
    let exercise = load_exercise("fundamentals", "json_02");
    let grader = Grader::new().unwrap();

    let invalid_output = r#"{
        "name": "Test Team",
        "department": "Engineering",
        "members": [
            {
                "name": "Eve",
                "role": "lead",
                "skills": ["management"]
            }
        ],
        "currentProject": {
            "name": "Release",
            "deadline": "December 2024",
            "status": "planning"
        }
    }"#;

    let result = grader.grade(&exercise, invalid_output).unwrap();
    assert!(
        !result.passed,
        "Expected invalid deadline format to fail (must be YYYY-MM-DD)"
    );
}

// =============================================================================
// TOOLS_01 Tests - Sandbox/Tool Calling Validation
// =============================================================================

#[test]
fn test_tools_01_valid_weather_call() {
    let exercise = load_exercise("fundamentals", "tools_01");
    let grader = Grader::new().unwrap();

    let valid_output = r#"{
        "tool_calls": [
            {
                "name": "get_weather",
                "arguments": {
                    "location": "San Francisco"
                }
            }
        ]
    }"#;

    let result = grader.grade(&exercise, valid_output).unwrap();
    assert!(
        result.passed,
        "Expected valid tool call to pass: {}",
        result.message
    );
}

#[test]
fn test_tools_01_valid_weather_with_units() {
    let exercise = load_exercise("fundamentals", "tools_01");
    let grader = Grader::new().unwrap();

    let valid_output = r#"{
        "tool_calls": [
            {
                "name": "get_weather",
                "arguments": {
                    "location": "New York",
                    "units": "fahrenheit"
                }
            }
        ]
    }"#;

    let result = grader.grade(&exercise, valid_output).unwrap();
    assert!(
        result.passed,
        "Expected valid tool call with units to pass: {}",
        result.message
    );
}

#[test]
fn test_tools_01_valid_forecast_call() {
    let exercise = load_exercise("fundamentals", "tools_01");
    let grader = Grader::new().unwrap();

    let valid_output = r#"{
        "tool_calls": [
            {
                "name": "get_forecast",
                "arguments": {
                    "location": "Seattle",
                    "days": 5
                }
            }
        ]
    }"#;

    let result = grader.grade(&exercise, valid_output).unwrap();
    assert!(
        result.passed,
        "Expected valid forecast call to pass: {}",
        result.message
    );
}

#[test]
fn test_tools_01_multiple_tool_calls() {
    let exercise = load_exercise("fundamentals", "tools_01");
    let grader = Grader::new().unwrap();

    let valid_output = r#"{
        "tool_calls": [
            {
                "name": "get_weather",
                "arguments": {"location": "London"}
            },
            {
                "name": "get_forecast",
                "arguments": {"location": "London", "days": 3, "units": "celsius"}
            }
        ]
    }"#;

    let result = grader.grade(&exercise, valid_output).unwrap();
    assert!(
        result.passed,
        "Expected multiple valid tool calls to pass: {}",
        result.message
    );
}

#[test]
fn test_tools_01_unknown_tool() {
    let exercise = load_exercise("fundamentals", "tools_01");
    let grader = Grader::new().unwrap();

    let invalid_output = r#"{
        "tool_calls": [
            {
                "name": "send_email",
                "arguments": {"to": "test@example.com"}
            }
        ]
    }"#;

    let result = grader.grade(&exercise, invalid_output).unwrap();
    assert!(!result.passed, "Expected unknown tool to fail");
    assert!(
        result.details.iter().any(|d| d.message.contains("Unknown")),
        "Expected 'Unknown tool' in error message"
    );
}

#[test]
fn test_tools_01_missing_required_argument() {
    let exercise = load_exercise("fundamentals", "tools_01");
    let grader = Grader::new().unwrap();

    // get_forecast requires 'days' parameter
    let invalid_output = r#"{
        "tool_calls": [
            {
                "name": "get_forecast",
                "arguments": {
                    "location": "Paris"
                }
            }
        ]
    }"#;

    let result = grader.grade(&exercise, invalid_output).unwrap();
    assert!(!result.passed, "Expected missing required argument to fail");
}

#[test]
fn test_tools_01_invalid_enum_value() {
    let exercise = load_exercise("fundamentals", "tools_01");
    let grader = Grader::new().unwrap();

    let invalid_output = r#"{
        "tool_calls": [
            {
                "name": "get_weather",
                "arguments": {
                    "location": "Tokyo",
                    "units": "kelvin"
                }
            }
        ]
    }"#;

    let result = grader.grade(&exercise, invalid_output).unwrap();
    assert!(
        !result.passed,
        "Expected invalid enum value for units to fail"
    );
}

#[test]
fn test_tools_01_days_out_of_range() {
    let exercise = load_exercise("fundamentals", "tools_01");
    let grader = Grader::new().unwrap();

    let invalid_output = r#"{
        "tool_calls": [
            {
                "name": "get_forecast",
                "arguments": {
                    "location": "Berlin",
                    "days": 10
                }
            }
        ]
    }"#;

    let result = grader.grade(&exercise, invalid_output).unwrap();
    assert!(!result.passed, "Expected days > 7 to fail (maximum: 7)");
}

#[test]
fn test_tools_01_extra_argument_not_allowed() {
    let exercise = load_exercise("fundamentals", "tools_01");
    let grader = Grader::new().unwrap();

    let invalid_output = r#"{
        "tool_calls": [
            {
                "name": "get_weather",
                "arguments": {
                    "location": "Sydney",
                    "extra_param": "not allowed"
                }
            }
        ]
    }"#;

    let result = grader.grade(&exercise, invalid_output).unwrap();
    assert!(
        !result.passed,
        "Expected extra parameter to fail (additionalProperties: false)"
    );
}

// =============================================================================
// Grader Type Tests
// =============================================================================

#[test]
fn test_exercise_grader_type_is_schema() {
    let exercise = load_exercise("fundamentals", "json_01");
    assert_eq!(
        exercise.manifest.grader.grader_type,
        GraderType::Schema,
        "json_01 should use schema grader"
    );
}

#[test]
fn test_exercise_grader_type_is_sandbox() {
    let exercise = load_exercise("fundamentals", "tools_01");
    assert_eq!(
        exercise.manifest.grader.grader_type,
        GraderType::Sandbox,
        "tools_01 should use sandbox grader"
    );
}

// =============================================================================
// Exercise Metadata Tests
// =============================================================================

#[test]
fn test_exercise_prerequisites() {
    let exercise = load_exercise("fundamentals", "json_02");
    assert!(
        exercise
            .manifest
            .exercise
            .prerequisites
            .contains(&"json_01".to_string()),
        "json_02 should have json_01 as prerequisite"
    );
}

#[test]
fn test_exercise_track() {
    let exercise = load_exercise("fundamentals", "json_01");
    assert_eq!(
        exercise.manifest.exercise.track,
        Track::Fundamentals,
        "Exercise should be in fundamentals track"
    );
}
