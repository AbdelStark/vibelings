//! Watch mode implementation.

use crate::config::load_progress;
use crate::runner::ExerciseRunner;
use crate::Result;
use console::{style, Term};
use notify::RecursiveMode;
use notify_debouncer_mini::{new_debouncer, DebouncedEventKind};
use std::sync::mpsc::channel;
use std::time::Duration;

/// Run the watch command (default mode).
pub async fn run() -> Result<()> {
    let term = Term::stdout();

    // Clear screen and show welcome
    term.clear_screen()?;
    println!("{}", style("🎯 Vibelings - Watch Mode").cyan().bold());
    println!();

    let runner = ExerciseRunner::new()?;
    let progress = load_progress().unwrap_or_default();

    // Find the current or next exercise
    let current_exercise = find_current_exercise(&runner, &progress)?;

    if current_exercise.is_none() {
        println!(
            "{}",
            style("🎉 Congratulations! All exercises completed!")
                .green()
                .bold()
        );
        println!();
        println!(
            "Run {} to verify all exercises.",
            style("vibelings verify").cyan()
        );
        return Ok(());
    }

    let current_exercise = current_exercise.unwrap();

    // Display current exercise info
    display_exercise_info(&runner, &current_exercise)?;

    // Set up file watcher
    let (tx, rx) = channel();

    let mut debouncer =
        new_debouncer(Duration::from_millis(500), tx).expect("Failed to create file watcher");

    debouncer
        .watcher()
        .watch(std::path::Path::new("exercises"), RecursiveMode::Recursive)
        .expect("Failed to watch exercises directory");

    println!();
    println!(
        "{}",
        style("Watching for changes... Press Ctrl+C to exit").dim()
    );
    println!();
    println!(
        "  {} hint  {} next  {} list  {} quit",
        style("[h]").cyan(),
        style("[n]").cyan(),
        style("[l]").cyan(),
        style("[q]").cyan(),
    );

    // Main watch loop
    loop {
        match rx.recv_timeout(Duration::from_millis(100)) {
            Ok(Ok(events)) => {
                for event in events {
                    if event.kind == DebouncedEventKind::Any {
                        println!();
                        println!("{}", style("File changed, re-running exercise...").dim());
                        println!();

                        // Re-run the current exercise
                        match runner.run_exercise(&current_exercise, false).await {
                            Ok(result) => {
                                if result.passed {
                                    println!("{}", style("✅ PASSED!").green().bold());
                                    println!();
                                    println!(
                                        "Press {} to continue to the next exercise.",
                                        style("[n]").cyan()
                                    );
                                } else {
                                    println!("{}", style("❌ Not quite right. Keep trying!").red());
                                }
                            }
                            Err(e) => {
                                println!("{}: {}", style("Error").red(), e);
                            }
                        }
                    }
                }
            }
            Ok(Err(e)) => {
                eprintln!("Watch error: {:?}", e);
            }
            Err(_) => {
                // Timeout, check for keyboard input
                // TODO: Handle keyboard input for h/n/l/q
            }
        }
    }
}

fn find_current_exercise(
    runner: &ExerciseRunner,
    progress: &crate::config::UserProgress,
) -> Result<Option<String>> {
    // If there's a current exercise in progress, use that
    if let Some(ref current) = progress.current_exercise {
        return Ok(Some(current.clone()));
    }

    // Otherwise, find the first incomplete exercise
    let exercises = runner.discover_exercises()?;
    let completed = progress.completed_exercises();

    for exercise in exercises {
        let id = exercise.full_id();
        if !completed.contains(&id) && exercise.prerequisites_met(&completed) {
            return Ok(Some(id));
        }
    }

    Ok(None)
}

fn display_exercise_info(runner: &ExerciseRunner, exercise_id: &str) -> Result<()> {
    let exercise = runner.get_exercise(exercise_id)?;

    println!(
        "{}",
        style(format!(
            "━━━ Exercise: {} ━━━",
            exercise.manifest.exercise.id
        ))
        .cyan()
        .bold()
    );
    println!();
    println!(
        "{}: {}",
        style("Title").bold(),
        exercise.manifest.exercise.title
    );
    println!(
        "{}: {}",
        style("Track").bold(),
        exercise.manifest.exercise.track.display_name()
    );

    if let Some(ref desc) = exercise.manifest.exercise.description {
        println!();
        println!("{}", desc);
    }

    // Read and display README excerpt
    if exercise.readme_path.exists() {
        println!();
        let readme = std::fs::read_to_string(&exercise.readme_path)?;
        let excerpt: String = readme.lines().take(10).collect::<Vec<_>>().join("\n");
        println!("{}", style(excerpt).dim());
        if readme.lines().count() > 10 {
            println!("{}", style("...").dim());
        }
    }

    Ok(())
}
