//! Fixture system for deterministic tool responses.
//!
//! This module provides the ability to load and use fixture data
//! for reproducible exercise grading. Fixtures are JSON files in
//! the exercise's `fixtures/` directory.
//!
//! ## Fixture Format
//!
//! Fixtures are JSON files with the following structure:
//!
//! ```json
//! {
//!   "tool": "get_weather",
//!   "arguments": { "location": "San Francisco" },
//!   "response": { "temperature": 72, "condition": "sunny" }
//! }
//! ```
//!
//! Multiple fixtures can be in one file as an array, or split across files.

use crate::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

/// A single tool fixture defining a mock response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolFixture {
    /// The name of the tool this fixture applies to
    pub tool: String,

    /// Optional arguments to match (if None, matches any call to this tool)
    #[serde(default)]
    pub arguments: Option<serde_json::Value>,

    /// The response to return when this fixture matches
    pub response: serde_json::Value,

    /// Optional error to return instead of response
    #[serde(default)]
    pub error: Option<String>,
}

/// A collection of fixtures loaded from an exercise.
#[derive(Debug, Clone, Default)]
pub struct FixtureStore {
    /// Fixtures indexed by tool name
    fixtures: HashMap<String, Vec<ToolFixture>>,
}

impl FixtureStore {
    /// Create a new empty fixture store.
    pub fn new() -> Self {
        Self::default()
    }

    /// Load fixtures from a directory.
    ///
    /// Reads all JSON files in the directory and parses them as fixtures.
    /// Files can contain a single fixture object or an array of fixtures.
    pub fn load_from_dir(path: &Path) -> Result<Self> {
        let mut store = Self::new();

        if !path.exists() || !path.is_dir() {
            return Ok(store);
        }

        for entry in std::fs::read_dir(path)? {
            let entry = entry?;
            let file_path = entry.path();

            if file_path.extension().and_then(|s| s.to_str()) == Some("json") {
                let content = std::fs::read_to_string(&file_path)?;

                // Try parsing as array first, then as single object
                if let Ok(fixtures) = serde_json::from_str::<Vec<ToolFixture>>(&content) {
                    for fixture in fixtures {
                        store.add_fixture(fixture);
                    }
                } else if let Ok(fixture) = serde_json::from_str::<ToolFixture>(&content) {
                    store.add_fixture(fixture);
                } else {
                    tracing::warn!("Failed to parse fixture file: {}", file_path.display());
                }
            }
        }

        Ok(store)
    }

    /// Add a fixture to the store.
    pub fn add_fixture(&mut self, fixture: ToolFixture) {
        self.fixtures
            .entry(fixture.tool.clone())
            .or_default()
            .push(fixture);
    }

    /// Check if the store has any fixtures.
    pub fn is_empty(&self) -> bool {
        self.fixtures.is_empty()
    }

    /// Get the number of fixtures.
    pub fn len(&self) -> usize {
        self.fixtures.values().map(|v| v.len()).sum()
    }

    /// Find a matching fixture for a tool call.
    ///
    /// If multiple fixtures match, returns the first one.
    /// Matching is done by tool name and optionally by arguments.
    pub fn find_fixture(
        &self,
        tool: &str,
        arguments: Option<&serde_json::Value>,
    ) -> Option<&ToolFixture> {
        let tool_fixtures = self.fixtures.get(tool)?;

        for fixture in tool_fixtures {
            // If fixture has no argument constraints, it matches any call
            if fixture.arguments.is_none() {
                return Some(fixture);
            }

            // If we have arguments, try to match them
            if let (Some(fixture_args), Some(call_args)) = (&fixture.arguments, arguments) {
                if arguments_match(fixture_args, call_args) {
                    return Some(fixture);
                }
            }
        }

        // If no specific match, return first fixture with no arguments constraint
        tool_fixtures.iter().find(|f| f.arguments.is_none())
    }

    /// Get the list of tools that have fixtures.
    pub fn available_tools(&self) -> Vec<&str> {
        self.fixtures.keys().map(|s| s.as_str()).collect()
    }
}

/// Check if fixture arguments match call arguments.
///
/// This does a partial match - the fixture arguments only need to
/// be a subset of the call arguments.
fn arguments_match(fixture_args: &serde_json::Value, call_args: &serde_json::Value) -> bool {
    match (fixture_args, call_args) {
        (serde_json::Value::Object(fixture), serde_json::Value::Object(call)) => {
            // All keys in fixture must exist in call with same value
            fixture.iter().all(|(key, value)| {
                call.get(key)
                    .map(|call_value| values_match(value, call_value))
                    .unwrap_or(false)
            })
        }
        _ => fixture_args == call_args,
    }
}

/// Check if two values match for fixture purposes.
fn values_match(fixture_value: &serde_json::Value, call_value: &serde_json::Value) -> bool {
    match (fixture_value, call_value) {
        (serde_json::Value::Object(f), serde_json::Value::Object(c)) => {
            f.iter().all(|(key, value)| {
                c.get(key)
                    .map(|call_value| values_match(value, call_value))
                    .unwrap_or(false)
            })
        }
        (serde_json::Value::Array(f), serde_json::Value::Array(c)) => {
            f.len() == c.len() && f.iter().zip(c.iter()).all(|(fv, cv)| values_match(fv, cv))
        }
        _ => fixture_value == call_value,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_empty_store() {
        let store = FixtureStore::new();
        assert!(store.is_empty());
        assert_eq!(store.len(), 0);
    }

    #[test]
    fn test_add_fixture() {
        let mut store = FixtureStore::new();
        store.add_fixture(ToolFixture {
            tool: "get_weather".to_string(),
            arguments: None,
            response: json!({"temp": 72}),
            error: None,
        });

        assert!(!store.is_empty());
        assert_eq!(store.len(), 1);
    }

    #[test]
    fn test_find_fixture_no_args() {
        let mut store = FixtureStore::new();
        store.add_fixture(ToolFixture {
            tool: "get_weather".to_string(),
            arguments: None,
            response: json!({"temp": 72}),
            error: None,
        });

        let fixture = store.find_fixture("get_weather", None);
        assert!(fixture.is_some());
        assert_eq!(fixture.unwrap().response, json!({"temp": 72}));
    }

    #[test]
    fn test_find_fixture_with_args() {
        let mut store = FixtureStore::new();
        store.add_fixture(ToolFixture {
            tool: "get_weather".to_string(),
            arguments: Some(json!({"location": "SF"})),
            response: json!({"temp": 72}),
            error: None,
        });
        store.add_fixture(ToolFixture {
            tool: "get_weather".to_string(),
            arguments: Some(json!({"location": "NYC"})),
            response: json!({"temp": 45}),
            error: None,
        });

        let sf = store.find_fixture("get_weather", Some(&json!({"location": "SF"})));
        assert!(sf.is_some());
        assert_eq!(sf.unwrap().response, json!({"temp": 72}));

        let nyc = store.find_fixture("get_weather", Some(&json!({"location": "NYC"})));
        assert!(nyc.is_some());
        assert_eq!(nyc.unwrap().response, json!({"temp": 45}));
    }

    #[test]
    fn test_find_fixture_not_found() {
        let store = FixtureStore::new();
        let fixture = store.find_fixture("unknown_tool", None);
        assert!(fixture.is_none());
    }

    #[test]
    fn test_arguments_match_simple() {
        assert!(arguments_match(&json!({"a": 1}), &json!({"a": 1})));
        assert!(!arguments_match(&json!({"a": 1}), &json!({"a": 2})));
    }

    #[test]
    fn test_arguments_match_subset() {
        // Fixture args can be a subset of call args
        assert!(arguments_match(&json!({"a": 1}), &json!({"a": 1, "b": 2})));
    }

    #[test]
    fn test_arguments_match_nested() {
        assert!(arguments_match(
            &json!({"a": {"b": 1}}),
            &json!({"a": {"b": 1, "c": 2}})
        ));
    }

    #[test]
    fn test_available_tools() {
        let mut store = FixtureStore::new();
        store.add_fixture(ToolFixture {
            tool: "tool_a".to_string(),
            arguments: None,
            response: json!({}),
            error: None,
        });
        store.add_fixture(ToolFixture {
            tool: "tool_b".to_string(),
            arguments: None,
            response: json!({}),
            error: None,
        });

        let tools = store.available_tools();
        assert!(tools.contains(&"tool_a"));
        assert!(tools.contains(&"tool_b"));
    }
}
