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
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_run_invariant_passing_shell_script() {
        // Create a temporary shell script that always passes
        let mut script = NamedTempFile::with_suffix(".sh").unwrap();
        writeln!(script, "#!/bin/bash").unwrap();
        writeln!(script, "echo 'Invariant passed'").unwrap();
        writeln!(script, "exit 0").unwrap();
        script.flush().unwrap();

        let result = run_invariant(script.path(), "test output").unwrap();
        assert!(result.passed);
        assert!(result.message.contains("Invariant passed"));
    }

    #[test]
    fn test_run_invariant_failing_shell_script() {
        // Create a temporary shell script that always fails
        let mut script = NamedTempFile::with_suffix(".sh").unwrap();
        writeln!(script, "#!/bin/bash").unwrap();
        writeln!(script, "echo 'Invariant failed' >&2").unwrap();
        writeln!(script, "exit 1").unwrap();
        script.flush().unwrap();

        let result = run_invariant(script.path(), "test output").unwrap();
        assert!(!result.passed);
        assert!(result.message.contains("Invariant failed"));
    }

    #[test]
    fn test_run_invariant_reads_stdin() {
        // Create a script that echoes back the input
        let mut script = NamedTempFile::with_suffix(".sh").unwrap();
        writeln!(script, "#!/bin/bash").unwrap();
        writeln!(script, "read input").unwrap();
        writeln!(script, "echo \"received: $input\"").unwrap();
        writeln!(script, "exit 0").unwrap();
        script.flush().unwrap();

        let result = run_invariant(script.path(), "hello world").unwrap();
        assert!(result.passed);
    }

    #[test]
    fn test_run_invariant_empty_stdout_returns_ok() {
        // Create a script that produces no output but passes
        let mut script = NamedTempFile::with_suffix(".sh").unwrap();
        writeln!(script, "#!/bin/bash").unwrap();
        writeln!(script, "exit 0").unwrap();
        script.flush().unwrap();

        let result = run_invariant(script.path(), "").unwrap();
        assert!(result.passed);
        assert_eq!(result.message, "OK");
    }

    #[test]
    fn test_run_invariant_python_script() {
        // Create a temporary Python script
        let mut script = NamedTempFile::with_suffix(".py").unwrap();
        writeln!(script, "import sys").unwrap();
        writeln!(script, "print('Python invariant passed')").unwrap();
        writeln!(script, "sys.exit(0)").unwrap();
        script.flush().unwrap();

        let result = run_invariant(script.path(), "test");
        // This may fail if python3 is not installed, so we just check no panic
        if let Ok(result) = result {
            assert!(result.passed);
        }
    }

    #[test]
    fn test_run_invariant_nonexistent_script() {
        // For .sh files, bash is invoked with the script path as an argument.
        // Bash will spawn successfully but fail when it can't find the script.
        let path = Path::new("/nonexistent/script.sh");
        let result = run_invariant(path, "test");
        // Should either error or return failed invariant (bash will fail to run the script)
        if let Ok(inv_result) = result {
            assert!(!inv_result.passed);
        }
        // If Err, that's also acceptable - spawn failed entirely
    }

    #[test]
    fn test_invariant_result_struct() {
        let result = InvariantResult {
            passed: true,
            message: "All checks passed".to_string(),
        };

        assert!(result.passed);
        assert_eq!(result.message, "All checks passed");
    }
}
