//! List command implementation - beautiful exercise browser.

use crate::cli::commands::json_output::{
    print_json, status_to_string, ExerciseInfo, ListOutput, ListSummary,
};
use crate::cli::ui::{self, icons};
use crate::config::load_progress;
use crate::runner::ExerciseRunner;
use crate::ExerciseStatus;
use crate::Result;
use console::style;

/// Run the list command.
pub async fn run(track: Option<&str>, show_all: bool, json_output: bool) -> Result<()> {
    let runner = ExerciseRunner::new()?;
    let exercises = runner.discover_exercises()?;
    let progress = load_progress().unwrap_or_default();
    let completed = progress.completed_exercises();

    // Collect exercise data
    let mut exercise_infos: Vec<ExerciseInfo> = Vec::new();
    let mut total_count = 0;
    let mut completed_count = 0;
    let mut in_progress_count = 0;

    for exercise in &exercises {
        let track_name = exercise.manifest.exercise.track.dir_name();

        // Filter by track if specified
        if let Some(filter_track) = track {
            if track_name != filter_track {
                continue;
            }
        }

        let prerequisites_met = exercise.prerequisites_met(&completed);

        // Skip locked exercises unless --all is specified (for human output only)
        if !json_output
            && !show_all
            && !prerequisites_met
            && !completed.contains(&exercise.full_id())
        {
            continue;
        }

        total_count += 1;
        let status = progress.get_status(&exercise.full_id());

        match status {
            ExerciseStatus::Completed => completed_count += 1,
            ExerciseStatus::InProgress => in_progress_count += 1,
            _ => {}
        }

        exercise_infos.push(ExerciseInfo {
            id: exercise.full_id(),
            title: exercise.manifest.exercise.title.clone(),
            track: track_name.to_string(),
            status: status_to_string(&status),
            unlocked: prerequisites_met || completed.contains(&exercise.full_id()),
            difficulty: exercise.manifest.exercise.difficulty,
            prerequisites: exercise.manifest.exercise.prerequisites.clone(),
        });
    }

    // Calculate summary
    let pending_count = total_count - completed_count - in_progress_count;
    let completion_percent = if total_count > 0 {
        (completed_count as f64 / total_count as f64) * 100.0
    } else {
        0.0
    };

    if json_output {
        // JSON output
        let output = ListOutput {
            exercises: exercise_infos,
            summary: ListSummary {
                total: total_count,
                completed: completed_count,
                in_progress: in_progress_count,
                pending: pending_count,
                completion_percent,
            },
        };
        return print_json(&output);
    }

    // Human-readable output
    ui::print_command_header(icons::BOOK, "Exercise Library");

    let mut current_track = String::new();
    let mut track_exercise_count = 0;
    let mut track_completed_count = 0;

    for info in &exercise_infos {
        // Print track header when it changes
        if info.track != current_track {
            // Print track summary for previous track
            if !current_track.is_empty() && track_exercise_count > 0 {
                print_track_progress(track_completed_count, track_exercise_count);
                println!();
            }

            // New track header
            println!();
            let display_name = match info.track.as_str() {
                "fundamentals" => "Agentic Fundamentals",
                "mcp" => "MCP in Practice",
                "workflows" => "Workflow Orchestration",
                "production" => "Production Engineering",
                _ => &info.track,
            };
            print_track_header(display_name);
            println!();

            current_track = info.track.clone();
            track_exercise_count = 0;
            track_completed_count = 0;
        }

        track_exercise_count += 1;

        let status = match info.status.as_str() {
            "completed" => {
                track_completed_count += 1;
                ExerciseStatus::Completed
            }
            "in_progress" => ExerciseStatus::InProgress,
            "flaky" => ExerciseStatus::Flaky,
            "needs_reruns" => ExerciseStatus::NeedsReruns,
            "experimental" => ExerciseStatus::Experimental,
            _ => ExerciseStatus::Pending,
        };

        // Status symbol with color
        let status_icon = ui::status_symbol(&status);

        // Locked indicator
        let locked = if !info.unlocked && status != ExerciseStatus::Completed {
            format!(" {}", style(icons::LOCKED).dim())
        } else {
            String::new()
        };

        // Difficulty stars
        let stars: String = (0..info.difficulty)
            .map(|_| format!("{}", style(icons::STAR).yellow()))
            .collect();

        // Extract exercise ID from full ID (e.g., "fundamentals/json_01" -> "json_01")
        let exercise_id = info.id.split('/').next_back().unwrap_or(&info.id);

        println!(
            "     {} {:16} {} {}{}",
            status_icon,
            style(exercise_id).white(),
            style(&info.title).dim(),
            stars,
            locked
        );
    }

    // Print final track summary
    if !current_track.is_empty() && track_exercise_count > 0 {
        print_track_progress(track_completed_count, track_exercise_count);
    }

    // Overall progress
    println!();
    ui::section_header("Overall Progress");
    ui::print_progress_bar(completed_count, total_count);

    if !show_all {
        println!();
        println!(
            "  {} Use {} to see all exercises including locked ones",
            icons::INFO,
            style("--all").cyan()
        );
    }

    println!();

    Ok(())
}

fn print_track_header(name: &str) {
    println!(
        "  {}{}{}",
        style(ui::box_chars::HORIZONTAL.repeat(3)).cyan(),
        style(format!(" {} ", name)).cyan().bold(),
        style(ui::box_chars::HORIZONTAL.repeat(30)).cyan()
    );
}

fn print_track_progress(completed: usize, total: usize) {
    let percentage = if total > 0 {
        (completed as f64 / total as f64 * 100.0) as usize
    } else {
        0
    };

    println!(
        "     {} {}/{} completed ({}%)",
        icons::BULLET,
        style(completed).green(),
        style(total).dim(),
        style(percentage).cyan()
    );
}
