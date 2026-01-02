//! Error types for the vibelings library.

use std::path::PathBuf;
use thiserror::Error;

/// Main error type for vibelings.
#[derive(Error, Debug)]
pub enum Error {
    /// Configuration-related errors
    #[error("Configuration error: {0}")]
    Config(#[from] ConfigError),

    /// Exercise-related errors
    #[error("Exercise error: {0}")]
    Exercise(#[from] ExerciseError),

    /// Provider-related errors
    #[error("Provider error: {0}")]
    Provider(#[from] ProviderError),

    /// Grading-related errors
    #[error("Grading error: {0}")]
    Grading(#[from] GradingError),

    /// Sandbox-related errors
    #[error("Sandbox error: {0}")]
    Sandbox(#[from] SandboxError),

    /// Trace-related errors
    #[error("Trace error: {0}")]
    Trace(#[from] TraceError),

    /// IO errors
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// JSON serialization/deserialization errors
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    /// TOML parsing errors
    #[error("TOML error: {0}")]
    Toml(#[from] toml::de::Error),
}

/// Configuration-related errors.
#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Configuration file not found at {0}. Run 'vibelings init' to create one")]
    NotFound(PathBuf),

    #[error("Invalid configuration: {0}")]
    Invalid(String),

    #[error("Missing required field: {0}")]
    MissingField(String),

    #[error("Invalid provider configuration: {0}")]
    InvalidProvider(String),

    #[error("Environment variable '{0}' not set. Run: export {0}=<your-api-key>")]
    EnvVarNotSet(String),
}

/// Exercise-related errors.
#[derive(Error, Debug)]
pub enum ExerciseError {
    #[error("Exercise '{0}' not found. Run 'vibelings list' to see available exercises")]
    NotFound(String),

    #[error("Invalid manifest at {path}: {reason}")]
    InvalidManifest { path: PathBuf, reason: String },

    #[error("Exercise directory not found: {0}")]
    DirectoryNotFound(PathBuf),

    #[error("Missing starter files for exercise: {0}")]
    MissingStarterFiles(String),

    #[error("Prerequisites not met for '{exercise}'. Complete first: {missing:?}")]
    PrerequisitesNotMet {
        exercise: String,
        missing: Vec<String>,
    },
}

/// Provider-related errors.
#[derive(Error, Debug)]
pub enum ProviderError {
    #[error("API key not configured for {0}. Check your configuration")]
    ApiKeyNotConfigured(String),

    #[error("Network error: {0}. Check your internet connection")]
    HttpError(String),

    #[error("Rate limited by provider. Wait a moment and try again. Details: {0}")]
    RateLimited(String),

    #[error("Model '{0}' not supported. Run 'vibelings doctor' to check available models")]
    ModelNotSupported(String),

    #[error("Provider error ({status}): {message}")]
    ResponseError { status: u16, message: String },

    #[error("Invalid response from provider: {0}")]
    InvalidResponse(String),

    #[error("Model '{model}' does not support {feature}. Try a different model")]
    FeatureNotSupported { model: String, feature: String },
}

/// Grading-related errors.
#[derive(Error, Debug)]
pub enum GradingError {
    #[error("Schema validation failed: {0}")]
    SchemaValidation(String),

    #[error("Schema file not found: {0}")]
    SchemaNotFound(PathBuf),

    #[error("Invariant check failed: {invariant}")]
    InvariantFailed { invariant: String },

    #[error("Invariant script not found: {0}")]
    InvariantScriptNotFound(PathBuf),

    #[error("Reliability threshold not met: {passed}/{total} runs passed, need {required}")]
    ReliabilityThresholdNotMet {
        passed: u32,
        total: u32,
        required: u32,
    },

    #[error("Grader script execution failed: {0}")]
    ScriptExecutionFailed(String),

    #[error("Invalid grader configuration: {0}")]
    InvalidConfig(String),

    #[error("Grader type not implemented: {0}")]
    NotImplemented(String),
}

/// Sandbox-related errors.
#[derive(Error, Debug)]
pub enum SandboxError {
    #[error("Command not allowed: {0}")]
    CommandNotAllowed(String),

    #[error("Network access denied")]
    NetworkDenied,

    #[error("Filesystem access denied: {0}")]
    FilesystemDenied(PathBuf),

    #[error("Tool execution timed out after {0} seconds")]
    Timeout(u64),

    #[error("Tool execution failed: {0}")]
    ExecutionFailed(String),

    #[error("Maximum tool calls exceeded: {0}")]
    MaxToolCallsExceeded(u32),
}

/// Trace-related errors.
#[derive(Error, Debug)]
pub enum TraceError {
    #[error("Trace not found: {0}")]
    NotFound(String),

    #[error("Invalid trace format: {0}")]
    InvalidFormat(String),

    #[error("Trace storage error: {0}")]
    StorageError(String),

    #[error("Trace replay failed: {0}")]
    ReplayFailed(String),
}

/// Result type alias using our Error type.
pub type Result<T> = std::result::Result<T, Error>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_error_not_found() {
        let err = ConfigError::NotFound(PathBuf::from("/etc/vibelings/config.toml"));
        let msg = format!("{}", err);
        assert!(msg.contains("not found"));
        assert!(msg.contains("vibelings init"));
    }

    #[test]
    fn test_config_error_invalid() {
        let err = ConfigError::Invalid("temperature must be between 0 and 2".to_string());
        let msg = format!("{}", err);
        assert!(msg.contains("Invalid"));
        assert!(msg.contains("temperature"));
    }

    #[test]
    fn test_config_error_env_var() {
        let err = ConfigError::EnvVarNotSet("OPENROUTER_API_KEY".to_string());
        let msg = format!("{}", err);
        assert!(msg.contains("OPENROUTER_API_KEY"));
        assert!(msg.contains("export"));
    }

    #[test]
    fn test_exercise_error_not_found() {
        let err = ExerciseError::NotFound("fundamentals/json_99".to_string());
        let msg = format!("{}", err);
        assert!(msg.contains("json_99"));
        assert!(msg.contains("vibelings list"));
    }

    #[test]
    fn test_exercise_error_invalid_manifest() {
        let err = ExerciseError::InvalidManifest {
            path: PathBuf::from("exercises/test/manifest.toml"),
            reason: "missing [grader] section".to_string(),
        };
        let msg = format!("{}", err);
        assert!(msg.contains("manifest"));
        assert!(msg.contains("missing [grader] section"));
    }

    #[test]
    fn test_exercise_error_prerequisites() {
        let err = ExerciseError::PrerequisitesNotMet {
            exercise: "tools_02".to_string(),
            missing: vec!["tools_01".to_string()],
        };
        let msg = format!("{}", err);
        assert!(msg.contains("tools_02"));
        assert!(msg.contains("tools_01"));
    }

    #[test]
    fn test_provider_error_rate_limited() {
        let err = ProviderError::RateLimited("Please wait 60 seconds".to_string());
        let msg = format!("{}", err);
        assert!(msg.contains("Rate limited"));
        assert!(msg.contains("Wait"));
    }

    #[test]
    fn test_provider_error_response() {
        let err = ProviderError::ResponseError {
            status: 500,
            message: "Internal server error".to_string(),
        };
        let msg = format!("{}", err);
        assert!(msg.contains("500"));
        assert!(msg.contains("Internal server error"));
    }

    #[test]
    fn test_provider_error_feature_not_supported() {
        let err = ProviderError::FeatureNotSupported {
            model: "gpt-3.5-turbo".to_string(),
            feature: "tool calling".to_string(),
        };
        let msg = format!("{}", err);
        assert!(msg.contains("gpt-3.5-turbo"));
        assert!(msg.contains("tool calling"));
    }

    #[test]
    fn test_grading_error_schema() {
        let err = GradingError::SchemaValidation("missing required field 'name'".to_string());
        let msg = format!("{}", err);
        assert!(msg.contains("Schema validation"));
        assert!(msg.contains("name"));
    }

    #[test]
    fn test_grading_error_reliability() {
        let err = GradingError::ReliabilityThresholdNotMet {
            passed: 2,
            total: 5,
            required: 4,
        };
        let msg = format!("{}", err);
        assert!(msg.contains("2/5"));
        assert!(msg.contains("need 4"));
    }

    #[test]
    fn test_grading_error_not_implemented() {
        let err = GradingError::NotImplemented("llm-judge".to_string());
        let msg = format!("{}", err);
        assert!(msg.contains("not implemented"));
        assert!(msg.contains("llm-judge"));
    }

    #[test]
    fn test_sandbox_error_timeout() {
        let err = SandboxError::Timeout(30);
        let msg = format!("{}", err);
        assert!(msg.contains("30"));
        assert!(msg.contains("timed out"));
    }

    #[test]
    fn test_sandbox_error_command_not_allowed() {
        let err = SandboxError::CommandNotAllowed("rm -rf".to_string());
        let msg = format!("{}", err);
        assert!(msg.contains("rm -rf"));
        assert!(msg.contains("not allowed"));
    }

    #[test]
    fn test_sandbox_error_max_tool_calls() {
        let err = SandboxError::MaxToolCallsExceeded(10);
        let msg = format!("{}", err);
        assert!(msg.contains("10"));
        assert!(msg.contains("exceeded"));
    }

    #[test]
    fn test_trace_error_not_found() {
        let err = TraceError::NotFound("trace-12345".to_string());
        let msg = format!("{}", err);
        assert!(msg.contains("trace-12345"));
        assert!(msg.contains("not found"));
    }

    #[test]
    fn test_error_conversion_from_config() {
        let config_err = ConfigError::Invalid("test".to_string());
        let err: Error = config_err.into();
        let msg = format!("{}", err);
        assert!(msg.contains("Configuration error"));
    }

    #[test]
    fn test_error_conversion_from_exercise() {
        let exercise_err = ExerciseError::NotFound("test".to_string());
        let err: Error = exercise_err.into();
        let msg = format!("{}", err);
        assert!(msg.contains("Exercise error"));
    }

    #[test]
    fn test_error_conversion_from_provider() {
        let provider_err = ProviderError::InvalidResponse("bad json".to_string());
        let err: Error = provider_err.into();
        let msg = format!("{}", err);
        assert!(msg.contains("Provider error"));
    }

    #[test]
    fn test_error_conversion_from_grading() {
        let grading_err = GradingError::SchemaValidation("test".to_string());
        let err: Error = grading_err.into();
        let msg = format!("{}", err);
        assert!(msg.contains("Grading error"));
    }

    #[test]
    fn test_error_conversion_from_sandbox() {
        let sandbox_err = SandboxError::NetworkDenied;
        let err: Error = sandbox_err.into();
        let msg = format!("{}", err);
        assert!(msg.contains("Sandbox error"));
    }

    #[test]
    fn test_error_conversion_from_trace() {
        let trace_err = TraceError::InvalidFormat("bad format".to_string());
        let err: Error = trace_err.into();
        let msg = format!("{}", err);
        assert!(msg.contains("Trace error"));
    }

    #[test]
    fn test_error_debug_format() {
        let err = Error::Config(ConfigError::Invalid("test".to_string()));
        let debug = format!("{:?}", err);
        assert!(debug.contains("Config"));
    }
}
