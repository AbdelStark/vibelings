//! # Vibelings
//!
//! "Rustlings for agentic programming" — a terminal-first, open-source,
//! exercise-driven curriculum for learning to build reliable agentic AI systems.
//!
//! ## Core Philosophy
//!
//! 1. **Contracts over vibes**: Schemas, tool interfaces, explicit success criteria
//! 2. **Observability first**: Traces, logs, cost/latency visibility
//! 3. **Deterministic scaffolding around non-deterministic cores**: Simulation environments,
//!    constrained tools, replayable traces
//! 4. **Security posture by default**: Least-privilege tools, sandboxing, explicit user consent
//!
//! ## Architecture
//!
//! - [`cli`] - Command-line interface commands
//! - [`config`] - Configuration loading and validation
//! - [`provider`] - Model provider abstraction (OpenRouter, OpenAI, Anthropic, local)
//! - [`runner`] - Exercise runner and orchestration
//! - [`grader`] - Grading engine (schema validation, invariants, multi-run)
//! - [`sandbox`] - Tool sandbox and security
//! - [`trace`] - Trace capture and replay

pub mod cli;
pub mod config;
pub mod grader;
pub mod provider;
pub mod runner;
pub mod sandbox;
pub mod trace;

mod error;
mod exercise;

pub use error::{Error, Result};
pub use exercise::{
    Exercise, ExerciseManifest, ExerciseMetadata, ExerciseRequirements, ExerciseRunConfig,
    ExerciseStatus, GraderConfig, GraderType, Track,
};

/// Application version from Cargo.toml
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Application name
pub const NAME: &str = env!("CARGO_PKG_NAME");
