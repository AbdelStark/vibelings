//! Vibelings CLI entrypoint.
//!
//! "Rustlings for agentic programming" — learn to build reliable agentic AI systems.

use std::process::ExitCode;
use vibelings::cli;

#[tokio::main]
async fn main() -> ExitCode {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing::Level::WARN.into()),
        )
        .with_target(false)
        .without_time()
        .init();

    match cli::run().await {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("Error: {}", e);
            ExitCode::FAILURE
        }
    }
}
