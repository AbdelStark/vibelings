//! Tool sandbox and security.
//!
//! The sandbox provides a secure environment for executing tools with:
//! - Command allowlisting
//! - Filesystem confinement
//! - Network isolation
//! - Timeout enforcement
//! - Fixture-based mock responses for deterministic grading

mod executor;
pub mod fixtures;

pub use executor::{SandboxExecutor, ToolExecution, ToolResult};
pub use fixtures::{FixtureStore, ToolFixture};

use crate::config::SandboxConfig;
use crate::error::SandboxError;
use crate::{Error, Result};
use std::collections::HashSet;
use std::path::PathBuf;

/// The sandbox environment for tool execution.
pub struct Sandbox {
    /// Allowed commands
    allowed_commands: HashSet<String>,

    /// Whether network is allowed
    allow_network: bool,

    /// Timeout in seconds
    timeout_seconds: u64,

    /// Working directory for tool execution
    work_dir: PathBuf,
}

impl Sandbox {
    /// Create a new sandbox with default configuration.
    pub fn new() -> Self {
        Self::with_config(&SandboxConfig::default())
    }

    /// Create a sandbox with custom configuration.
    pub fn with_config(config: &SandboxConfig) -> Self {
        let allowed_commands: HashSet<String> = config.allowed_commands.iter().cloned().collect();

        Self {
            allowed_commands,
            allow_network: config.network,
            timeout_seconds: config.timeout_seconds,
            work_dir: std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")),
        }
    }

    /// Set the working directory.
    pub fn with_work_dir(mut self, dir: PathBuf) -> Self {
        self.work_dir = dir;
        self
    }

    /// Check if a command is allowed.
    pub fn is_command_allowed(&self, command: &str) -> bool {
        // Extract the base command name
        let base_command = command.split_whitespace().next().unwrap_or("");
        self.allowed_commands.contains(base_command)
    }

    /// Execute a tool in the sandbox.
    pub fn execute(&self, tool_name: &str, args: &[String]) -> Result<ToolResult> {
        // Check if command is allowed
        if !self.allowed_commands.contains(tool_name) {
            return Err(Error::Sandbox(SandboxError::CommandNotAllowed(
                tool_name.to_string(),
            )));
        }

        let executor = SandboxExecutor::new(
            self.timeout_seconds,
            self.allow_network,
            self.work_dir.clone(),
        );

        executor.execute(tool_name, args)
    }

    /// Get the list of allowed commands.
    pub fn allowed_commands(&self) -> &HashSet<String> {
        &self.allowed_commands
    }
}

impl Default for Sandbox {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sandbox_creation() {
        let sandbox = Sandbox::new();
        assert!(sandbox.is_command_allowed("cat"));
        assert!(sandbox.is_command_allowed("ls"));
        assert!(!sandbox.is_command_allowed("rm"));
    }

    #[test]
    fn test_command_allowlist() {
        let config = SandboxConfig {
            allowed_commands: vec!["echo".to_string()],
            ..Default::default()
        };
        let sandbox = Sandbox::with_config(&config);

        assert!(sandbox.is_command_allowed("echo"));
        assert!(!sandbox.is_command_allowed("cat"));
    }
}
