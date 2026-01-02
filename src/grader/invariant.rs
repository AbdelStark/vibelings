//! Invariant script execution.

use crate::error::GradingError;
use crate::{Error, Result};
use std::path::Path;
use std::process::{Command, Stdio};

/// Result of running an invariant check.
pub struct InvariantResult {
    /// Whether the invariant passed
    pub passed: bool,

    /// Message from the invariant
    pub message: String,
}

/// Run an invariant script with the given output.
pub fn run_invariant(script_path: &Path, output: &str) -> Result<InvariantResult> {
    // Determine how to run the script based on extension
    let extension = script_path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("");

    let mut command = match extension {
        "sh" => {
            let mut cmd = Command::new("bash");
            cmd.arg(script_path);
            cmd
        }
        "py" => {
            let mut cmd = Command::new("python3");
            cmd.arg(script_path);
            cmd
        }
        _ => {
            // Try to run directly (might be executable)
            Command::new(script_path)
        }
    };

    // Pass the output to the script via stdin
    let child = command
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| Error::Grading(GradingError::ScriptExecutionFailed(e.to_string())))?;

    // Write output to stdin
    use std::io::Write;
    if let Some(mut stdin) = child.stdin {
        let _ = stdin.write_all(output.as_bytes());
    }

    let result = command
        .output()
        .map_err(|e| Error::Grading(GradingError::ScriptExecutionFailed(e.to_string())))?;

    let stdout = String::from_utf8_lossy(&result.stdout).to_string();
    let stderr = String::from_utf8_lossy(&result.stderr).to_string();

    let passed = result.status.success();
    let message = if passed {
        if stdout.is_empty() {
            "OK".to_string()
        } else {
            stdout.trim().to_string()
        }
    } else if stderr.is_empty() {
        stdout.trim().to_string()
    } else {
        stderr.trim().to_string()
    };

    Ok(InvariantResult { passed, message })
}

#[cfg(test)]
mod tests {
    // Tests would require creating temporary script files
}
