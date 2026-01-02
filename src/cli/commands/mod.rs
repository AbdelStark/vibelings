//! CLI command implementations.

pub mod cost;
pub mod doctor;
pub mod hint;
pub mod init;
pub mod json_output;
pub mod list;
pub mod progress;
pub mod replay;
pub mod reset;
pub mod run;
pub mod verify;
pub mod watch;

use clap::{Parser, Subcommand};

/// Vibelings - Rustlings for agentic programming
#[derive(Parser)]
#[command(name = "vibelings")]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    /// Output results as JSON (for scripting and CI integration)
    #[arg(long, global = true)]
    pub json: bool,

    /// Subcommand to run (default: watch mode)
    #[command(subcommand)]
    pub command: Option<Commands>,
}

/// Available commands.
#[derive(Subcommand)]
pub enum Commands {
    /// Initialize a new vibelings workspace
    Init {
        /// Start with a specific track
        #[arg(short, long)]
        track: Option<String>,
    },

    /// Run a specific exercise
    Run {
        /// Exercise identifier (e.g., "fundamentals/json_01")
        exercise: String,

        /// Show verbose output including traces
        #[arg(short, long)]
        verbose: bool,

        /// Preview exercise without running (shows prompt and expected schema)
        #[arg(short, long)]
        dry_run: bool,
    },

    /// List all exercises with their status
    List {
        /// Filter by track
        #[arg(short, long)]
        track: Option<String>,

        /// Show all exercises including locked ones
        #[arg(short, long)]
        all: bool,

        /// Search exercises by keyword (matches ID, title, description)
        #[arg(short, long)]
        search: Option<String>,
    },

    /// Show hints for the current exercise
    Hint {
        /// Exercise to show hints for (default: current)
        #[arg(short, long)]
        exercise: Option<String>,

        /// Hint level (1-3, higher = more detailed)
        #[arg(short, long, default_value = "1")]
        level: u8,
    },

    /// Verify all completed exercises still pass
    Verify {
        /// Verify specific exercise only
        #[arg(short, long)]
        exercise: Option<String>,
    },

    /// Replay a previous run for debugging
    Replay {
        /// Run ID to replay
        run_id: String,
    },

    /// Check environment and configuration
    Doctor {
        /// Perform full API connectivity test (may incur small cost)
        #[arg(short, long)]
        full: bool,
    },

    /// Show token costs per exercise
    Cost {
        /// Show costs for specific exercise only
        #[arg(short, long)]
        exercise: Option<String>,
    },

    /// Show curriculum progress dashboard
    Progress,

    /// Reset an exercise to its starter state
    Reset {
        /// Exercise to reset
        exercise: String,

        /// Skip confirmation prompt
        #[arg(short, long)]
        force: bool,
    },
}
