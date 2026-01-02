//! Exercise types and manifest parsing.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// An exercise track (collection of related exercises).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum Track {
    /// Track 1: Core agentic primitives without frameworks
    Fundamentals,
    /// Track 2: Model Context Protocol implementation
    Mcp,
    /// Track 3: Integration with workflow tools (n8n)
    Workflows,
    /// Track 4: Reliability at scale
    Production,
}

impl Track {
    /// Returns the display name for the track.
    pub fn display_name(&self) -> &'static str {
        match self {
            Track::Fundamentals => "Agentic Fundamentals",
            Track::Mcp => "MCP in Practice",
            Track::Workflows => "Workflow Orchestration",
            Track::Production => "Production Engineering",
        }
    }

    /// Returns the directory name for the track.
    pub fn dir_name(&self) -> &'static str {
        match self {
            Track::Fundamentals => "fundamentals",
            Track::Mcp => "mcp",
            Track::Workflows => "workflows",
            Track::Production => "production",
        }
    }

    /// Parse a track from a directory name.
    pub fn from_dir_name(name: &str) -> Option<Self> {
        match name {
            "fundamentals" => Some(Track::Fundamentals),
            "mcp" => Some(Track::Mcp),
            "workflows" => Some(Track::Workflows),
            "production" => Some(Track::Production),
            _ => None,
        }
    }
}

impl std::fmt::Display for Track {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display_name())
    }
}

/// Status of an exercise.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ExerciseStatus {
    /// Not yet attempted
    Pending,
    /// Currently in progress
    InProgress,
    /// Passed deterministic checks
    Completed,
    /// Passed but under reliability threshold
    Flaky,
    /// Multi-run exercise, insufficient data
    NeedsReruns,
    /// Exercise depends on rapidly changing ecosystem
    Experimental,
}

impl ExerciseStatus {
    /// Returns the display symbol for the status.
    pub fn symbol(&self) -> &'static str {
        match self {
            ExerciseStatus::Pending => "⏳",
            ExerciseStatus::InProgress => "🔄",
            ExerciseStatus::Completed => "✅",
            ExerciseStatus::Flaky => "🟡",
            ExerciseStatus::NeedsReruns => "🔁",
            ExerciseStatus::Experimental => "🧪",
        }
    }
}

impl std::fmt::Display for ExerciseStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExerciseStatus::Pending => write!(f, "Pending"),
            ExerciseStatus::InProgress => write!(f, "In Progress"),
            ExerciseStatus::Completed => write!(f, "Completed"),
            ExerciseStatus::Flaky => write!(f, "Flaky"),
            ExerciseStatus::NeedsReruns => write!(f, "Needs Reruns"),
            ExerciseStatus::Experimental => write!(f, "Experimental"),
        }
    }
}

/// Type of grader used for an exercise.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum GraderType {
    /// JSON Schema validation
    Schema,
    /// Shell script-based invariant checking
    Invariants,
    /// Combined schema + invariants
    Combined,
    /// Tool sandbox state checking
    Sandbox,
    /// Multi-run reliability checking
    Reliability,
    /// LLM-as-judge (last resort)
    LlmJudge,
}

/// Requirements for running an exercise.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ExerciseRequirements {
    /// Whether the exercise requires tool calling capability
    #[serde(default)]
    pub tool_calling: bool,

    /// Whether the exercise requires JSON mode
    #[serde(default)]
    pub json_mode: bool,

    /// Minimum context window size required
    #[serde(default = "default_context_window")]
    pub min_context_window: u32,

    /// Whether network access is required
    #[serde(default)]
    pub network: bool,
}

fn default_context_window() -> u32 {
    4096
}

/// Runtime configuration for an exercise.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExerciseRunConfig {
    /// Maximum number of tool calls allowed
    #[serde(default)]
    pub max_tool_calls: u32,

    /// Timeout in seconds
    #[serde(default = "default_timeout")]
    pub timeout_seconds: u64,

    /// Number of runs for reliability exercises
    #[serde(default = "default_runs")]
    pub runs: u32,

    /// Required passes for reliability exercises
    #[serde(default)]
    pub required_passes: Option<u32>,
}

fn default_timeout() -> u64 {
    30
}

fn default_runs() -> u32 {
    1
}

impl Default for ExerciseRunConfig {
    fn default() -> Self {
        Self {
            max_tool_calls: 0,
            timeout_seconds: default_timeout(),
            runs: default_runs(),
            required_passes: None,
        }
    }
}

/// Grader configuration for an exercise.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraderConfig {
    /// Type of grader to use
    #[serde(rename = "type")]
    pub grader_type: GraderType,

    /// Path to schema file (relative to exercise directory)
    #[serde(default)]
    pub schema_path: Option<String>,

    /// Paths to invariant scripts (relative to exercise directory)
    #[serde(default)]
    pub invariants: Vec<String>,

    /// For LLM-judge: path to rubric file
    #[serde(default)]
    pub rubric_path: Option<String>,
}

/// The manifest for an exercise (parsed from manifest.toml).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExerciseManifest {
    /// Exercise metadata
    pub exercise: ExerciseMetadata,

    /// Requirements for running
    #[serde(default)]
    pub requirements: ExerciseRequirements,

    /// Runtime configuration
    #[serde(default)]
    pub run: ExerciseRunConfig,

    /// Grader configuration
    pub grader: GraderConfig,
}

/// Exercise metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExerciseMetadata {
    /// Unique exercise identifier (e.g., "contracts_json_01")
    pub id: String,

    /// Human-readable title
    pub title: String,

    /// Track this exercise belongs to
    pub track: Track,

    /// List of exercise IDs that must be completed first
    #[serde(default)]
    pub prerequisites: Vec<String>,

    /// Short description of the exercise
    #[serde(default)]
    pub description: Option<String>,

    /// Difficulty level (1-5)
    #[serde(default = "default_difficulty")]
    pub difficulty: u8,
}

fn default_difficulty() -> u8 {
    1
}

/// A loaded exercise with all its metadata and paths.
#[derive(Debug, Clone)]
pub struct Exercise {
    /// The parsed manifest
    pub manifest: ExerciseManifest,

    /// Path to the exercise directory
    pub path: PathBuf,

    /// Current status of the exercise
    pub status: ExerciseStatus,

    /// Path to README.md
    pub readme_path: PathBuf,

    /// Path to starter directory
    pub starter_path: PathBuf,

    /// Path to grader directory
    pub grader_path: PathBuf,

    /// Path to fixtures directory (optional)
    pub fixtures_path: Option<PathBuf>,
}

impl Exercise {
    /// Returns the full exercise identifier (track/id).
    pub fn full_id(&self) -> String {
        format!(
            "{}/{}",
            self.manifest.exercise.track.dir_name(),
            self.manifest.exercise.id
        )
    }

    /// Returns the display title with track.
    pub fn display_title(&self) -> String {
        format!(
            "[{}] {}",
            self.manifest.exercise.track.display_name(),
            self.manifest.exercise.title
        )
    }

    /// Check if prerequisites are met given a set of completed exercise IDs.
    pub fn prerequisites_met(&self, completed: &std::collections::HashSet<String>) -> bool {
        self.manifest
            .exercise
            .prerequisites
            .iter()
            .all(|prereq| completed.contains(prereq))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_track_display_name() {
        assert_eq!(Track::Fundamentals.display_name(), "Agentic Fundamentals");
        assert_eq!(Track::Mcp.display_name(), "MCP in Practice");
    }

    #[test]
    fn test_track_dir_name() {
        assert_eq!(Track::Fundamentals.dir_name(), "fundamentals");
        assert_eq!(Track::Production.dir_name(), "production");
    }

    #[test]
    fn test_track_from_dir_name() {
        assert_eq!(
            Track::from_dir_name("fundamentals"),
            Some(Track::Fundamentals)
        );
        assert_eq!(Track::from_dir_name("unknown"), None);
    }

    #[test]
    fn test_status_symbol() {
        assert_eq!(ExerciseStatus::Completed.symbol(), "✅");
        assert_eq!(ExerciseStatus::Pending.symbol(), "⏳");
    }

    #[test]
    fn test_manifest_deserialization() {
        let toml_str = r#"
[exercise]
id = "json_01"
title = "Basic JSON Output"
track = "fundamentals"

[requirements]
json_mode = true

[run]
timeout_seconds = 30

[grader]
type = "schema"
schema_path = "grader/schema.json"
"#;
        let manifest: ExerciseManifest = toml::from_str(toml_str).unwrap();
        assert_eq!(manifest.exercise.id, "json_01");
        assert_eq!(manifest.exercise.track, Track::Fundamentals);
        assert!(manifest.requirements.json_mode);
    }
}
