//! Sandbox tool execution.

use crate::error::SandboxError;
use crate::{Error, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

/// A tool execution request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolExecution {
    /// Tool/command name
    pub name: String,

    /// Arguments
    pub args: Vec<String>,

    /// Input to pass via stdin
    pub input: Option<String>,
}

/// Result of a tool execution.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolResult {
    /// Whether execution was successful
    pub success: bool,

    /// Exit code
    pub exit_code: i32,

    /// Standard output
    pub stdout: String,

    /// Standard error
    pub stderr: String,

    /// Duration in milliseconds
    pub duration_ms: u64,
}

/// Executor for running tools in a sandboxed environment.
#[allow(dead_code)]
pub struct SandboxExecutor {
    timeout_seconds: u64,
    allow_network: bool,
    work_dir: PathBuf,
}

impl SandboxExecutor {
    /// Create a new sandbox executor.
    pub fn new(timeout_seconds: u64, allow_network: bool, work_dir: PathBuf) -> Self {
        Self {
            timeout_seconds,
            allow_network,
            work_dir,
        }
    }

    /// Execute a tool in the sandbox.
    pub fn execute(&self, tool_name: &str, args: &[String]) -> Result<ToolResult> {
        let start = Instant::now();

        // Build the command
        let mut command = Command::new(tool_name);
        command
            .args(args)
            .current_dir(&self.work_dir)
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        // On Linux, we could add more isolation here with namespaces/seccomp
        // For now, we rely on command allowlisting

        // Execute with timeout
        let result = self.run_with_timeout(&mut command)?;

        let duration_ms = start.elapsed().as_millis() as u64;

        Ok(ToolResult {
            success: result.success,
            exit_code: result.exit_code,
            stdout: result.stdout,
            stderr: result.stderr,
            duration_ms,
        })
    }

    /// Execute a tool with input via stdin.
    pub fn execute_with_input(
        &self,
        tool_name: &str,
        args: &[String],
        input: &str,
    ) -> Result<ToolResult> {
        let start = Instant::now();

        let mut command = Command::new(tool_name);
        command
            .args(args)
            .current_dir(&self.work_dir)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        let mut child = command
            .spawn()
            .map_err(|e| Error::Sandbox(SandboxError::ExecutionFailed(e.to_string())))?;

        // Write input
        if let Some(mut stdin) = child.stdin.take() {
            use std::io::Write;
            let _ = stdin.write_all(input.as_bytes());
        }

        // Wait for completion with timeout
        let timeout = Duration::from_secs(self.timeout_seconds);
        let output = match child.wait_with_output() {
            Ok(output) => output,
            Err(e) => {
                return Err(Error::Sandbox(SandboxError::ExecutionFailed(e.to_string())));
            }
        };

        if start.elapsed() > timeout {
            return Err(Error::Sandbox(SandboxError::Timeout(self.timeout_seconds)));
        }

        let duration_ms = start.elapsed().as_millis() as u64;

        Ok(ToolResult {
            success: output.status.success(),
            exit_code: output.status.code().unwrap_or(-1),
            stdout: String::from_utf8_lossy(&output.stdout).to_string(),
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
            duration_ms,
        })
    }

    fn run_with_timeout(&self, command: &mut Command) -> Result<ToolResult> {
        let child = command
            .spawn()
            .map_err(|e| Error::Sandbox(SandboxError::ExecutionFailed(e.to_string())))?;

        let start = Instant::now();
        let timeout = Duration::from_secs(self.timeout_seconds);

        let output = child
            .wait_with_output()
            .map_err(|e| Error::Sandbox(SandboxError::ExecutionFailed(e.to_string())))?;

        if start.elapsed() > timeout {
            return Err(Error::Sandbox(SandboxError::Timeout(self.timeout_seconds)));
        }

        Ok(ToolResult {
            success: output.status.success(),
            exit_code: output.status.code().unwrap_or(-1),
            stdout: String::from_utf8_lossy(&output.stdout).to_string(),
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
            duration_ms: start.elapsed().as_millis() as u64,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_executor_echo() {
        let executor = SandboxExecutor::new(30, false, PathBuf::from("."));
        let result = executor.execute("echo", &["hello".to_string()]);

        assert!(result.is_ok());
        let result = result.unwrap();
        assert!(result.success);
        assert!(result.stdout.contains("hello"));
    }

    #[test]
    fn test_executor_with_multiple_args() {
        let executor = SandboxExecutor::new(30, false, PathBuf::from("."));
        let result = executor.execute(
            "echo",
            &["hello".to_string(), "world".to_string(), "test".to_string()],
        );

        assert!(result.is_ok());
        let result = result.unwrap();
        assert!(result.success);
        assert!(result.stdout.contains("hello"));
        assert!(result.stdout.contains("world"));
        assert!(result.stdout.contains("test"));
    }

    #[test]
    fn test_executor_nonexistent_command() {
        let executor = SandboxExecutor::new(30, false, PathBuf::from("."));
        let result = executor.execute("nonexistent_command_12345", &[]);

        assert!(result.is_err());
    }

    #[test]
    fn test_executor_with_stdin() {
        let executor = SandboxExecutor::new(30, false, PathBuf::from("."));
        let result = executor.execute_with_input("cat", &[], "test input");

        assert!(result.is_ok());
        let result = result.unwrap();
        assert!(result.success);
        assert!(result.stdout.contains("test input"));
    }

    #[test]
    fn test_executor_exit_code() {
        let executor = SandboxExecutor::new(30, false, PathBuf::from("."));

        // Test successful exit (exit code 0)
        let result = executor.execute("true", &[]);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert!(result.success);
        assert_eq!(result.exit_code, 0);

        // Test failure exit (exit code 1)
        let result = executor.execute("false", &[]);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert!(!result.success);
        assert_ne!(result.exit_code, 0);
    }

    #[test]
    fn test_tool_execution_struct() {
        let exec = ToolExecution {
            name: "echo".to_string(),
            args: vec!["hello".to_string()],
            input: None,
        };

        assert_eq!(exec.name, "echo");
        assert_eq!(exec.args.len(), 1);
        assert!(exec.input.is_none());
    }

    #[test]
    fn test_tool_result_struct() {
        let result = ToolResult {
            success: true,
            exit_code: 0,
            stdout: "output".to_string(),
            stderr: "".to_string(),
            duration_ms: 100,
        };

        assert!(result.success);
        assert_eq!(result.exit_code, 0);
        assert_eq!(result.stdout, "output");
        assert_eq!(result.duration_ms, 100);
    }
}
