//! Progress command implementation - curriculum progress dashboard.

use crate::cli::commands::json_output::print_json;
use crate::cli::ui::{self, icons};
use crate::config::load_progress;
use crate::runner::ExerciseRunner;
use crate::ExerciseStatus;
use crate::Result;
use console::style;
use serde::Serialize;
use std::collections::HashMap;

/// JSON output for the progress command.
#[derive(Debug, Serialize)]
pub struct ProgressOutput {
    /// Per-track progress
    pub tracks: Vec<TrackProgress>,
    /// Overall statistics
    pub summary: ProgressSummary,
}

/// Progress for a single track.
#[derive(Debug, Serialize)]
pub struct TrackProgress {
    /// Track name
    pub name: String,
    /// Track display name
    pub display_name: String,
    /// Total exercises in track
    pub total: usize,
    /// Completed exercises
    pub completed: usize,
    /// In-progress exercises
    pub in_progress: usize,
    /// Pending exercises
    pub pending: usize,
    /// Completion percentage
    pub completion_percent: f64,
}

/// Overall progress summary.
#[derive(Debug, Serialize)]
pub struct ProgressSummary {
    /// Total exercises
    pub total_exercises: usize,
    /// Completed exercises
    pub completed_exercises: usize,
    /// In-progress exercises
    pub in_progress_exercises: usize,
    /// Pending exercises
    pub pending_exercises: usize,
    /// Overall completion percentage
    pub completion_percent: f64,
    /// Total tokens used
    pub total_tokens: u64,
    /// Total cost in USD
    pub total_cost_usd: f64,
    /// Number of tracks
    pub tracks_count: usize,
    /// Number of tracks completed (100%)
    pub tracks_completed: usize,
}

/// Run the progress command.
pub async fn run(json_output: bool) -> Result<()> {
    let runner = ExerciseRunner::new()?;
    let exercises = runner.discover_exercises()?;
    let progress = load_progress().unwrap_or_default();

    // Collect track statistics
    let mut track_stats: HashMap<String, (String, usize, usize, usize, usize)> = HashMap::new();

    for exercise in &exercises {
        let track_name = exercise.manifest.exercise.track.dir_name().to_string();
        let display_name = exercise.manifest.exercise.track.display_name().to_string();
        let status = progress.get_status(&exercise.full_id());

        let entry = track_stats
            .entry(track_name)
            .or_insert((display_name, 0, 0, 0, 0));
        entry.1 += 1; // total

        match status {
            ExerciseStatus::Completed => entry.2 += 1,
            ExerciseStatus::InProgress => entry.3 += 1,
            _ => entry.4 += 1, // pending
        }
    }

    // Build track progress list
    let mut tracks: Vec<TrackProgress> = track_stats
        .into_iter()
        .map(
            |(name, (display_name, total, completed, in_progress, pending))| {
                let completion_percent = if total > 0 {
                    (completed as f64 / total as f64) * 100.0
                } else {
                    0.0
                };
                TrackProgress {
                    name,
                    display_name,
                    total,
                    completed,
                    in_progress,
                    pending,
                    completion_percent,
                }
            },
        )
        .collect();

    // Sort tracks by a consistent order
    tracks.sort_by(|a, b| {
        let order = |s: &str| match s {
            "fundamentals" => 0,
            "mcp" => 1,
            "workflows" => 2,
            "production" => 3,
            "context" => 4,
            _ => 5,
        };
        order(&a.name).cmp(&order(&b.name))
    });

    // Calculate summary
    let total_exercises: usize = tracks.iter().map(|t| t.total).sum();
    let completed_exercises: usize = tracks.iter().map(|t| t.completed).sum();
    let in_progress_exercises: usize = tracks.iter().map(|t| t.in_progress).sum();
    let pending_exercises: usize = tracks.iter().map(|t| t.pending).sum();
    let tracks_completed = tracks
        .iter()
        .filter(|t| t.total > 0 && t.completed == t.total)
        .count();

    let completion_percent = if total_exercises > 0 {
        (completed_exercises as f64 / total_exercises as f64) * 100.0
    } else {
        0.0
    };

    // Token/cost totals from progress data
    let total_tokens: u64 = progress.exercises.values().map(|e| e.total_tokens).sum();
    let total_cost_usd: f64 = progress.exercises.values().map(|e| e.total_cost).sum();

    let summary = ProgressSummary {
        total_exercises,
        completed_exercises,
        in_progress_exercises,
        pending_exercises,
        completion_percent,
        total_tokens,
        total_cost_usd,
        tracks_count: tracks.len(),
        tracks_completed,
    };

    if json_output {
        let output = ProgressOutput { tracks, summary };
        return print_json(&output);
    }

    // Human-readable output
    ui::print_command_header(icons::TROPHY, "Curriculum Progress");

    // Overall progress bar at top
    println!();
    let bar_width = 40;
    let filled = if total_exercises > 0 {
        (completed_exercises * bar_width) / total_exercises
    } else {
        0
    };
    let empty = bar_width - filled;

    let bar_color = if completion_percent >= 100.0 {
        style("█".repeat(filled)).green()
    } else if completion_percent >= 50.0 {
        style("█".repeat(filled)).cyan()
    } else {
        style("█".repeat(filled)).yellow()
    };

    println!(
        "  [{}{}] {:.0}%",
        bar_color,
        style("░".repeat(empty)).dim(),
        completion_percent
    );
    println!(
        "  {} / {} exercises completed",
        style(completed_exercises).green().bold(),
        style(total_exercises).white()
    );

    // Track-by-track progress
    println!();
    ui::section_header("Track Progress");
    println!();

    for track in &tracks {
        let track_bar_width = 20;
        let track_filled = if track.total > 0 {
            (track.completed * track_bar_width) / track.total
        } else {
            0
        };
        let track_empty = track_bar_width - track_filled;

        let status_icon = if track.completed == track.total && track.total > 0 {
            format!("{}", style(icons::CHECK).green())
        } else if track.completed > 0 || track.in_progress > 0 {
            format!("{}", style(icons::IN_PROGRESS).yellow())
        } else {
            format!("{}", style(icons::PENDING).dim())
        };

        let bar_style = if track.completed == track.total && track.total > 0 {
            style("█".repeat(track_filled)).green()
        } else {
            style("█".repeat(track_filled)).cyan()
        };

        println!(
            "  {} {:24} [{}{}] {}/{}",
            status_icon,
            style(&track.display_name).white(),
            bar_style,
            style("░".repeat(track_empty)).dim(),
            style(track.completed).green(),
            style(track.total).dim()
        );
    }

    // Statistics
    println!();
    ui::section_header("Statistics");
    println!();

    println!(
        "  {} Tracks completed:     {}/{}",
        icons::BULLET,
        style(tracks_completed).cyan().bold(),
        style(tracks.len()).dim()
    );
    println!(
        "  {} Exercises completed:  {}/{}",
        icons::BULLET,
        style(completed_exercises).cyan().bold(),
        style(total_exercises).dim()
    );
    println!(
        "  {} In progress:          {}",
        icons::BULLET,
        style(in_progress_exercises).yellow()
    );

    if total_tokens > 0 {
        println!();
        println!(
            "  {} Total tokens used:    {}",
            icons::BULLET,
            style(format_tokens(total_tokens)).cyan()
        );
        println!(
            "  {} Total cost:           {}",
            icons::BULLET,
            style(format!("${:.4}", total_cost_usd)).green()
        );
    }

    // Motivational message
    println!();
    if completion_percent >= 100.0 {
        println!(
            "  {} {}",
            icons::TROPHY,
            style("Congratulations! You've completed all exercises!")
                .green()
                .bold()
        );
    } else if completion_percent >= 75.0 {
        println!(
            "  {} {}",
            icons::STAR,
            style("Almost there! Keep up the great work!").cyan().bold()
        );
    } else if completion_percent >= 50.0 {
        println!(
            "  {} {}",
            icons::INFO,
            style("Halfway there! You're making excellent progress.").cyan()
        );
    } else if completed_exercises > 0 {
        println!(
            "  {} {}",
            icons::INFO,
            style("Great start! Keep learning and practicing.").dim()
        );
    } else {
        println!(
            "  {} {}",
            icons::INFO,
            style("Run 'vibelings' to start your journey!").dim()
        );
    }

    println!();

    Ok(())
}

/// Format token count with K/M suffixes for readability.
fn format_tokens(tokens: u64) -> String {
    if tokens >= 1_000_000 {
        format!("{:.1}M", tokens as f64 / 1_000_000.0)
    } else if tokens >= 1_000 {
        format!("{:.1}K", tokens as f64 / 1_000.0)
    } else {
        tokens.to_string()
    }
}
