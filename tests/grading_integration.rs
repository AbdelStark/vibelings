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
// JSON_03 Tests - Array Contracts
// =============================================================================

#[test]
fn test_json_03_valid_schedule() {
    let exercise = load_exercise("fundamentals", "json_03");
    let grader = Grader::new().unwrap();

    let valid_output = r#"{
        "event_name": "AI Engineering Summit 2025",
        "date": "2025-06-15",
        "sessions": [
            {
                "id": "S001",
                "title": "Building Reliable Agentic Systems",
                "speaker": "Dr. Sarah Chen",
                "time_slot": "09:00",
                "duration_minutes": 60,
                "track": "keynote"
            },
            {
                "id": "S002",
                "title": "MCP Workshop",
                "speaker": "Alex Rivera",
                "time_slot": "10:30",
                "duration_minutes": 90,
                "track": "workshop"
            }
        ]
    }"#;

    let result = grader.grade(&exercise, valid_output).unwrap();
    assert!(
        result.passed,
        "Expected valid schedule to pass: {}",
        result.message
    );
}

#[test]
fn test_json_03_too_few_sessions() {
    let exercise = load_exercise("fundamentals", "json_03");
    let grader = Grader::new().unwrap();

    let invalid_output = r#"{
        "event_name": "Small Event",
        "date": "2025-06-15",
        "sessions": [
            {
                "id": "S001",
                "title": "Only Session",
                "speaker": "Solo Speaker",
                "time_slot": "09:00",
                "duration_minutes": 60,
                "track": "keynote"
            }
        ]
    }"#;

    let result = grader.grade(&exercise, invalid_output).unwrap();
    assert!(
        !result.passed,
        "Expected single session to fail (minItems: 2)"
    );
}

#[test]
fn test_json_03_invalid_session_id_format() {
    let exercise = load_exercise("fundamentals", "json_03");
    let grader = Grader::new().unwrap();

    let invalid_output = r#"{
        "event_name": "Test Event",
        "date": "2025-06-15",
        "sessions": [
            {
                "id": "session-1",
                "title": "Session One",
                "speaker": "Speaker 1",
                "time_slot": "09:00",
                "duration_minutes": 60,
                "track": "keynote"
            },
            {
                "id": "S002",
                "title": "Session Two",
                "speaker": "Speaker 2",
                "time_slot": "10:00",
                "duration_minutes": 60,
                "track": "technical"
            }
        ]
    }"#;

    let result = grader.grade(&exercise, invalid_output).unwrap();
    assert!(
        !result.passed,
        "Expected invalid session id format to fail (pattern: SXXX)"
    );
}

#[test]
fn test_json_03_invalid_track_enum() {
    let exercise = load_exercise("fundamentals", "json_03");
    let grader = Grader::new().unwrap();

    let invalid_output = r#"{
        "event_name": "Test Event",
        "date": "2025-06-15",
        "sessions": [
            {
                "id": "S001",
                "title": "Session One",
                "speaker": "Speaker 1",
                "time_slot": "09:00",
                "duration_minutes": 60,
                "track": "keynote"
            },
            {
                "id": "S002",
                "title": "Session Two",
                "speaker": "Speaker 2",
                "time_slot": "10:00",
                "duration_minutes": 60,
                "track": "panel"
            }
        ]
    }"#;

    let result = grader.grade(&exercise, invalid_output).unwrap();
    assert!(!result.passed, "Expected invalid track enum value to fail");
}

#[test]
fn test_json_03_duration_out_of_range() {
    let exercise = load_exercise("fundamentals", "json_03");
    let grader = Grader::new().unwrap();

    let invalid_output = r#"{
        "event_name": "Test Event",
        "date": "2025-06-15",
        "sessions": [
            {
                "id": "S001",
                "title": "Session One",
                "speaker": "Speaker 1",
                "time_slot": "09:00",
                "duration_minutes": 180,
                "track": "workshop"
            },
            {
                "id": "S002",
                "title": "Session Two",
                "speaker": "Speaker 2",
                "time_slot": "10:00",
                "duration_minutes": 60,
                "track": "technical"
            }
        ]
    }"#;

    let result = grader.grade(&exercise, invalid_output).unwrap();
    assert!(!result.passed, "Expected duration > 120 to fail (max: 120)");
}

#[test]
fn test_json_03_invalid_time_format() {
    let exercise = load_exercise("fundamentals", "json_03");
    let grader = Grader::new().unwrap();

    let invalid_output = r#"{
        "event_name": "Test Event",
        "date": "2025-06-15",
        "sessions": [
            {
                "id": "S001",
                "title": "Session One",
                "speaker": "Speaker 1",
                "time_slot": "9:00am",
                "duration_minutes": 60,
                "track": "keynote"
            },
            {
                "id": "S002",
                "title": "Session Two",
                "speaker": "Speaker 2",
                "time_slot": "10:00",
                "duration_minutes": 60,
                "track": "technical"
            }
        ]
    }"#;

    let result = grader.grade(&exercise, invalid_output).unwrap();
    assert!(
        !result.passed,
        "Expected invalid time format to fail (must be HH:MM)"
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
// MCP_SERVER_01 Tests - MCP Tool Definition
// =============================================================================

#[test]
fn test_mcp_server_01_valid_tool_definition() {
    let exercise = load_exercise("mcp", "server_01");
    let grader = Grader::new().unwrap();

    let valid_output = r#"{
        "name": "calculate_area",
        "description": "Calculate the area of a geometric shape",
        "inputSchema": {
            "type": "object",
            "properties": {
                "shape": {
                    "type": "string",
                    "enum": ["circle", "rectangle", "triangle"],
                    "description": "The type of shape"
                },
                "radius": {
                    "type": "number",
                    "description": "Radius for circle"
                },
                "width": {
                    "type": "number",
                    "description": "Width for rectangle"
                },
                "height": {
                    "type": "number",
                    "description": "Height for rectangle or triangle"
                },
                "base": {
                    "type": "number",
                    "description": "Base for triangle"
                }
            },
            "required": ["shape"]
        }
    }"#;

    let result = grader.grade(&exercise, valid_output).unwrap();
    assert!(
        result.passed,
        "Expected valid MCP tool definition to pass: {}",
        result.message
    );
}

#[test]
fn test_mcp_server_01_wrong_tool_name() {
    let exercise = load_exercise("mcp", "server_01");
    let grader = Grader::new().unwrap();

    let invalid_output = r#"{
        "name": "compute_area",
        "description": "Calculate the area of a geometric shape",
        "inputSchema": {
            "type": "object",
            "properties": {
                "shape": {
                    "type": "string",
                    "enum": ["circle", "rectangle", "triangle"]
                },
                "radius": { "type": "number" },
                "width": { "type": "number" },
                "height": { "type": "number" },
                "base": { "type": "number" }
            },
            "required": ["shape"]
        }
    }"#;

    let result = grader.grade(&exercise, invalid_output).unwrap();
    assert!(!result.passed, "Expected wrong tool name to fail");
}

#[test]
fn test_mcp_server_01_missing_input_schema() {
    let exercise = load_exercise("mcp", "server_01");
    let grader = Grader::new().unwrap();

    let invalid_output = r#"{
        "name": "calculate_area",
        "description": "Calculate the area of a geometric shape"
    }"#;

    let result = grader.grade(&exercise, invalid_output).unwrap();
    assert!(!result.passed, "Expected missing inputSchema to fail");
}

#[test]
fn test_mcp_server_01_using_parameters_instead_of_input_schema() {
    let exercise = load_exercise("mcp", "server_01");
    let grader = Grader::new().unwrap();

    // Using OpenAI-style "parameters" instead of MCP-style "inputSchema"
    let invalid_output = r#"{
        "name": "calculate_area",
        "description": "Calculate the area of a geometric shape",
        "parameters": {
            "type": "object",
            "properties": {
                "shape": {
                    "type": "string",
                    "enum": ["circle", "rectangle", "triangle"]
                },
                "radius": { "type": "number" },
                "width": { "type": "number" },
                "height": { "type": "number" },
                "base": { "type": "number" }
            },
            "required": ["shape"]
        }
    }"#;

    let result = grader.grade(&exercise, invalid_output).unwrap();
    assert!(
        !result.passed,
        "Expected using 'parameters' instead of 'inputSchema' to fail"
    );
}

#[test]
fn test_mcp_server_01_missing_required_property() {
    let exercise = load_exercise("mcp", "server_01");
    let grader = Grader::new().unwrap();

    // Missing the 'base' property
    let invalid_output = r#"{
        "name": "calculate_area",
        "description": "Calculate the area of a geometric shape",
        "inputSchema": {
            "type": "object",
            "properties": {
                "shape": {
                    "type": "string",
                    "enum": ["circle", "rectangle", "triangle"]
                },
                "radius": { "type": "number" },
                "width": { "type": "number" },
                "height": { "type": "number" }
            },
            "required": ["shape"]
        }
    }"#;

    let result = grader.grade(&exercise, invalid_output).unwrap();
    assert!(!result.passed, "Expected missing base property to fail");
}

#[test]
fn test_mcp_server_01_wrong_shape_enum() {
    let exercise = load_exercise("mcp", "server_01");
    let grader = Grader::new().unwrap();

    // Using wrong enum values for shape
    let invalid_output = r#"{
        "name": "calculate_area",
        "description": "Calculate the area of a geometric shape",
        "inputSchema": {
            "type": "object",
            "properties": {
                "shape": {
                    "type": "string",
                    "enum": ["square", "pentagon", "hexagon"]
                },
                "radius": { "type": "number" },
                "width": { "type": "number" },
                "height": { "type": "number" },
                "base": { "type": "number" }
            },
            "required": ["shape"]
        }
    }"#;

    let result = grader.grade(&exercise, invalid_output).unwrap();
    assert!(!result.passed, "Expected wrong shape enum values to fail");
}

#[test]
fn test_mcp_server_01_mcp_track() {
    let exercise = load_exercise("mcp", "server_01");
    assert_eq!(
        exercise.manifest.exercise.track,
        Track::Mcp,
        "Exercise should be in MCP track"
    );
}

// =============================================================================
// MCP_CLIENT_01 Tests - MCP JSON-RPC Tool Call Request
// =============================================================================

#[test]
fn test_mcp_client_01_valid_request() {
    let exercise = load_exercise("mcp", "client_01");
    let grader = Grader::new().unwrap();

    let valid_output = r#"{
        "jsonrpc": "2.0",
        "id": "calc-001",
        "method": "tools/call",
        "params": {
            "name": "calculate_area",
            "arguments": {
                "shape": "circle",
                "radius": 5
            }
        }
    }"#;

    let result = grader.grade(&exercise, valid_output).unwrap();
    assert!(
        result.passed,
        "Expected valid MCP request to pass: {}",
        result.message
    );
}

#[test]
fn test_mcp_client_01_numeric_id() {
    let exercise = load_exercise("mcp", "client_01");
    let grader = Grader::new().unwrap();

    let valid_output = r#"{
        "jsonrpc": "2.0",
        "id": 42,
        "method": "tools/call",
        "params": {
            "name": "calculate_area",
            "arguments": {
                "shape": "circle",
                "radius": 10
            }
        }
    }"#;

    let result = grader.grade(&exercise, valid_output).unwrap();
    assert!(
        result.passed,
        "Expected numeric id to pass: {}",
        result.message
    );
}

#[test]
fn test_mcp_client_01_wrong_jsonrpc_version() {
    let exercise = load_exercise("mcp", "client_01");
    let grader = Grader::new().unwrap();

    let invalid_output = r#"{
        "jsonrpc": "1.0",
        "id": "test",
        "method": "tools/call",
        "params": {
            "name": "calculate_area",
            "arguments": {
                "shape": "circle",
                "radius": 5
            }
        }
    }"#;

    let result = grader.grade(&exercise, invalid_output).unwrap();
    assert!(!result.passed, "Expected wrong jsonrpc version to fail");
}

#[test]
fn test_mcp_client_01_wrong_method() {
    let exercise = load_exercise("mcp", "client_01");
    let grader = Grader::new().unwrap();

    let invalid_output = r#"{
        "jsonrpc": "2.0",
        "id": "test",
        "method": "call_tool",
        "params": {
            "name": "calculate_area",
            "arguments": {
                "shape": "circle",
                "radius": 5
            }
        }
    }"#;

    let result = grader.grade(&exercise, invalid_output).unwrap();
    assert!(!result.passed, "Expected wrong method to fail");
}

#[test]
fn test_mcp_client_01_missing_id() {
    let exercise = load_exercise("mcp", "client_01");
    let grader = Grader::new().unwrap();

    let invalid_output = r#"{
        "jsonrpc": "2.0",
        "method": "tools/call",
        "params": {
            "name": "calculate_area",
            "arguments": {
                "shape": "circle",
                "radius": 5
            }
        }
    }"#;

    let result = grader.grade(&exercise, invalid_output).unwrap();
    assert!(!result.passed, "Expected missing id to fail");
}

#[test]
fn test_mcp_client_01_wrong_shape() {
    let exercise = load_exercise("mcp", "client_01");
    let grader = Grader::new().unwrap();

    let invalid_output = r#"{
        "jsonrpc": "2.0",
        "id": "test",
        "method": "tools/call",
        "params": {
            "name": "calculate_area",
            "arguments": {
                "shape": "square",
                "radius": 5
            }
        }
    }"#;

    let result = grader.grade(&exercise, invalid_output).unwrap();
    assert!(
        !result.passed,
        "Expected wrong shape to fail (must be circle)"
    );
}

#[test]
fn test_mcp_client_01_string_radius() {
    let exercise = load_exercise("mcp", "client_01");
    let grader = Grader::new().unwrap();

    let invalid_output = r#"{
        "jsonrpc": "2.0",
        "id": "test",
        "method": "tools/call",
        "params": {
            "name": "calculate_area",
            "arguments": {
                "shape": "circle",
                "radius": "five"
            }
        }
    }"#;

    let result = grader.grade(&exercise, invalid_output).unwrap();
    assert!(
        !result.passed,
        "Expected string radius to fail (must be number)"
    );
}

#[test]
fn test_mcp_client_01_negative_radius() {
    let exercise = load_exercise("mcp", "client_01");
    let grader = Grader::new().unwrap();

    let invalid_output = r#"{
        "jsonrpc": "2.0",
        "id": "test",
        "method": "tools/call",
        "params": {
            "name": "calculate_area",
            "arguments": {
                "shape": "circle",
                "radius": -5
            }
        }
    }"#;

    let result = grader.grade(&exercise, invalid_output).unwrap();
    assert!(!result.passed, "Expected negative radius to fail");
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

// =============================================================================
// WORKFLOW_JSON_01 Tests - Workflow JSON Schema
// =============================================================================

#[test]
fn test_workflow_json_01_valid_workflow() {
    let exercise = load_exercise("workflows", "workflow_json_01");
    let grader = Grader::new().unwrap();

    let valid_output = r#"{
        "name": "User Signup Processing",
        "nodes": [
            {
                "id": "webhook-1",
                "name": "Signup Webhook",
                "type": "n8n-nodes-base.webhook",
                "position": [0, 0],
                "parameters": {"path": "signup"}
            },
            {
                "id": "if-1",
                "name": "Validate Email",
                "type": "n8n-nodes-base.if",
                "position": [200, 0],
                "parameters": {"conditions": {}}
            },
            {
                "id": "postgres-1",
                "name": "Store User",
                "type": "n8n-nodes-base.postgres",
                "position": [400, 0],
                "parameters": {"operation": "insert"}
            }
        ],
        "connections": {
            "Signup Webhook": {
                "main": [[{"node": "Validate Email", "type": "main", "index": 0}]]
            },
            "Validate Email": {
                "main": [[{"node": "Store User", "type": "main", "index": 0}]]
            }
        }
    }"#;

    let result = grader.grade(&exercise, valid_output).unwrap();
    assert!(
        result.passed,
        "Expected valid workflow to pass: {}",
        result.message
    );
}

#[test]
fn test_workflow_json_01_missing_connections() {
    let exercise = load_exercise("workflows", "workflow_json_01");
    let grader = Grader::new().unwrap();

    let invalid_output = r#"{
        "name": "Test Workflow",
        "nodes": [
            {
                "id": "webhook-1",
                "name": "Signup Webhook",
                "type": "n8n-nodes-base.webhook",
                "position": [0, 0],
                "parameters": {}
            },
            {
                "id": "if-1",
                "name": "Validate Email",
                "type": "n8n-nodes-base.if",
                "position": [200, 0],
                "parameters": {}
            },
            {
                "id": "postgres-1",
                "name": "Store User",
                "type": "n8n-nodes-base.postgres",
                "position": [400, 0],
                "parameters": {}
            }
        ],
        "connections": {}
    }"#;

    let result = grader.grade(&exercise, invalid_output).unwrap();
    assert!(!result.passed, "Expected missing connections to fail");
}

#[test]
fn test_workflow_json_01_wrong_node_type() {
    let exercise = load_exercise("workflows", "workflow_json_01");
    let grader = Grader::new().unwrap();

    let invalid_output = r#"{
        "name": "Test Workflow",
        "nodes": [
            {
                "id": "webhook-1",
                "name": "Signup Webhook",
                "type": "n8n-nodes-base.http",
                "position": [0, 0],
                "parameters": {}
            },
            {
                "id": "if-1",
                "name": "Validate Email",
                "type": "n8n-nodes-base.if",
                "position": [200, 0],
                "parameters": {}
            },
            {
                "id": "postgres-1",
                "name": "Store User",
                "type": "n8n-nodes-base.postgres",
                "position": [400, 0],
                "parameters": {}
            }
        ],
        "connections": {
            "Signup Webhook": {
                "main": [[{"node": "Validate Email", "type": "main", "index": 0}]]
            },
            "Validate Email": {
                "main": [[{"node": "Store User", "type": "main", "index": 0}]]
            }
        }
    }"#;

    let result = grader.grade(&exercise, invalid_output).unwrap();
    assert!(!result.passed, "Expected wrong node type to fail");
}

#[test]
fn test_workflow_json_01_workflows_track() {
    let exercise = load_exercise("workflows", "workflow_json_01");
    assert_eq!(
        exercise.manifest.exercise.track,
        Track::Workflows,
        "Exercise should be in workflows track"
    );
}

// =============================================================================
// WORKFLOW_TOOL_WIRING_01 Tests - Tool Wiring Patterns
// =============================================================================

#[test]
fn test_workflow_tool_wiring_01_valid_pipeline() {
    let exercise = load_exercise("workflows", "workflow_tool_wiring_01");
    let grader = Grader::new().unwrap();

    let valid_output = r#"{
        "pipeline": {
            "name": "Order Processing Pipeline",
            "steps": [
                {
                    "id": "fetch_order",
                    "tool": "http_request",
                    "input_mapping": {"url": "{{trigger.order_url}}"},
                    "output_schema": {"order_id": "string"}
                },
                {
                    "id": "transform_data",
                    "tool": "data_transform",
                    "input_mapping": {"source": "{{fetch_order}}"},
                    "output_schema": {"order_id": "string"}
                },
                {
                    "id": "enrich_customer",
                    "tool": "crm_lookup",
                    "input_mapping": {"id": "{{transform_data.customer_id}}"},
                    "output_schema": {"name": "string"}
                },
                {
                    "id": "validate_order",
                    "tool": "validator",
                    "input_mapping": {"order": "{{transform_data}}"},
                    "output_schema": {"valid": "boolean"}
                },
                {
                    "id": "format_output",
                    "tool": "formatter",
                    "input_mapping": {"data": "{{validate_order}}"},
                    "output_schema": {"result": "object"}
                }
            ],
            "error_handling": {
                "on_step_failure": "retry",
                "retry_policy": {"max_retries": 3, "backoff": "exponential"},
                "fallback": {"action": "queue_for_review"}
            }
        }
    }"#;

    let result = grader.grade(&exercise, valid_output).unwrap();
    assert!(
        result.passed,
        "Expected valid pipeline to pass: {}",
        result.message
    );
}

#[test]
fn test_workflow_tool_wiring_01_missing_error_handling() {
    let exercise = load_exercise("workflows", "workflow_tool_wiring_01");
    let grader = Grader::new().unwrap();

    let invalid_output = r#"{
        "pipeline": {
            "name": "Test Pipeline",
            "steps": [
                {"id": "fetch_order", "tool": "http", "input_mapping": {}, "output_schema": {}},
                {"id": "transform_data", "tool": "transform", "input_mapping": {}, "output_schema": {}},
                {"id": "enrich_customer", "tool": "crm", "input_mapping": {}, "output_schema": {}},
                {"id": "validate_order", "tool": "validate", "input_mapping": {}, "output_schema": {}},
                {"id": "format_output", "tool": "format", "input_mapping": {}, "output_schema": {}}
            ]
        }
    }"#;

    let result = grader.grade(&exercise, invalid_output).unwrap();
    assert!(!result.passed, "Expected missing error_handling to fail");
}

// =============================================================================
// WORKFLOW_HUMAN_LOOP_01 Tests - Human-in-the-Loop Patterns
// =============================================================================

#[test]
fn test_workflow_human_loop_01_valid_workflow() {
    let exercise = load_exercise("workflows", "workflow_human_loop_01");
    let grader = Grader::new().unwrap();

    let valid_output = r#"{
        "workflow": {
            "name": "Content Moderation",
            "steps": [
                {"id": "receive_content", "action": "ingest"},
                {"id": "classify_content", "action": "ml_classify"},
                {"id": "route_decision", "action": "conditional_route"},
                {"id": "apply_action", "action": "execute"}
            ],
            "approval_gates": [
                {
                    "id": "human_review",
                    "trigger_condition": "confidence < 0.8",
                    "request_to": ["moderator_queue"],
                    "context_fields": ["content_id", "classification"],
                    "timeout_seconds": 3600,
                    "outcomes": {
                        "approved": {"next_step": "apply_action", "action": "publish"},
                        "rejected": {"next_step": "apply_action", "action": "remove"},
                        "timeout": {"next_step": "apply_action", "action": "hold"}
                    }
                }
            ],
            "timeout_handling": {
                "default_action": "hold_for_review",
                "escalation": {
                    "enabled": true,
                    "escalate_to": ["senior_moderator"],
                    "notify": ["team@example.com"]
                }
            }
        }
    }"#;

    let result = grader.grade(&exercise, valid_output).unwrap();
    assert!(
        result.passed,
        "Expected valid human-in-the-loop workflow to pass: {}",
        result.message
    );
}

#[test]
fn test_workflow_human_loop_01_missing_approval_gate() {
    let exercise = load_exercise("workflows", "workflow_human_loop_01");
    let grader = Grader::new().unwrap();

    let invalid_output = r#"{
        "workflow": {
            "name": "Content Moderation",
            "steps": [
                {"id": "receive_content", "action": "ingest"},
                {"id": "classify_content", "action": "ml_classify"},
                {"id": "route_decision", "action": "conditional_route"},
                {"id": "apply_action", "action": "execute"}
            ],
            "approval_gates": [],
            "timeout_handling": {
                "default_action": "hold",
                "escalation": {"enabled": false}
            }
        }
    }"#;

    let result = grader.grade(&exercise, invalid_output).unwrap();
    assert!(!result.passed, "Expected missing approval gate to fail");
}

#[test]
fn test_workflow_human_loop_01_timeout_too_short() {
    let exercise = load_exercise("workflows", "workflow_human_loop_01");
    let grader = Grader::new().unwrap();

    let invalid_output = r#"{
        "workflow": {
            "name": "Content Moderation",
            "steps": [
                {"id": "receive_content", "action": "ingest"},
                {"id": "classify_content", "action": "ml_classify"},
                {"id": "route_decision", "action": "conditional_route"},
                {"id": "apply_action", "action": "execute"}
            ],
            "approval_gates": [
                {
                    "id": "human_review",
                    "trigger_condition": "confidence < 0.8",
                    "request_to": ["queue"],
                    "context_fields": ["id"],
                    "timeout_seconds": 30,
                    "outcomes": {
                        "approved": {"next_step": "apply_action", "action": "publish"},
                        "rejected": {"next_step": "apply_action", "action": "remove"},
                        "timeout": {"next_step": "apply_action", "action": "hold"}
                    }
                }
            ],
            "timeout_handling": {
                "default_action": "hold",
                "escalation": {"enabled": false}
            }
        }
    }"#;

    let result = grader.grade(&exercise, invalid_output).unwrap();
    assert!(!result.passed, "Expected timeout < 60 seconds to fail");
}

// =============================================================================
// PRODUCTION_EVAL_01 Tests - Evaluation Harness Design
// =============================================================================

#[test]
fn test_production_eval_01_valid_harness() {
    let exercise = load_exercise("production", "production_eval_01");
    let grader = Grader::new().unwrap();

    let valid_output = r#"{
        "eval_harness": {
            "name": "Customer Support Eval",
            "agent_under_test": {
                "name": "support_agent",
                "capabilities": ["classification"],
                "model": "claude-sonnet-4-20250514"
            },
            "test_cases": [
                {"id": "tc1", "input": "billing issue", "expected": {"category": "billing"}, "type": "deterministic"},
                {"id": "tc2", "input": "system down", "expected": {"category": "technical"}, "type": "deterministic"},
                {"id": "tc3", "input": "frustrated", "expected": "empathy", "type": "semantic"}
            ],
            "metrics": [
                {"name": "accuracy", "type": "accuracy", "threshold": 0.95, "aggregation": "mean"},
                {"name": "latency", "type": "latency", "threshold": 2000, "aggregation": "p95"}
            ],
            "reliability": {
                "runs_per_case": 5,
                "pass_threshold": 0.8,
                "confidence_interval": 0.95
            },
            "regression_detection": {
                "baseline": "v1.0",
                "tolerance": 0.05,
                "alert_on_degradation": true
            }
        }
    }"#;

    let result = grader.grade(&exercise, valid_output).unwrap();
    assert!(
        result.passed,
        "Expected valid eval harness to pass: {}",
        result.message
    );
}

#[test]
fn test_production_eval_01_production_track() {
    let exercise = load_exercise("production", "production_eval_01");
    assert_eq!(
        exercise.manifest.exercise.track,
        Track::Production,
        "Exercise should be in production track"
    );
}

#[test]
fn test_production_eval_01_missing_test_cases() {
    let exercise = load_exercise("production", "production_eval_01");
    let grader = Grader::new().unwrap();

    let invalid_output = r#"{
        "eval_harness": {
            "name": "Test",
            "agent_under_test": {"name": "agent", "capabilities": ["a"], "model": "m"},
            "test_cases": [
                {"id": "tc1", "input": "x", "expected": "y", "type": "deterministic"}
            ],
            "metrics": [
                {"name": "m1", "type": "accuracy", "threshold": 0.9, "aggregation": "mean"},
                {"name": "m2", "type": "latency", "threshold": 1000, "aggregation": "p95"}
            ],
            "reliability": {"runs_per_case": 5, "pass_threshold": 0.8, "confidence_interval": 0.95},
            "regression_detection": {"baseline": "v1", "tolerance": 0.05, "alert_on_degradation": true}
        }
    }"#;

    let result = grader.grade(&exercise, invalid_output).unwrap();
    assert!(!result.passed, "Expected < 3 test cases to fail");
}

// =============================================================================
// PRODUCTION_SECURITY_01 Tests - Prompt Injection Defense
// =============================================================================

#[test]
fn test_production_security_01_valid_config() {
    let exercise = load_exercise("production", "production_security_01");
    let grader = Grader::new().unwrap();

    let valid_output = r#"{
        "security_config": {
            "name": "Agent Security v1",
            "input_validation": {
                "max_length": 10000,
                "blocked_patterns": ["ignore instructions"],
                "sanitization_rules": [{"type": "strip"}],
                "encoding_check": true
            },
            "output_filtering": {
                "action_allowlist": ["read", "respond"],
                "pii_detection": true
            },
            "privilege_separation": {
                "trust_levels": [
                    {"name": "user", "level": 1},
                    {"name": "admin", "level": 2}
                ],
                "tool_permissions": {"read": 1, "write": 2}
            },
            "attack_detection": {
                "patterns": [{"name": "override", "pattern": "ignore.*instruction", "severity": "high"}],
                "logging_level": "verbose"
            },
            "response_procedures": {
                "on_detection": "block_and_log",
                "quarantine": true
            }
        }
    }"#;

    let result = grader.grade(&exercise, valid_output).unwrap();
    assert!(
        result.passed,
        "Expected valid security config to pass: {}",
        result.message
    );
}

#[test]
fn test_production_security_01_missing_trust_levels() {
    let exercise = load_exercise("production", "production_security_01");
    let grader = Grader::new().unwrap();

    let invalid_output = r#"{
        "security_config": {
            "name": "Test",
            "input_validation": {"max_length": 1000, "blocked_patterns": ["x"], "sanitization_rules": [{}]},
            "output_filtering": {"action_allowlist": ["a"]},
            "privilege_separation": {
                "trust_levels": [{"name": "user", "level": 1}],
                "tool_permissions": {"a": 1}
            },
            "attack_detection": {"patterns": [{"name": "x", "pattern": "x"}], "logging_level": "minimal"},
            "response_procedures": {"on_detection": "log_only"}
        }
    }"#;

    let result = grader.grade(&exercise, invalid_output).unwrap();
    assert!(!result.passed, "Expected < 2 trust levels to fail");
}

// =============================================================================
// PRODUCTION_BUDGET_01 Tests - Cost and Latency Budgets
// =============================================================================

#[test]
fn test_production_budget_01_valid_config() {
    let exercise = load_exercise("production", "production_budget_01");
    let grader = Grader::new().unwrap();

    let valid_output = r#"{
        "budget_config": {
            "name": "Production Budget",
            "cost_limits": {
                "per_request_max_usd": 0.10,
                "daily_limit_usd": 100.0,
                "monthly_limit_usd": 2000.0
            },
            "latency_slos": {
                "p50_ms": 1000,
                "p95_ms": 3000,
                "p99_ms": 5000,
                "timeout_ms": 10000
            },
            "token_limits": {
                "max_input_tokens": 4096,
                "max_output_tokens": 2048,
                "max_total_tokens": 8192
            },
            "degradation_policy": {
                "on_cost_warning": ["use_smaller_model"],
                "on_latency_warning": ["skip_tools"],
                "on_limit_reached": "reject"
            },
            "monitoring": {
                "metrics_to_track": ["cost", "latency"],
                "alert_channels": ["slack"],
                "dashboard_enabled": true
            }
        }
    }"#;

    let result = grader.grade(&exercise, valid_output).unwrap();
    assert!(
        result.passed,
        "Expected valid budget config to pass: {}",
        result.message
    );
}

#[test]
fn test_production_budget_01_invalid_on_limit_reached() {
    let exercise = load_exercise("production", "production_budget_01");
    let grader = Grader::new().unwrap();

    let invalid_output = r#"{
        "budget_config": {
            "name": "Test",
            "cost_limits": {"per_request_max_usd": 0.1, "daily_limit_usd": 100, "monthly_limit_usd": 1000},
            "latency_slos": {"p50_ms": 100, "p95_ms": 200, "p99_ms": 300, "timeout_ms": 500},
            "token_limits": {"max_input_tokens": 100, "max_output_tokens": 100, "max_total_tokens": 200},
            "degradation_policy": {
                "on_cost_warning": ["a"],
                "on_latency_warning": ["b"],
                "on_limit_reached": "crash"
            },
            "monitoring": {"metrics_to_track": ["x"], "alert_channels": ["y"]}
        }
    }"#;

    let result = grader.grade(&exercise, invalid_output).unwrap();
    assert!(
        !result.passed,
        "Expected invalid on_limit_reached value to fail"
    );
}

// =============================================================================
// MCP_RESOURCE_01 Tests - MCP Resource Definition
// =============================================================================

#[test]
fn test_mcp_resource_01_valid_resource() {
    let exercise = load_exercise("mcp", "resource_01");
    let grader = Grader::new().unwrap();

    let valid_output = r#"{
        "uri": "user://profile/current",
        "name": "Current User Profile",
        "description": "The authenticated user's profile information including name, email, and preferences",
        "mimeType": "application/json"
    }"#;

    let result = grader.grade(&exercise, valid_output).unwrap();
    assert!(
        result.passed,
        "Expected valid MCP resource definition to pass: {}",
        result.message
    );
}

#[test]
fn test_mcp_resource_01_wrong_uri() {
    let exercise = load_exercise("mcp", "resource_01");
    let grader = Grader::new().unwrap();

    let invalid_output = r#"{
        "uri": "https://api.example.com/profile",
        "name": "User Profile",
        "description": "The user profile information from the API",
        "mimeType": "application/json"
    }"#;

    let result = grader.grade(&exercise, invalid_output).unwrap();
    assert!(!result.passed, "Expected wrong URI to fail");
}

#[test]
fn test_mcp_resource_01_wrong_mime_type() {
    let exercise = load_exercise("mcp", "resource_01");
    let grader = Grader::new().unwrap();

    let invalid_output = r#"{
        "uri": "user://profile/current",
        "name": "User Profile",
        "description": "The user profile information as plain text",
        "mimeType": "text/plain"
    }"#;

    let result = grader.grade(&exercise, invalid_output).unwrap();
    assert!(!result.passed, "Expected wrong MIME type to fail");
}

#[test]
fn test_mcp_resource_01_name_too_short() {
    let exercise = load_exercise("mcp", "resource_01");
    let grader = Grader::new().unwrap();

    let invalid_output = r#"{
        "uri": "user://profile/current",
        "name": "User",
        "description": "The user profile information with all details",
        "mimeType": "application/json"
    }"#;

    let result = grader.grade(&exercise, invalid_output).unwrap();
    assert!(!result.passed, "Expected name < 5 chars to fail");
}

#[test]
fn test_mcp_resource_01_description_too_short() {
    let exercise = load_exercise("mcp", "resource_01");
    let grader = Grader::new().unwrap();

    let invalid_output = r#"{
        "uri": "user://profile/current",
        "name": "User Profile",
        "description": "User profile data",
        "mimeType": "application/json"
    }"#;

    let result = grader.grade(&exercise, invalid_output).unwrap();
    assert!(!result.passed, "Expected description < 20 chars to fail");
}

#[test]
fn test_mcp_resource_01_missing_field() {
    let exercise = load_exercise("mcp", "resource_01");
    let grader = Grader::new().unwrap();

    let invalid_output = r#"{
        "uri": "user://profile/current",
        "name": "User Profile",
        "mimeType": "application/json"
    }"#;

    let result = grader.grade(&exercise, invalid_output).unwrap();
    assert!(!result.passed, "Expected missing description to fail");
}

#[test]
fn test_mcp_resource_01_extra_field() {
    let exercise = load_exercise("mcp", "resource_01");
    let grader = Grader::new().unwrap();

    let invalid_output = r#"{
        "uri": "user://profile/current",
        "name": "Current User Profile",
        "description": "The authenticated user's profile information",
        "mimeType": "application/json",
        "extra_field": "not allowed"
    }"#;

    let result = grader.grade(&exercise, invalid_output).unwrap();
    assert!(
        !result.passed,
        "Expected extra field to fail (additionalProperties: false)"
    );
}

#[test]
fn test_mcp_resource_01_mcp_track() {
    let exercise = load_exercise("mcp", "resource_01");
    assert_eq!(
        exercise.manifest.exercise.track,
        Track::Mcp,
        "Exercise should be in MCP track"
    );
}

// =============================================================================
// CONTEXT_01 Tests - System Prompt Structure
// =============================================================================

#[test]
fn test_context_01_valid_prompt_structure() {
    let exercise = load_exercise("context", "context_01");
    let grader = Grader::new().unwrap();

    let valid_output = r#"{
        "role": "You are a helpful customer support assistant for TechCorp, specializing in helping users troubleshoot technical issues.",
        "capabilities": [
            "Answer questions about product features",
            "Help troubleshoot common technical issues",
            "Guide users through account management"
        ],
        "constraints": [
            "Do not provide legal or financial advice",
            "Do not access or modify user accounts directly"
        ],
        "response_format": {
            "style": "conversational",
            "max_length": 500,
            "include_sources": false
        },
        "examples": [
            {
                "user": "How do I reset my password?",
                "assistant": "To reset your password, go to Settings > Account > Change Password. You'll receive a confirmation email."
            }
        ]
    }"#;

    let result = grader.grade(&exercise, valid_output).unwrap();
    assert!(
        result.passed,
        "Expected valid prompt structure to pass: {}",
        result.message
    );
}

#[test]
fn test_context_01_missing_capabilities() {
    let exercise = load_exercise("context", "context_01");
    let grader = Grader::new().unwrap();

    let invalid_output = r#"{
        "role": "You are a helpful customer support assistant for TechCorp.",
        "capabilities": ["Help users"],
        "constraints": [
            "Do not provide legal advice",
            "Do not access accounts"
        ],
        "response_format": {
            "style": "concise",
            "max_length": 300,
            "include_sources": false
        },
        "examples": [
            {"user": "Hello", "assistant": "Hi there! How can I help?"}
        ]
    }"#;

    let result = grader.grade(&exercise, invalid_output).unwrap();
    assert!(
        !result.passed,
        "Expected too few capabilities (< 3) to fail"
    );
}

#[test]
fn test_context_01_invalid_style_enum() {
    let exercise = load_exercise("context", "context_01");
    let grader = Grader::new().unwrap();

    let invalid_output = r#"{
        "role": "You are a helpful customer support assistant for TechCorp.",
        "capabilities": ["Help A", "Help B", "Help C"],
        "constraints": ["Constraint A", "Constraint B"],
        "response_format": {
            "style": "casual",
            "max_length": 300,
            "include_sources": false
        },
        "examples": [
            {"user": "Hello", "assistant": "Hi there! How can I help?"}
        ]
    }"#;

    let result = grader.grade(&exercise, invalid_output).unwrap();
    assert!(!result.passed, "Expected invalid style enum to fail");
}

#[test]
fn test_context_01_missing_examples() {
    let exercise = load_exercise("context", "context_01");
    let grader = Grader::new().unwrap();

    let invalid_output = r#"{
        "role": "You are a helpful customer support assistant for TechCorp.",
        "capabilities": ["Help A", "Help B", "Help C"],
        "constraints": ["Constraint A", "Constraint B"],
        "response_format": {
            "style": "concise",
            "max_length": 300,
            "include_sources": false
        },
        "examples": []
    }"#;

    let result = grader.grade(&exercise, invalid_output).unwrap();
    assert!(!result.passed, "Expected empty examples array to fail");
}

#[test]
fn test_context_01_context_track() {
    let exercise = load_exercise("context", "context_01");
    assert_eq!(
        exercise.manifest.exercise.track,
        Track::Context,
        "Exercise should be in context track"
    );
}

// =============================================================================
// CONTEXT_02 Tests - Context Budget Management
// =============================================================================

#[test]
fn test_context_02_valid_budget_allocation() {
    let exercise = load_exercise("context", "context_02");
    let grader = Grader::new().unwrap();

    let valid_output = r#"{
        "allocations": [
            {
                "context_type": "system_prompt",
                "tokens": 1500,
                "priority": 1,
                "compression_strategy": "keep_verbatim",
                "justification": "System prompt defines agent behavior and must be preserved completely."
            },
            {
                "context_type": "code_context",
                "tokens": 2500,
                "priority": 2,
                "compression_strategy": "sample_representative",
                "justification": "Code context is essential for understanding the task at hand."
            },
            {
                "context_type": "documentation",
                "tokens": 1500,
                "priority": 3,
                "compression_strategy": "summarize",
                "justification": "Documentation provides reference material that can be summarized."
            },
            {
                "context_type": "conversation_history",
                "tokens": 1500,
                "priority": 4,
                "compression_strategy": "truncate_oldest",
                "justification": "Recent conversation is more relevant than older turns."
            },
            {
                "context_type": "tool_definitions",
                "tokens": 1000,
                "priority": 5,
                "compression_strategy": "eliminate",
                "justification": "Tool definitions can be loaded on-demand when needed."
            }
        ],
        "total_tokens": 8000
    }"#;

    let result = grader.grade(&exercise, valid_output).unwrap();
    assert!(
        result.passed,
        "Expected valid budget allocation to pass: {}",
        result.message
    );
}

#[test]
fn test_context_02_wrong_total_tokens() {
    let exercise = load_exercise("context", "context_02");
    let grader = Grader::new().unwrap();

    let invalid_output = r#"{
        "allocations": [
            {"context_type": "system_prompt", "tokens": 1000, "priority": 1, "compression_strategy": "keep_verbatim", "justification": "Preserved for behavior definition."},
            {"context_type": "code_context", "tokens": 2000, "priority": 2, "compression_strategy": "summarize", "justification": "Code context for task understanding."},
            {"context_type": "documentation", "tokens": 1000, "priority": 3, "compression_strategy": "summarize", "justification": "Reference material."},
            {"context_type": "conversation_history", "tokens": 1000, "priority": 4, "compression_strategy": "truncate_oldest", "justification": "Recent history."},
            {"context_type": "tool_definitions", "tokens": 1000, "priority": 5, "compression_strategy": "eliminate", "justification": "On-demand loading."}
        ],
        "total_tokens": 6000
    }"#;

    let result = grader.grade(&exercise, invalid_output).unwrap();
    assert!(
        !result.passed,
        "Expected wrong total_tokens (not 8000) to fail"
    );
}

#[test]
fn test_context_02_too_few_allocations() {
    let exercise = load_exercise("context", "context_02");
    let grader = Grader::new().unwrap();

    let invalid_output = r#"{
        "allocations": [
            {"context_type": "system_prompt", "tokens": 4000, "priority": 1, "compression_strategy": "keep_verbatim", "justification": "Preserved for behavior definition."},
            {"context_type": "code_context", "tokens": 4000, "priority": 2, "compression_strategy": "summarize", "justification": "Code context for task understanding."}
        ],
        "total_tokens": 8000
    }"#;

    let result = grader.grade(&exercise, invalid_output).unwrap();
    assert!(!result.passed, "Expected too few allocations (< 5) to fail");
}

#[test]
fn test_context_02_invalid_context_type() {
    let exercise = load_exercise("context", "context_02");
    let grader = Grader::new().unwrap();

    let invalid_output = r#"{
        "allocations": [
            {"context_type": "system_prompt", "tokens": 1600, "priority": 1, "compression_strategy": "keep_verbatim", "justification": "Preserved for behavior definition."},
            {"context_type": "custom_data", "tokens": 1600, "priority": 2, "compression_strategy": "summarize", "justification": "Custom data source."},
            {"context_type": "documentation", "tokens": 1600, "priority": 3, "compression_strategy": "summarize", "justification": "Reference material."},
            {"context_type": "conversation_history", "tokens": 1600, "priority": 4, "compression_strategy": "truncate_oldest", "justification": "Recent history."},
            {"context_type": "tool_definitions", "tokens": 1600, "priority": 5, "compression_strategy": "eliminate", "justification": "On-demand loading."}
        ],
        "total_tokens": 8000
    }"#;

    let result = grader.grade(&exercise, invalid_output).unwrap();
    assert!(!result.passed, "Expected invalid context_type enum to fail");
}

// =============================================================================
// CONTEXT_03 Tests - Just-in-Time Context Retrieval
// =============================================================================

#[test]
fn test_context_03_valid_jit_strategy() {
    let exercise = load_exercise("context", "context_03");
    let grader = Grader::new().unwrap();

    let valid_output = r#"{
        "initial_context": [
            {"name": "system_prompt", "description": "Core instructions and behavioral guidelines", "token_estimate": 500},
            {"name": "user_profile", "description": "Current user preferences and history summary", "token_estimate": 200}
        ],
        "triggers": [
            {"name": "code_query", "pattern": "code|function|class|method", "sources": ["codebase"], "max_tokens": 2000, "cache": true},
            {"name": "doc_query", "pattern": "how to|documentation|guide", "sources": ["docs"], "max_tokens": 1500, "cache": true},
            {"name": "error_query", "pattern": "error|exception|failed|bug", "sources": ["logs", "codebase"], "max_tokens": 1000, "cache": false}
        ],
        "sources": [
            {"name": "codebase", "type": "knowledge_base", "description": "Project source code repository", "avg_tokens": 1500},
            {"name": "docs", "type": "documentation", "description": "Technical documentation and guides", "avg_tokens": 1000},
            {"name": "logs", "type": "database", "description": "Application logs and error reports", "avg_tokens": 500}
        ],
        "loading_order": ["codebase", "docs", "logs"]
    }"#;

    let result = grader.grade(&exercise, valid_output).unwrap();
    assert!(
        result.passed,
        "Expected valid JIT strategy to pass: {}",
        result.message
    );
}

#[test]
fn test_context_03_too_few_triggers() {
    let exercise = load_exercise("context", "context_03");
    let grader = Grader::new().unwrap();

    let invalid_output = r#"{
        "initial_context": [
            {"name": "system_prompt", "description": "Core instructions and behavioral guidelines", "token_estimate": 500},
            {"name": "user_profile", "description": "Current user preferences and history", "token_estimate": 200}
        ],
        "triggers": [
            {"name": "code_query", "pattern": "code", "sources": ["codebase"], "max_tokens": 2000, "cache": true}
        ],
        "sources": [
            {"name": "codebase", "type": "knowledge_base", "description": "Project source code repository", "avg_tokens": 1500},
            {"name": "docs", "type": "documentation", "description": "Technical documentation", "avg_tokens": 1000},
            {"name": "logs", "type": "database", "description": "Application logs", "avg_tokens": 500}
        ],
        "loading_order": ["codebase", "docs", "logs"]
    }"#;

    let result = grader.grade(&exercise, invalid_output).unwrap();
    assert!(!result.passed, "Expected too few triggers (< 3) to fail");
}

#[test]
fn test_context_03_too_few_sources() {
    let exercise = load_exercise("context", "context_03");
    let grader = Grader::new().unwrap();

    let invalid_output = r#"{
        "initial_context": [
            {"name": "system_prompt", "description": "Core instructions and guidelines", "token_estimate": 500},
            {"name": "user_profile", "description": "Current user preferences and history", "token_estimate": 200}
        ],
        "triggers": [
            {"name": "trigger1", "pattern": "code", "sources": ["codebase"], "max_tokens": 2000, "cache": true},
            {"name": "trigger2", "pattern": "docs", "sources": ["codebase"], "max_tokens": 1500, "cache": true},
            {"name": "trigger3", "pattern": "logs", "sources": ["codebase"], "max_tokens": 1000, "cache": false}
        ],
        "sources": [
            {"name": "codebase", "type": "knowledge_base", "description": "Project source code repository", "avg_tokens": 1500}
        ],
        "loading_order": ["codebase"]
    }"#;

    let result = grader.grade(&exercise, invalid_output).unwrap();
    assert!(!result.passed, "Expected too few sources (< 3) to fail");
}

#[test]
fn test_context_03_invalid_source_type() {
    let exercise = load_exercise("context", "context_03");
    let grader = Grader::new().unwrap();

    let invalid_output = r#"{
        "initial_context": [
            {"name": "system_prompt", "description": "Core instructions and guidelines", "token_estimate": 500},
            {"name": "user_profile", "description": "Current user preferences and history", "token_estimate": 200}
        ],
        "triggers": [
            {"name": "trigger1", "pattern": "code", "sources": ["src1"], "max_tokens": 2000, "cache": true},
            {"name": "trigger2", "pattern": "docs", "sources": ["src2"], "max_tokens": 1500, "cache": true},
            {"name": "trigger3", "pattern": "logs", "sources": ["src3"], "max_tokens": 1000, "cache": false}
        ],
        "sources": [
            {"name": "src1", "type": "file_system", "description": "Local file system", "avg_tokens": 1500},
            {"name": "src2", "type": "documentation", "description": "Technical docs", "avg_tokens": 1000},
            {"name": "src3", "type": "database", "description": "Application logs", "avg_tokens": 500}
        ],
        "loading_order": ["src1", "src2", "src3"]
    }"#;

    let result = grader.grade(&exercise, invalid_output).unwrap();
    assert!(!result.passed, "Expected invalid source type enum to fail");
}

// =============================================================================
// CONTEXT_04 Tests - Conversation Compaction
// =============================================================================

#[test]
fn test_context_04_valid_compacted_conversation() {
    let exercise = load_exercise("context", "context_04");
    let grader = Grader::new().unwrap();

    let valid_output = r#"{
        "decisions": [
            {"statement": "Decided to use PostgreSQL for the database", "importance": "Foundation for all data storage decisions", "turn": 3},
            {"statement": "Agreed to implement REST API first, then GraphQL", "importance": "Defines the API development roadmap", "turn": 7}
        ],
        "facts": [
            {"statement": "The project deadline is March 15th, 2025", "relevance": "Critical for planning and prioritization", "turn": 1},
            {"statement": "Budget is capped at $50,000 for infrastructure", "relevance": "Constrains technology choices", "turn": 2}
        ],
        "current_state": "We have completed the database schema design and are now beginning the API implementation phase. The next step is to set up the REST endpoints.",
        "open_items": [
            "Finalize authentication strategy",
            "Choose deployment platform (AWS vs GCP)",
            "Schedule security review"
        ],
        "metadata": {
            "original_tokens": 5000,
            "compacted_tokens": 800
        }
    }"#;

    let result = grader.grade(&exercise, valid_output).unwrap();
    assert!(
        result.passed,
        "Expected valid compacted conversation to pass: {}",
        result.message
    );
}

#[test]
fn test_context_04_too_few_decisions() {
    let exercise = load_exercise("context", "context_04");
    let grader = Grader::new().unwrap();

    let invalid_output = r#"{
        "decisions": [
            {"statement": "Decided to use PostgreSQL for the database", "importance": "Foundation for all data storage decisions", "turn": 3}
        ],
        "facts": [
            {"statement": "The project deadline is March 15th, 2025", "relevance": "Critical for planning", "turn": 1},
            {"statement": "Budget is capped at $50,000", "relevance": "Constrains choices", "turn": 2}
        ],
        "current_state": "We have completed the database schema design and are beginning implementation.",
        "open_items": ["Finalize auth strategy"],
        "metadata": {"original_tokens": 5000, "compacted_tokens": 600}
    }"#;

    let result = grader.grade(&exercise, invalid_output).unwrap();
    assert!(!result.passed, "Expected too few decisions (< 2) to fail");
}

#[test]
fn test_context_04_too_few_facts() {
    let exercise = load_exercise("context", "context_04");
    let grader = Grader::new().unwrap();

    let invalid_output = r#"{
        "decisions": [
            {"statement": "Decided to use PostgreSQL", "importance": "Foundation for storage", "turn": 3},
            {"statement": "Agreed to implement REST API first", "importance": "Defines roadmap", "turn": 7}
        ],
        "facts": [
            {"statement": "The project deadline is March 15th", "relevance": "Critical for planning", "turn": 1}
        ],
        "current_state": "We have completed the database schema design and are beginning implementation.",
        "open_items": ["Finalize auth strategy"],
        "metadata": {"original_tokens": 5000, "compacted_tokens": 600}
    }"#;

    let result = grader.grade(&exercise, invalid_output).unwrap();
    assert!(!result.passed, "Expected too few facts (< 2) to fail");
}

#[test]
fn test_context_04_empty_open_items() {
    let exercise = load_exercise("context", "context_04");
    let grader = Grader::new().unwrap();

    let invalid_output = r#"{
        "decisions": [
            {"statement": "Decided to use PostgreSQL", "importance": "Foundation for storage", "turn": 3},
            {"statement": "Agreed to implement REST API first", "importance": "Defines roadmap", "turn": 7}
        ],
        "facts": [
            {"statement": "The project deadline is March 15th", "relevance": "Critical for planning", "turn": 1},
            {"statement": "Budget is capped at $50,000", "relevance": "Constrains choices", "turn": 2}
        ],
        "current_state": "We have completed the database schema design and are beginning implementation.",
        "open_items": [],
        "metadata": {"original_tokens": 5000, "compacted_tokens": 600}
    }"#;

    let result = grader.grade(&exercise, invalid_output).unwrap();
    assert!(!result.passed, "Expected empty open_items to fail");
}

#[test]
fn test_context_04_current_state_too_short() {
    let exercise = load_exercise("context", "context_04");
    let grader = Grader::new().unwrap();

    let invalid_output = r#"{
        "decisions": [
            {"statement": "Decided to use PostgreSQL", "importance": "Foundation for storage", "turn": 3},
            {"statement": "Agreed to implement REST API", "importance": "Defines roadmap", "turn": 7}
        ],
        "facts": [
            {"statement": "Deadline is March 15th", "relevance": "Critical for planning", "turn": 1},
            {"statement": "Budget is $50,000", "relevance": "Constrains choices", "turn": 2}
        ],
        "current_state": "Working on API.",
        "open_items": ["Finalize auth"],
        "metadata": {"original_tokens": 5000, "compacted_tokens": 600}
    }"#;

    let result = grader.grade(&exercise, invalid_output).unwrap();
    assert!(
        !result.passed,
        "Expected current_state too short (< 20 chars) to fail"
    );
}

// =============================================================================
// CONTEXT_05 Tests - Token-Efficient Tool Design
// =============================================================================

#[test]
fn test_context_05_valid_tool_set() {
    let exercise = load_exercise("context", "context_05");
    let grader = Grader::new().unwrap();

    let valid_output = r#"{
        "tools": [
            {
                "name": "search_files",
                "description": "Search for files in the codebase matching a pattern or query",
                "parameters": [
                    {"name": "query", "type": "string", "required": true, "description": "The search query or pattern to match"}
                ],
                "output_schema": {"files": "array", "count": "number"},
                "token_estimate": 80
            },
            {
                "name": "read_file",
                "description": "Read the contents of a file at the specified path",
                "parameters": [
                    {"name": "path", "type": "string", "required": true, "description": "The path to the file to read"}
                ],
                "output_schema": {"content": "string", "lines": "number"},
                "token_estimate": 70
            },
            {
                "name": "write_file",
                "description": "Write content to a file, creating it if necessary",
                "parameters": [
                    {"name": "path", "type": "string", "required": true, "description": "The destination file path"},
                    {"name": "content", "type": "string", "required": true, "description": "The content to write"}
                ],
                "output_schema": {"success": "boolean"},
                "token_estimate": 90
            },
            {
                "name": "run_command",
                "description": "Execute a shell command and return the output",
                "parameters": [
                    {"name": "command", "type": "string", "required": true, "description": "The command to execute"}
                ],
                "output_schema": {"stdout": "string", "exit_code": "number"},
                "token_estimate": 85
            },
            {
                "name": "git_status",
                "description": "Get the current git status of the repository",
                "parameters": [
                    {"name": "detailed", "type": "boolean", "required": false, "description": "Include detailed file changes"}
                ],
                "output_schema": {"branch": "string", "changes": "array"},
                "token_estimate": 75
            }
        ],
        "overlap_analysis": [
            {
                "tools": ["search_files", "read_file"],
                "potential_overlap": "Both tools interact with files in the codebase",
                "mitigation": "search_files finds files, read_file retrieves content - clear separation of concerns"
            }
        ]
    }"#;

    let result = grader.grade(&exercise, valid_output).unwrap();
    assert!(
        result.passed,
        "Expected valid tool set to pass: {}",
        result.message
    );
}

#[test]
fn test_context_05_too_few_tools() {
    let exercise = load_exercise("context", "context_05");
    let grader = Grader::new().unwrap();

    let invalid_output = r#"{
        "tools": [
            {
                "name": "search_files",
                "description": "Search for files in the codebase matching a pattern",
                "parameters": [
                    {"name": "query", "type": "string", "required": true, "description": "Search query"}
                ],
                "output_schema": {"files": "array"},
                "token_estimate": 80
            },
            {
                "name": "read_file",
                "description": "Read the contents of a file at the path",
                "parameters": [
                    {"name": "path", "type": "string", "required": true, "description": "File path"}
                ],
                "output_schema": {"content": "string"},
                "token_estimate": 70
            }
        ],
        "overlap_analysis": [
            {"tools": ["search_files", "read_file"], "potential_overlap": "Both work with files", "mitigation": "Different purposes"}
        ]
    }"#;

    let result = grader.grade(&exercise, invalid_output).unwrap();
    assert!(!result.passed, "Expected too few tools (< 5) to fail");
}

#[test]
fn test_context_05_invalid_tool_name_format() {
    let exercise = load_exercise("context", "context_05");
    let grader = Grader::new().unwrap();

    let invalid_output = r#"{
        "tools": [
            {"name": "SearchFiles", "description": "Search for files matching a pattern", "parameters": [{"name": "q", "type": "string", "required": true, "description": "Query"}], "output_schema": {"f": "array"}, "token_estimate": 80},
            {"name": "read_file", "description": "Read the contents of a file at path", "parameters": [{"name": "p", "type": "string", "required": true, "description": "Path"}], "output_schema": {"c": "string"}, "token_estimate": 70},
            {"name": "write_file", "description": "Write content to a file creating if needed", "parameters": [{"name": "p", "type": "string", "required": true, "description": "Path"}], "output_schema": {"ok": "boolean"}, "token_estimate": 90},
            {"name": "run_cmd", "description": "Execute a shell command and return output", "parameters": [{"name": "c", "type": "string", "required": true, "description": "Command"}], "output_schema": {"out": "string"}, "token_estimate": 85},
            {"name": "git_stat", "description": "Get the current git status of the repo", "parameters": [{"name": "d", "type": "boolean", "required": false, "description": "Details"}], "output_schema": {"b": "string"}, "token_estimate": 75}
        ],
        "overlap_analysis": [
            {"tools": ["SearchFiles", "read_file"], "potential_overlap": "Both work with files", "mitigation": "Different purposes"}
        ]
    }"#;

    let result = grader.grade(&exercise, invalid_output).unwrap();
    assert!(
        !result.passed,
        "Expected invalid tool name format (not snake_case) to fail"
    );
}

#[test]
fn test_context_05_missing_overlap_analysis() {
    let exercise = load_exercise("context", "context_05");
    let grader = Grader::new().unwrap();

    let invalid_output = r#"{
        "tools": [
            {"name": "search_files", "description": "Search for files matching a pattern", "parameters": [{"name": "query", "type": "string", "required": true, "description": "Search query"}], "output_schema": {"files": "array"}, "token_estimate": 80},
            {"name": "read_file", "description": "Read the contents of a file at path", "parameters": [{"name": "path", "type": "string", "required": true, "description": "File path"}], "output_schema": {"content": "string"}, "token_estimate": 70},
            {"name": "write_file", "description": "Write content to a file creating if needed", "parameters": [{"name": "path", "type": "string", "required": true, "description": "File path"}], "output_schema": {"success": "boolean"}, "token_estimate": 90},
            {"name": "run_command", "description": "Execute a shell command and return output", "parameters": [{"name": "command", "type": "string", "required": true, "description": "Shell command"}], "output_schema": {"stdout": "string"}, "token_estimate": 85},
            {"name": "git_status", "description": "Get the current git status of the repo", "parameters": [{"name": "detailed", "type": "boolean", "required": false, "description": "Include details"}], "output_schema": {"branch": "string"}, "token_estimate": 75}
        ],
        "overlap_analysis": []
    }"#;

    let result = grader.grade(&exercise, invalid_output).unwrap();
    assert!(
        !result.passed,
        "Expected empty overlap_analysis to fail (minItems: 1)"
    );
}

#[test]
fn test_context_05_invalid_parameter_type() {
    let exercise = load_exercise("context", "context_05");
    let grader = Grader::new().unwrap();

    let invalid_output = r#"{
        "tools": [
            {"name": "search_files", "description": "Search for files matching a pattern", "parameters": [{"name": "query", "type": "text", "required": true, "description": "Search query"}], "output_schema": {"files": "array"}, "token_estimate": 80},
            {"name": "read_file", "description": "Read the contents of a file at path", "parameters": [{"name": "path", "type": "string", "required": true, "description": "File path"}], "output_schema": {"content": "string"}, "token_estimate": 70},
            {"name": "write_file", "description": "Write content to a file creating if needed", "parameters": [{"name": "path", "type": "string", "required": true, "description": "File path"}], "output_schema": {"success": "boolean"}, "token_estimate": 90},
            {"name": "run_command", "description": "Execute a shell command and return output", "parameters": [{"name": "command", "type": "string", "required": true, "description": "Shell command"}], "output_schema": {"stdout": "string"}, "token_estimate": 85},
            {"name": "git_status", "description": "Get the current git status of the repo", "parameters": [{"name": "detailed", "type": "boolean", "required": false, "description": "Include details"}], "output_schema": {"branch": "string"}, "token_estimate": 75}
        ],
        "overlap_analysis": [
            {"tools": ["search_files", "read_file"], "potential_overlap": "Both work with files", "mitigation": "Different purposes"}
        ]
    }"#;

    let result = grader.grade(&exercise, invalid_output).unwrap();
    assert!(
        !result.passed,
        "Expected invalid parameter type enum to fail"
    );
}

#[test]
fn test_context_05_context_track() {
    let exercise = load_exercise("context", "context_05");
    assert_eq!(
        exercise.manifest.exercise.track,
        Track::Context,
        "Exercise should be in context track"
    );
}

// =============================================================================
// TOOLS_02 Tests - Multi-Tool Orchestration
// =============================================================================

#[test]
fn test_tools_02_fundamentals_track() {
    let exercise = load_exercise("fundamentals", "tools_02");
    assert_eq!(
        exercise.manifest.exercise.track,
        Track::Fundamentals,
        "Exercise should be in fundamentals track"
    );
}

#[test]
fn test_tools_02_unknown_tool() {
    let exercise = load_exercise("fundamentals", "tools_02");
    let grader = Grader::new().unwrap();

    let invalid_output = r#"{
        "tool_calls": [
            {"name": "unknown_function", "arguments": {"x": 1}}
        ]
    }"#;

    let result = grader.grade(&exercise, invalid_output).unwrap();
    assert!(!result.passed, "Expected unknown tool to fail");
}

#[test]
fn test_tools_02_missing_required_path() {
    let exercise = load_exercise("fundamentals", "tools_02");
    let grader = Grader::new().unwrap();

    let invalid_output = r#"{
        "tool_calls": [
            {"name": "read_file", "arguments": {}}
        ]
    }"#;

    let result = grader.grade(&exercise, invalid_output).unwrap();
    assert!(!result.passed, "Expected missing required path to fail");
}

#[test]
fn test_tools_02_write_without_read() {
    let exercise = load_exercise("fundamentals", "tools_02");
    let grader = Grader::new().unwrap();

    // Invariant requires reading orders.py before writing
    let invalid_output = r#"{
        "tool_calls": [
            {"name": "write_file", "arguments": {"path": "/src/orders.py", "content": "content"}}
        ]
    }"#;

    let result = grader.grade(&exercise, invalid_output).unwrap();
    assert!(
        !result.passed,
        "Expected write without read to fail due to invariant"
    );
}

// =============================================================================
// ERROR_01 Tests - Handling Tool Failures
// =============================================================================

#[test]
fn test_error_01_fundamentals_track() {
    let exercise = load_exercise("fundamentals", "error_01");
    assert_eq!(
        exercise.manifest.exercise.track,
        Track::Fundamentals,
        "Exercise should be in fundamentals track"
    );
}

#[test]
fn test_error_01_unknown_tool() {
    let exercise = load_exercise("fundamentals", "error_01");
    let grader = Grader::new().unwrap();

    let invalid_output = r#"{
        "tool_calls": [
            {"name": "delete_user", "arguments": {"user_id": "123"}}
        ]
    }"#;

    let result = grader.grade(&exercise, invalid_output).unwrap();
    assert!(!result.passed, "Expected unknown tool to fail");
}

#[test]
fn test_error_01_missing_user_id() {
    let exercise = load_exercise("fundamentals", "error_01");
    let grader = Grader::new().unwrap();

    let invalid_output = r#"{
        "tool_calls": [
            {"name": "get_user_profile", "arguments": {}}
        ]
    }"#;

    let result = grader.grade(&exercise, invalid_output).unwrap();
    assert!(!result.passed, "Expected missing user_id to fail");
}

#[test]
fn test_error_01_missing_error_handling() {
    let exercise = load_exercise("fundamentals", "error_01");
    let grader = Grader::new().unwrap();

    // No on_error field - should fail invariant
    let invalid_output = r#"{
        "tool_calls": [
            {"name": "get_user_profile", "arguments": {"user_id": "user123"}}
        ]
    }"#;

    let result = grader.grade(&exercise, invalid_output).unwrap();
    assert!(
        !result.passed,
        "Expected missing error handling to fail invariant"
    );
}

// =============================================================================
// GUARDRAILS_01 Tests - Input/Output Validation
// =============================================================================

#[test]
fn test_guardrails_01_fundamentals_track() {
    let exercise = load_exercise("fundamentals", "guardrails_01");
    assert_eq!(
        exercise.manifest.exercise.track,
        Track::Fundamentals,
        "Exercise should be in fundamentals track"
    );
}

#[test]
fn test_guardrails_01_invalid_order_id_format() {
    let exercise = load_exercise("fundamentals", "guardrails_01");
    let grader = Grader::new().unwrap();

    let invalid_output = r#"{
        "tool_calls": [
            {"name": "lookup_order", "arguments": {"order_id": "ABC-123"}}
        ]
    }"#;

    let result = grader.grade(&exercise, invalid_output).unwrap();
    assert!(
        !result.passed,
        "Expected invalid order_id format (non-numeric) to fail"
    );
}

#[test]
fn test_guardrails_01_invalid_email_format() {
    let exercise = load_exercise("fundamentals", "guardrails_01");
    let grader = Grader::new().unwrap();

    let invalid_output = r#"{
        "tool_calls": [
            {"name": "send_email", "arguments": {
                "to": "not-an-email",
                "subject": "Test",
                "body": "Test body"
            }}
        ]
    }"#;

    let result = grader.grade(&exercise, invalid_output).unwrap();
    assert!(!result.passed, "Expected invalid email format to fail");
}

#[test]
fn test_guardrails_01_missing_input_validation() {
    let exercise = load_exercise("fundamentals", "guardrails_01");
    let grader = Grader::new().unwrap();

    // No input_validation field - should fail invariant
    let invalid_output = r#"{
        "tool_calls": [
            {"name": "lookup_order", "arguments": {"order_id": "123456"}}
        ]
    }"#;

    let result = grader.grade(&exercise, invalid_output).unwrap();
    assert!(
        !result.passed,
        "Expected missing input_validation to fail invariant"
    );
}

// =============================================================================
// OBSERVABILITY_01 Tests - Tracing and Cost Awareness
// =============================================================================

#[test]
fn test_observability_01_fundamentals_track() {
    let exercise = load_exercise("fundamentals", "observability_01");
    assert_eq!(
        exercise.manifest.exercise.track,
        Track::Fundamentals,
        "Exercise should be in fundamentals track"
    );
}

#[test]
fn test_observability_01_unknown_tool() {
    let exercise = load_exercise("fundamentals", "observability_01");
    let grader = Grader::new().unwrap();

    let invalid_output = r#"{
        "tool_calls": [
            {"name": "delete_document", "arguments": {"document_id": "doc-001"}}
        ]
    }"#;

    let result = grader.grade(&exercise, invalid_output).unwrap();
    assert!(!result.passed, "Expected unknown tool to fail");
}

#[test]
fn test_observability_01_invalid_format_enum() {
    let exercise = load_exercise("fundamentals", "observability_01");
    let grader = Grader::new().unwrap();

    let invalid_output = r#"{
        "tool_calls": [
            {"name": "read_document", "arguments": {"document_id": "doc-001", "format": "pdf"}}
        ]
    }"#;

    let result = grader.grade(&exercise, invalid_output).unwrap();
    assert!(!result.passed, "Expected invalid format enum to fail");
}

#[test]
fn test_observability_01_summary_length_out_of_range() {
    let exercise = load_exercise("fundamentals", "observability_01");
    let grader = Grader::new().unwrap();

    let invalid_output = r#"{
        "tool_calls": [
            {"name": "summarize_text", "arguments": {"text": "Some text", "max_length": 1500}}
        ]
    }"#;

    let result = grader.grade(&exercise, invalid_output).unwrap();
    assert!(!result.passed, "Expected max_length > 1000 to fail");
}

#[test]
fn test_observability_01_missing_message_body() {
    let exercise = load_exercise("fundamentals", "observability_01");
    let grader = Grader::new().unwrap();

    let invalid_output = r#"{
        "tool_calls": [
            {"name": "send_message", "arguments": {"to": "user@example.com", "subject": "Test"}}
        ]
    }"#;

    let result = grader.grade(&exercise, invalid_output).unwrap();
    assert!(!result.passed, "Expected missing body field to fail");
}
