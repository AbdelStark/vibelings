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
    /// Track 5: Context Engineering - managing finite context effectively
    Context,
}

impl Track {
    /// Returns the display name for the track.
    pub fn display_name(&self) -> &'static str {
        match self {
            Track::Fundamentals => "Agentic Fundamentals",
            Track::Mcp => "MCP in Practice",
            Track::Workflows => "Workflow Orchestration",
            Track::Production => "Production Engineering",
            Track::Context => "Context Engineering",
        }
    }

    /// Returns the directory name for the track.
    pub fn dir_name(&self) -> &'static str {
        match self {
            Track::Fundamentals => "fundamentals",
            Track::Mcp => "mcp",
            Track::Workflows => "workflows",
            Track::Production => "production",
            Track::Context => "context",
        }
    }

    /// Parse a track from a directory name.
    pub fn from_dir_name(name: &str) -> Option<Self> {
        match name {
            "fundamentals" => Some(Track::Fundamentals),
            "mcp" => Some(Track::Mcp),
            "workflows" => Some(Track::Workflows),
            "production" => Some(Track::Production),
            "context" => Some(Track::Context),
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

impl std::fmt::Display for GraderType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GraderType::Schema => write!(f, "schema"),
            GraderType::Invariants => write!(f, "invariants"),
            GraderType::Combined => write!(f, "combined"),
            GraderType::Sandbox => write!(f, "sandbox"),
            GraderType::Reliability => write!(f, "reliability"),
            GraderType::LlmJudge => write!(f, "llm-judge"),
        }
    }
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
    use std::collections::HashSet;

    #[test]
    fn test_track_display_name() {
        assert_eq!(Track::Fundamentals.display_name(), "Agentic Fundamentals");
        assert_eq!(Track::Mcp.display_name(), "MCP in Practice");
        assert_eq!(Track::Workflows.display_name(), "Workflow Orchestration");
        assert_eq!(Track::Production.display_name(), "Production Engineering");
        assert_eq!(Track::Context.display_name(), "Context Engineering");
    }

    #[test]
    fn test_track_dir_name() {
        assert_eq!(Track::Fundamentals.dir_name(), "fundamentals");
        assert_eq!(Track::Mcp.dir_name(), "mcp");
        assert_eq!(Track::Workflows.dir_name(), "workflows");
        assert_eq!(Track::Production.dir_name(), "production");
        assert_eq!(Track::Context.dir_name(), "context");
    }

    #[test]
    fn test_track_from_dir_name() {
        assert_eq!(
            Track::from_dir_name("fundamentals"),
            Some(Track::Fundamentals)
        );
        assert_eq!(Track::from_dir_name("mcp"), Some(Track::Mcp));
        assert_eq!(Track::from_dir_name("workflows"), Some(Track::Workflows));
        assert_eq!(Track::from_dir_name("production"), Some(Track::Production));
        assert_eq!(Track::from_dir_name("context"), Some(Track::Context));
        assert_eq!(Track::from_dir_name("unknown"), None);
        assert_eq!(Track::from_dir_name(""), None);
    }

    #[test]
    fn test_track_display_format() {
        let track = Track::Fundamentals;
        assert_eq!(format!("{}", track), "Agentic Fundamentals");
    }

    #[test]
    fn test_status_symbol() {
        assert_eq!(ExerciseStatus::Pending.symbol(), "⏳");
        assert_eq!(ExerciseStatus::InProgress.symbol(), "🔄");
        assert_eq!(ExerciseStatus::Completed.symbol(), "✅");
        assert_eq!(ExerciseStatus::Flaky.symbol(), "🟡");
        assert_eq!(ExerciseStatus::NeedsReruns.symbol(), "🔁");
        assert_eq!(ExerciseStatus::Experimental.symbol(), "🧪");
    }

    #[test]
    fn test_status_display_format() {
        assert_eq!(format!("{}", ExerciseStatus::Pending), "Pending");
        assert_eq!(format!("{}", ExerciseStatus::InProgress), "In Progress");
        assert_eq!(format!("{}", ExerciseStatus::Completed), "Completed");
        assert_eq!(format!("{}", ExerciseStatus::Flaky), "Flaky");
        assert_eq!(format!("{}", ExerciseStatus::NeedsReruns), "Needs Reruns");
        assert_eq!(format!("{}", ExerciseStatus::Experimental), "Experimental");
    }

    #[test]
    fn test_grader_type_display_format() {
        assert_eq!(format!("{}", GraderType::Schema), "schema");
        assert_eq!(format!("{}", GraderType::Invariants), "invariants");
        assert_eq!(format!("{}", GraderType::Combined), "combined");
        assert_eq!(format!("{}", GraderType::Sandbox), "sandbox");
        assert_eq!(format!("{}", GraderType::Reliability), "reliability");
        assert_eq!(format!("{}", GraderType::LlmJudge), "llm-judge");
    }

    #[test]
    fn test_exercise_requirements_default() {
        let requirements = ExerciseRequirements::default();
        assert!(!requirements.tool_calling);
        assert!(!requirements.json_mode);
        // Note: Default::default() gives 0 for u32, but serde default gives 4096
        assert_eq!(requirements.min_context_window, 0);
        assert!(!requirements.network);
    }

    #[test]
    fn test_exercise_requirements_serde_default() {
        // When deserializing with requirements section but missing fields,
        // min_context_window should use the serde default function (4096)
        let toml_str = r#"
[exercise]
id = "test"
title = "Test"
track = "fundamentals"

[requirements]
# min_context_window omitted, should use default

[grader]
type = "schema"
"#;
        let manifest: ExerciseManifest = toml::from_str(toml_str).unwrap();
        assert_eq!(manifest.requirements.min_context_window, 4096);
    }

    #[test]
    fn test_exercise_run_config_default() {
        let config = ExerciseRunConfig::default();
        assert_eq!(config.max_tool_calls, 0);
        assert_eq!(config.timeout_seconds, 30);
        assert_eq!(config.runs, 1);
        assert!(config.required_passes.is_none());
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

    #[test]
    fn test_manifest_with_prerequisites() {
        let toml_str = r#"
[exercise]
id = "json_02"
title = "Complex JSON"
track = "fundamentals"
prerequisites = ["json_01"]
description = "Learn complex schemas"
difficulty = 2

[grader]
type = "schema"
schema_path = "schema.json"
"#;
        let manifest: ExerciseManifest = toml::from_str(toml_str).unwrap();
        assert_eq!(manifest.exercise.prerequisites, vec!["json_01"]);
        assert_eq!(
            manifest.exercise.description,
            Some("Learn complex schemas".to_string())
        );
        assert_eq!(manifest.exercise.difficulty, 2);
    }

    #[test]
    fn test_manifest_with_multi_run() {
        let toml_str = r#"
[exercise]
id = "reliability_01"
title = "Reliability Test"
track = "production"

[run]
runs = 5
required_passes = 4

[grader]
type = "schema"
schema_path = "schema.json"
"#;
        let manifest: ExerciseManifest = toml::from_str(toml_str).unwrap();
        assert_eq!(manifest.run.runs, 5);
        assert_eq!(manifest.run.required_passes, Some(4));
    }

    #[test]
    fn test_manifest_with_invariants() {
        let toml_str = r#"
[exercise]
id = "invariant_test"
title = "Invariant Test"
track = "fundamentals"

[grader]
type = "combined"
schema_path = "schema.json"
invariants = ["check1.sh", "check2.sh"]
"#;
        let manifest: ExerciseManifest = toml::from_str(toml_str).unwrap();
        assert_eq!(manifest.grader.grader_type, GraderType::Combined);
        assert_eq!(manifest.grader.invariants.len(), 2);
    }

    #[test]
    fn test_exercise_full_id() {
        let manifest = ExerciseManifest {
            exercise: ExerciseMetadata {
                id: "json_01".to_string(),
                title: "Basic JSON".to_string(),
                track: Track::Fundamentals,
                prerequisites: vec![],
                description: None,
                difficulty: 1,
            },
            requirements: ExerciseRequirements::default(),
            run: ExerciseRunConfig::default(),
            grader: GraderConfig {
                grader_type: GraderType::Schema,
                schema_path: Some("schema.json".to_string()),
                invariants: vec![],
                rubric_path: None,
            },
        };

        let exercise = Exercise {
            manifest,
            path: PathBuf::from("exercises/fundamentals/json_01"),
            status: ExerciseStatus::Pending,
            readme_path: PathBuf::from("exercises/fundamentals/json_01/README.md"),
            starter_path: PathBuf::from("exercises/fundamentals/json_01/starter"),
            grader_path: PathBuf::from("exercises/fundamentals/json_01/grader"),
            fixtures_path: None,
        };

        assert_eq!(exercise.full_id(), "fundamentals/json_01");
    }

    #[test]
    fn test_exercise_display_title() {
        let manifest = ExerciseManifest {
            exercise: ExerciseMetadata {
                id: "server_01".to_string(),
                title: "MCP Server Basics".to_string(),
                track: Track::Mcp,
                prerequisites: vec![],
                description: None,
                difficulty: 1,
            },
            requirements: ExerciseRequirements::default(),
            run: ExerciseRunConfig::default(),
            grader: GraderConfig {
                grader_type: GraderType::Schema,
                schema_path: Some("schema.json".to_string()),
                invariants: vec![],
                rubric_path: None,
            },
        };

        let exercise = Exercise {
            manifest,
            path: PathBuf::from("exercises/mcp/server_01"),
            status: ExerciseStatus::Pending,
            readme_path: PathBuf::from("exercises/mcp/server_01/README.md"),
            starter_path: PathBuf::from("exercises/mcp/server_01/starter"),
            grader_path: PathBuf::from("exercises/mcp/server_01/grader"),
            fixtures_path: None,
        };

        assert_eq!(
            exercise.display_title(),
            "[MCP in Practice] MCP Server Basics"
        );
    }

    #[test]
    fn test_exercise_prerequisites_met_no_prereqs() {
        let manifest = ExerciseManifest {
            exercise: ExerciseMetadata {
                id: "json_01".to_string(),
                title: "Basic".to_string(),
                track: Track::Fundamentals,
                prerequisites: vec![],
                description: None,
                difficulty: 1,
            },
            requirements: ExerciseRequirements::default(),
            run: ExerciseRunConfig::default(),
            grader: GraderConfig {
                grader_type: GraderType::Schema,
                schema_path: None,
                invariants: vec![],
                rubric_path: None,
            },
        };

        let exercise = Exercise {
            manifest,
            path: PathBuf::from("exercises/fundamentals/json_01"),
            status: ExerciseStatus::Pending,
            readme_path: PathBuf::from("README.md"),
            starter_path: PathBuf::from("starter"),
            grader_path: PathBuf::from("grader"),
            fixtures_path: None,
        };

        let completed: HashSet<String> = HashSet::new();
        assert!(exercise.prerequisites_met(&completed));
    }

    #[test]
    fn test_exercise_prerequisites_met_with_prereqs() {
        let manifest = ExerciseManifest {
            exercise: ExerciseMetadata {
                id: "json_02".to_string(),
                title: "Complex".to_string(),
                track: Track::Fundamentals,
                prerequisites: vec!["fundamentals/json_01".to_string()],
                description: None,
                difficulty: 2,
            },
            requirements: ExerciseRequirements::default(),
            run: ExerciseRunConfig::default(),
            grader: GraderConfig {
                grader_type: GraderType::Schema,
                schema_path: None,
                invariants: vec![],
                rubric_path: None,
            },
        };

        let exercise = Exercise {
            manifest,
            path: PathBuf::from("exercises/fundamentals/json_02"),
            status: ExerciseStatus::Pending,
            readme_path: PathBuf::from("README.md"),
            starter_path: PathBuf::from("starter"),
            grader_path: PathBuf::from("grader"),
            fixtures_path: None,
        };

        // Prerequisites not met
        let empty: HashSet<String> = HashSet::new();
        assert!(!exercise.prerequisites_met(&empty));

        // Prerequisites met
        let mut completed: HashSet<String> = HashSet::new();
        completed.insert("fundamentals/json_01".to_string());
        assert!(exercise.prerequisites_met(&completed));
    }

    #[test]
    fn test_exercise_prerequisites_met_multiple_prereqs() {
        let manifest = ExerciseManifest {
            exercise: ExerciseMetadata {
                id: "advanced".to_string(),
                title: "Advanced".to_string(),
                track: Track::Production,
                prerequisites: vec![
                    "fundamentals/json_01".to_string(),
                    "fundamentals/tools_01".to_string(),
                ],
                description: None,
                difficulty: 4,
            },
            requirements: ExerciseRequirements::default(),
            run: ExerciseRunConfig::default(),
            grader: GraderConfig {
                grader_type: GraderType::Schema,
                schema_path: None,
                invariants: vec![],
                rubric_path: None,
            },
        };

        let exercise = Exercise {
            manifest,
            path: PathBuf::from("exercises/production/advanced"),
            status: ExerciseStatus::Pending,
            readme_path: PathBuf::from("README.md"),
            starter_path: PathBuf::from("starter"),
            grader_path: PathBuf::from("grader"),
            fixtures_path: None,
        };

        // Only one prereq met
        let mut partial: HashSet<String> = HashSet::new();
        partial.insert("fundamentals/json_01".to_string());
        assert!(!exercise.prerequisites_met(&partial));

        // Both prereqs met
        partial.insert("fundamentals/tools_01".to_string());
        assert!(exercise.prerequisites_met(&partial));
    }

    #[test]
    fn test_track_serialization() {
        let track = Track::Context;
        let json = serde_json::to_string(&track).unwrap();
        assert_eq!(json, "\"context\"");

        let deserialized: Track = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, Track::Context);
    }

    #[test]
    fn test_status_serialization() {
        let status = ExerciseStatus::Flaky;
        let json = serde_json::to_string(&status).unwrap();
        assert_eq!(json, "\"flaky\"");

        let deserialized: ExerciseStatus = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, ExerciseStatus::Flaky);
    }
}
