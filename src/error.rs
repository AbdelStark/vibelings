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
    #[error("Configuration file not found at {0}")]
    NotFound(PathBuf),

    #[error("Invalid configuration: {0}")]
    Invalid(String),

    #[error("Missing required field: {0}")]
    MissingField(String),

    #[error("Invalid provider configuration: {0}")]
    InvalidProvider(String),

    #[error("Environment variable not set: {0}")]
    EnvVarNotSet(String),
}

/// Exercise-related errors.
#[derive(Error, Debug)]
pub enum ExerciseError {
    #[error("Exercise not found: {0}")]
    NotFound(String),

    #[error("Invalid manifest at {path}: {reason}")]
    InvalidManifest { path: PathBuf, reason: String },

    #[error("Exercise directory not found: {0}")]
    DirectoryNotFound(PathBuf),

    #[error("Missing starter files for exercise: {0}")]
    MissingStarterFiles(String),

    #[error("Prerequisites not met for exercise {exercise}: missing {missing:?}")]
    PrerequisitesNotMet {
        exercise: String,
        missing: Vec<String>,
    },
}

/// Provider-related errors.
#[derive(Error, Debug)]
pub enum ProviderError {
    #[error("API key not configured for provider: {0}")]
    ApiKeyNotConfigured(String),

    #[error("HTTP request failed: {0}")]
    HttpError(String),

    #[error("Rate limited by provider: {0}")]
    RateLimited(String),

    #[error("Model not supported: {0}")]
    ModelNotSupported(String),

    #[error("Provider response error: {status} - {message}")]
    ResponseError { status: u16, message: String },

    #[error("Invalid response format: {0}")]
    InvalidResponse(String),

    #[error("Feature not supported by model: {model} does not support {feature}")]
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
