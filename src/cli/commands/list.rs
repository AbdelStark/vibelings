//! List command implementation.

use crate::config::load_progress;
use crate::runner::ExerciseRunner;
use crate::Result;
use console::style;

/// Run the list command.
pub async fn run(track: Option<&str>, show_all: bool) -> Result<()> {
    println!("{}", style("📚 Vibelings Exercises").cyan().bold());
    println!();

    let runner = ExerciseRunner::new()?;
    let exercises = runner.discover_exercises()?;
    let progress = load_progress().unwrap_or_default();
    let completed = progress.completed_exercises();

    let mut current_track = String::new();
    let mut exercise_count = 0;
    let mut completed_count = 0;

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
            if !current_track.is_empty() {
                println!();
            }
            println!(
                "{}",
                style(format!(
                    "═══ {} ═══",
                    exercise.manifest.exercise.track.display_name()
                ))
                .bold()
            );
            current_track = track_name.to_string();
        }

        exercise_count += 1;
        let status = progress.get_status(&exercise.full_id());
        let status_symbol = status.symbol();

        if status == crate::ExerciseStatus::Completed {
            completed_count += 1;
        }

        let locked = if !prerequisites_met && status != crate::ExerciseStatus::Completed {
            style(" 🔒").dim()
        } else {
            style("")
        };

        println!(
            "  {} {} {}{}",
            status_symbol,
            style(&exercise.manifest.exercise.id).white(),
            style(&exercise.manifest.exercise.title).dim(),
            locked
        );
    }

    println!();
    println!(
        "Progress: {}/{} exercises completed",
        style(completed_count).green(),
        style(exercise_count).white()
    );

    if !show_all {
        println!();
        println!(
            "Use {} to see all exercises including locked ones",
            style("--all").cyan()
        );
    }

    Ok(())
}
