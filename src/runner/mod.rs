//! Exercise runner and orchestration.
//!
//! Handles discovering exercises, running them, and coordinating
//! with the grader and sandbox.

mod discovery;
mod executor;

pub use discovery::ExerciseRunner;
pub use executor::RunResult;
