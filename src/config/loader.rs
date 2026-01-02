//! Configuration loading and saving.

use super::types::{UserConfig, UserProgress};
use crate::error::{ConfigError, Error, Result};
use directories::ProjectDirs;
use std::path::PathBuf;

/// Get the configuration directory path.
fn config_dir() -> Result<PathBuf> {
    ProjectDirs::from("", "", "vibelings")
        .map(|dirs| dirs.config_dir().to_path_buf())
        .ok_or_else(|| {
            Error::Config(ConfigError::Invalid(
                "Could not determine config directory".to_string(),
            ))
        })
}

/// Get the path to the config file.
pub fn config_path() -> Result<PathBuf> {
    Ok(config_dir()?.join("config.toml"))
}

/// Get the path to the progress file.
pub fn progress_path() -> Result<PathBuf> {
    Ok(config_dir()?.join("progress.toml"))
}

/// Load user configuration from the config file.
pub fn load_config() -> Result<UserConfig> {
    let path = config_path()?;
    if !path.exists() {
        return Err(Error::Config(ConfigError::NotFound(path)));
    }

    let content = std::fs::read_to_string(&path)?;
    let config: UserConfig = toml::from_str(&content)?;
    Ok(config)
}

/// Load user configuration or create default if not exists.
pub fn load_or_create_config() -> Result<UserConfig> {
    let path = config_path()?;

    if path.exists() {
        let content = std::fs::read_to_string(&path)?;
        let config: UserConfig = toml::from_str(&content)?;
        Ok(config)
    } else {
        let config = UserConfig::default();
        save_config(&config)?;
        Ok(config)
    }
}

/// Save user configuration to the config file.
pub fn save_config(config: &UserConfig) -> Result<()> {
    let path = config_path()?;

    // Ensure directory exists
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    let content = toml::to_string_pretty(config).map_err(|e| {
        Error::Config(ConfigError::Invalid(format!(
            "Failed to serialize config: {}",
            e
        )))
    })?;
    std::fs::write(&path, content)?;
    Ok(())
}

/// Load user progress from the progress file.
pub fn load_progress() -> Result<UserProgress> {
    let path = progress_path()?;

    if path.exists() {
        let content = std::fs::read_to_string(&path)?;
        let progress: UserProgress = toml::from_str(&content)?;
        Ok(progress)
    } else {
        Ok(UserProgress::default())
    }
}

/// Save user progress to the progress file.
pub fn save_progress(progress: &UserProgress) -> Result<()> {
    let path = progress_path()?;

    // Ensure directory exists
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    let content = toml::to_string_pretty(progress).map_err(|e| {
        Error::Config(ConfigError::Invalid(format!(
            "Failed to serialize progress: {}",
            e
        )))
    })?;
    std::fs::write(&path, content)?;
    Ok(())
}

/// Generate the default config file content as a string.
pub fn default_config_content() -> String {
    r#"# Vibelings Configuration
# See https://github.com/AbdelStark/vibelings for documentation

[model]
# Provider: "openrouter", "openai", "anthropic", or "local"
provider = "openrouter"
# Model identifier (provider-specific format)
model = "anthropic/claude-sonnet-4-20250514"
# Temperature (0.0 = deterministic, higher = more creative)
temperature = 0

[openrouter]
# Environment variable containing your OpenRouter API key
api_key_env = "OPENROUTER_API_KEY"
# Enable Zero Data Retention (privacy)
zdr = true
# Data collection policy: "allow" or "deny"
data_collection = "deny"
# Allow fallback to other providers if primary fails
allow_fallbacks = true
# Preferred provider order for model routing
provider_order = ["anthropic", "openai"]

[sandbox]
# Allow exercises to access the network (default: false for security)
network = false
# Timeout for tool execution in seconds
timeout_seconds = 30
# Allowed commands in the sandbox
allowed_commands = ["cat", "ls", "grep", "jq"]

[display]
# Show token costs after each run
show_cost = true
# Show trace information for debugging
show_trace = true
# Color mode: "auto", "always", or "never"
color = "auto"
"#
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config_content() {
        let content = default_config_content();
        assert!(content.contains("[model]"));
        assert!(content.contains("[sandbox]"));
    }
}
