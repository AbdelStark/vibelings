//! Configuration loading and management.
//!
//! Handles loading user configuration from `~/.config/vibelings/config.toml`
//! and progress tracking from `~/.config/vibelings/progress.toml`.

mod loader;
mod types;

pub use loader::{
    default_config_content, load_config, load_or_create_config, load_progress, save_progress,
};
pub use types::{
    DisplayConfig, ExerciseProgress, ModelConfig, OpenRouterConfig, ProviderType, SandboxConfig,
    UserConfig, UserProgress,
};
