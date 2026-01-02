//! List command implementation - beautiful exercise browser.

use crate::cli::ui::{self, icons};
use crate::config::load_progress;
use crate::runner::ExerciseRunner;
use crate::Result;
use console::style;

/// Run the list command.
pub async fn run(track: Option<&str>, show_all: bool) -> Result<()> {
    ui::print_command_header(icons::BOOK, "Exercise Library");

    let runner = ExerciseRunner::new()?;
    let exercises = runner.discover_exercises()?;
    let progress = load_progress().unwrap_or_default();
    let completed = progress.completed_exercises();

    let mut current_track = String::new();
    let mut exercise_count = 0;
    let mut completed_count = 0;
    let mut track_exercise_count = 0;
    let mut track_completed_count = 0;

    for exercise in &exercises {
        let track_name = exercise.manifest.exercise.track.dir_name();

        // Filter by track if specified
        if let Some(filter_track) = track {
            if track_name != filter_track {
                continue;
            }
        }

        // Check if prerequisites are met
        let prerequisites_met = exercise.prerequisites_met(&completed);

        // Skip locked exercises unless --all is specified
        if !show_all && !prerequisites_met && !completed.contains(&exercise.full_id()) {
            continue;
        }

        // Print track header when it changes
        if track_name != current_track {
            // Print track summary for previous track
            if !current_track.is_empty() && track_exercise_count > 0 {
                print_track_progress(track_completed_count, track_exercise_count);
                println!();
            }

            // New track header
            println!();
            print_track_header(exercise.manifest.exercise.track.display_name());
            println!();

            current_track = track_name.to_string();
            track_exercise_count = 0;
            track_completed_count = 0;
        }

        exercise_count += 1;
        track_exercise_count += 1;
        let status = progress.get_status(&exercise.full_id());

        if status == crate::ExerciseStatus::Completed {
            completed_count += 1;
            track_completed_count += 1;
        }

        // Status symbol with color
        let status_icon = ui::status_symbol(&status);

        // Locked indicator
        let locked = if !prerequisites_met && status != crate::ExerciseStatus::Completed {
            format!(" {}", style(icons::LOCKED).dim())
        } else {
            String::new()
        };

        // Difficulty stars
        let difficulty = exercise.manifest.exercise.difficulty;
        let stars: String = (0..difficulty)
            .map(|_| format!("{}", style(icons::STAR).yellow()))
            .collect();

        println!(
            "     {} {:16} {} {}{}",
            status_icon,
            style(&exercise.manifest.exercise.id).white(),
            style(&exercise.manifest.exercise.title).dim(),
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
    ui::print_progress_bar(completed_count, exercise_count);

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
