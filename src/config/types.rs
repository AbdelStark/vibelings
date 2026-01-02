//! Configuration types.

use crate::ExerciseStatus;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// The main user configuration.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UserConfig {
    /// Model configuration
    #[serde(default)]
    pub model: ModelConfig,

    /// OpenRouter-specific configuration
    #[serde(default)]
    pub openrouter: OpenRouterConfig,

    /// OpenAI-specific configuration
    #[serde(default)]
    pub openai: OpenAIConfig,

    /// Anthropic-specific configuration
    #[serde(default)]
    pub anthropic: AnthropicConfig,

    /// Local provider configuration
    #[serde(default)]
    pub local: LocalConfig,

    /// Sandbox configuration
    #[serde(default)]
    pub sandbox: SandboxConfig,

    /// Display configuration
    #[serde(default)]
    pub display: DisplayConfig,
}

/// Provider type enum.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "lowercase")]
pub enum ProviderType {
    /// OpenRouter (unified API)
    #[default]
    OpenRouter,
    /// Direct OpenAI API
    OpenAI,
    /// Direct Anthropic API
    Anthropic,
    /// Local endpoint (Ollama, vLLM, etc.)
    Local,
}

impl std::fmt::Display for ProviderType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProviderType::OpenRouter => write!(f, "openrouter"),
            ProviderType::OpenAI => write!(f, "openai"),
            ProviderType::Anthropic => write!(f, "anthropic"),
            ProviderType::Local => write!(f, "local"),
        }
    }
}

/// Model configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelConfig {
    /// Provider to use
    #[serde(default)]
    pub provider: ProviderType,

    /// Model identifier
    #[serde(default = "default_model")]
    pub model: String,

    /// Temperature (0.0 - 2.0)
    #[serde(default)]
    pub temperature: f32,

    /// Maximum tokens to generate
    #[serde(default)]
    pub max_tokens: Option<u32>,
}

fn default_model() -> String {
    "anthropic/claude-sonnet-4-20250514".to_string()
}

impl Default for ModelConfig {
    fn default() -> Self {
        Self {
            provider: ProviderType::default(),
            model: default_model(),
            temperature: 0.0,
            max_tokens: None,
        }
    }
}

/// OpenRouter-specific configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenRouterConfig {
    /// Environment variable containing the API key
    #[serde(default = "default_api_key_env")]
    pub api_key_env: String,

    /// Enable Zero Data Retention
    #[serde(default = "default_true")]
    pub zdr: bool,

    /// Data collection policy
    #[serde(default = "default_data_collection")]
    pub data_collection: String,

    /// Allow provider fallbacks
    #[serde(default = "default_true")]
    pub allow_fallbacks: bool,

    /// Preferred provider order
    #[serde(default)]
    pub provider_order: Vec<String>,
}

fn default_api_key_env() -> String {
    "OPENROUTER_API_KEY".to_string()
}

fn default_true() -> bool {
    true
}

fn default_data_collection() -> String {
    "deny".to_string()
}

impl Default for OpenRouterConfig {
    fn default() -> Self {
        Self {
            api_key_env: default_api_key_env(),
            zdr: true,
            data_collection: default_data_collection(),
            allow_fallbacks: true,
            provider_order: vec!["anthropic".to_string(), "openai".to_string()],
        }
    }
}

/// OpenAI-specific configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenAIConfig {
    /// Environment variable containing the API key
    #[serde(default = "default_openai_api_key_env")]
    pub api_key_env: String,

    /// Optional organization ID environment variable
    #[serde(default)]
    pub org_id_env: Option<String>,
}

fn default_openai_api_key_env() -> String {
    "OPENAI_API_KEY".to_string()
}

impl Default for OpenAIConfig {
    fn default() -> Self {
        Self {
            api_key_env: default_openai_api_key_env(),
            org_id_env: None,
        }
    }
}

/// Anthropic-specific configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnthropicConfig {
    /// Environment variable containing the API key
    #[serde(default = "default_anthropic_api_key_env")]
    pub api_key_env: String,
}

fn default_anthropic_api_key_env() -> String {
    "ANTHROPIC_API_KEY".to_string()
}

impl Default for AnthropicConfig {
    fn default() -> Self {
        Self {
            api_key_env: default_anthropic_api_key_env(),
        }
    }
}

/// Local provider configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalConfig {
    /// Base URL for local OpenAI-compatible API
    #[serde(default = "default_local_base_url")]
    pub base_url: String,

    /// Optional API key for authenticated local endpoints
    #[serde(default)]
    pub api_key: Option<String>,
}

fn default_local_base_url() -> String {
    "http://localhost:11434/v1".to_string()
}

impl Default for LocalConfig {
    fn default() -> Self {
        Self {
            base_url: default_local_base_url(),
            api_key: None,
        }
    }
}

/// Sandbox configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SandboxConfig {
    /// Allow network access
    #[serde(default)]
    pub network: bool,

    /// Timeout in seconds
    #[serde(default = "default_timeout")]
    pub timeout_seconds: u64,

    /// Allowed commands
    #[serde(default = "default_allowed_commands")]
    pub allowed_commands: Vec<String>,
}

fn default_timeout() -> u64 {
    30
}

fn default_allowed_commands() -> Vec<String> {
    vec![
        "cat".to_string(),
        "ls".to_string(),
        "grep".to_string(),
        "jq".to_string(),
    ]
}

impl Default for SandboxConfig {
    fn default() -> Self {
        Self {
            network: false,
            timeout_seconds: default_timeout(),
            allowed_commands: default_allowed_commands(),
        }
    }
}

/// Display configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisplayConfig {
    /// Show cost information
    #[serde(default = "default_true")]
    pub show_cost: bool,

    /// Show trace information
    #[serde(default = "default_true")]
    pub show_trace: bool,

    /// Color mode
    #[serde(default = "default_color")]
    pub color: String,
}

fn default_color() -> String {
    "auto".to_string()
}

impl Default for DisplayConfig {
    fn default() -> Self {
        Self {
            show_cost: true,
            show_trace: true,
            color: default_color(),
        }
    }
}

/// Exercise progress entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExerciseProgress {
    /// Current status
    pub status: ExerciseStatus,

    /// Number of attempts
    #[serde(default)]
    pub attempts: u32,

    /// Number of successful runs (for reliability tracking)
    #[serde(default)]
    pub successful_runs: u32,

    /// Total runs (for reliability tracking)
    #[serde(default)]
    pub total_runs: u32,

    /// Last attempt timestamp
    #[serde(default)]
    pub last_attempt: Option<String>,

    /// Total tokens used
    #[serde(default)]
    pub total_tokens: u64,

    /// Total cost in USD
    #[serde(default)]
    pub total_cost: f64,
}

/// User progress tracking.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UserProgress {
    /// Progress by exercise ID
    #[serde(default)]
    pub exercises: HashMap<String, ExerciseProgress>,

    /// Currently active exercise
    #[serde(default)]
    pub current_exercise: Option<String>,
}

impl UserProgress {
    /// Get the set of completed exercise IDs.
    pub fn completed_exercises(&self) -> std::collections::HashSet<String> {
        self.exercises
            .iter()
            .filter(|(_, p)| p.status == ExerciseStatus::Completed)
            .map(|(id, _)| id.clone())
            .collect()
    }

    /// Check if an exercise is completed.
    pub fn is_completed(&self, exercise_id: &str) -> bool {
        self.exercises
            .get(exercise_id)
            .is_some_and(|p| p.status == ExerciseStatus::Completed)
    }

    /// Get the status of an exercise.
    pub fn get_status(&self, exercise_id: &str) -> ExerciseStatus {
        self.exercises
            .get(exercise_id)
            .map(|p| p.status)
            .unwrap_or(ExerciseStatus::Pending)
    }

    /// Mark an exercise as completed.
    pub fn mark_completed(&mut self, exercise_id: &str) {
        let progress = self
            .exercises
            .entry(exercise_id.to_string())
            .or_insert_with(|| ExerciseProgress {
                status: ExerciseStatus::Pending,
                attempts: 0,
                successful_runs: 0,
                total_runs: 0,
                last_attempt: None,
                total_tokens: 0,
                total_cost: 0.0,
            });
        progress.status = ExerciseStatus::Completed;
        progress.successful_runs += 1;
        progress.total_runs += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = UserConfig::default();
        assert_eq!(config.model.provider, ProviderType::OpenRouter);
        assert!(!config.sandbox.network);
    }

    #[test]
    fn test_config_deserialization() {
        let toml_str = r#"
[model]
provider = "openai"
model = "gpt-4"
temperature = 0.5

[sandbox]
network = true
timeout_seconds = 60
"#;
        let config: UserConfig = toml::from_str(toml_str).unwrap();
        assert_eq!(config.model.provider, ProviderType::OpenAI);
        assert_eq!(config.model.model, "gpt-4");
        assert!(config.sandbox.network);
    }

    #[test]
    fn test_progress_tracking() {
        let mut progress = UserProgress::default();
        progress.exercises.insert(
            "json_01".to_string(),
            ExerciseProgress {
                status: ExerciseStatus::Completed,
                attempts: 1,
                successful_runs: 1,
                total_runs: 1,
                last_attempt: None,
                total_tokens: 100,
                total_cost: 0.001,
            },
        );

        assert!(progress.is_completed("json_01"));
        assert!(!progress.is_completed("json_02"));
        assert!(progress.completed_exercises().contains("json_01"));
    }

    #[test]
    fn test_mark_completed() {
        let mut progress = UserProgress::default();

        // Initially, exercise should not be completed
        assert!(!progress.is_completed("json_01"));
        assert_eq!(progress.get_status("json_01"), ExerciseStatus::Pending);

        // Mark as completed
        progress.mark_completed("json_01");

        // Should now be completed
        assert!(progress.is_completed("json_01"));
        assert_eq!(progress.get_status("json_01"), ExerciseStatus::Completed);
        assert!(progress.completed_exercises().contains("json_01"));

        // Check the exercise progress details
        let ex_progress = progress.exercises.get("json_01").unwrap();
        assert_eq!(ex_progress.successful_runs, 1);
        assert_eq!(ex_progress.total_runs, 1);

        // Mark again should increment runs
        progress.mark_completed("json_01");
        let ex_progress = progress.exercises.get("json_01").unwrap();
        assert_eq!(ex_progress.successful_runs, 2);
        assert_eq!(ex_progress.total_runs, 2);
    }
}
