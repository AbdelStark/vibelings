//! Command-line interface commands.
//!
//! Implements all CLI commands:
//! - `vibelings init` - Create workspace + config + first track
//! - `vibelings` (default) - Watch mode, reruns on file changes
//! - `vibelings run <exercise>` - Run single exercise once
//! - `vibelings list` - Interactive exercise list with status
//! - `vibelings hint` - Layered hints (static first, AI hint optional)
//! - `vibelings verify` - Run full test suite for completed exercises
//! - `vibelings replay <run_id>` - Replay trace for debugging
//! - `vibelings doctor` - Verify keys, model access, tool support
//! - `vibelings cost` - Show token costs per exercise
//! - `vibelings reset <exercise>` - Reset exercise to starter state

mod commands;

pub use commands::{Cli, Commands};

use crate::Result;
use clap::Parser;

/// Run the CLI application.
pub async fn run() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Init { track }) => commands::init::run(track.as_deref()).await,
        Some(Commands::Run { exercise, verbose }) => commands::run::run(exercise, *verbose).await,
        Some(Commands::List { track, all }) => commands::list::run(track.as_deref(), *all).await,
        Some(Commands::Hint { exercise, level }) => {
            commands::hint::run(exercise.as_deref(), *level).await
        }
        Some(Commands::Verify { exercise }) => commands::verify::run(exercise.as_deref()).await,
        Some(Commands::Replay { run_id }) => commands::replay::run(run_id).await,
        Some(Commands::Doctor) => commands::doctor::run().await,
        Some(Commands::Cost { exercise }) => commands::cost::run(exercise.as_deref()).await,
        Some(Commands::Reset { exercise, force }) => commands::reset::run(exercise, *force).await,
        None => {
            // Default: watch mode
            commands::watch::run().await
        }
    }
}
