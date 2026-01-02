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
