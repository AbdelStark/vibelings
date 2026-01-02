//! Watch mode implementation.

use crate::config::{load_progress, save_progress, UserProgress};
use crate::runner::ExerciseRunner;
use crate::Result;
use console::{style, Key, Term};
use notify::RecursiveMode;
use notify_debouncer_mini::{new_debouncer, DebouncedEventKind};
use std::sync::mpsc::{channel, Sender, TryRecvError};
use std::time::Duration;

/// Keyboard input event for watch mode.
#[derive(Debug, Clone)]
enum KeyboardEvent {
    Hint,
    Next,
    List,
    Quit,
    Unknown,
}

/// Spawn a thread to listen for keyboard input.
fn spawn_keyboard_listener(tx: Sender<KeyboardEvent>) {
    std::thread::spawn(move || {
        let term = Term::stdout();
        loop {
            if let Ok(key) = term.read_key() {
                let event = match key {
                    Key::Char('h') | Key::Char('H') => KeyboardEvent::Hint,
                    Key::Char('n') | Key::Char('N') => KeyboardEvent::Next,
                    Key::Char('l') | Key::Char('L') => KeyboardEvent::List,
                    Key::Char('q') | Key::Char('Q') => KeyboardEvent::Quit,
                    Key::Escape => KeyboardEvent::Quit,
                    _ => KeyboardEvent::Unknown,
                };

                // Send the event; if the receiver is dropped, exit the thread
                if tx.send(event).is_err() {
                    break;
                }
            }
        }
    });
}

/// Run the watch command (default mode).
pub async fn run() -> Result<()> {
    let term = Term::stdout();

    // Clear screen and show welcome
    term.clear_screen()?;
    println!("{}", style("🎯 Vibelings - Watch Mode").cyan().bold());
    println!();

    let runner = ExerciseRunner::new()?;
    let mut progress = load_progress().unwrap_or_default();

    // Find the current or next exercise
    let mut current_exercise = find_current_exercise(&runner, &progress)?;

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

    let mut exercise_id = current_exercise.clone().unwrap();

    // Display current exercise info
    display_exercise_info(&runner, &exercise_id)?;

    // Set up file watcher
    let (file_tx, file_rx) = channel();

    let mut debouncer =
        new_debouncer(Duration::from_millis(500), file_tx).expect("Failed to create file watcher");

    debouncer
        .watcher()
        .watch(std::path::Path::new("exercises"), RecursiveMode::Recursive)
        .expect("Failed to watch exercises directory");

    // Set up keyboard listener
    let (key_tx, key_rx) = channel();
    spawn_keyboard_listener(key_tx);

    // Track if current exercise has passed
    let mut current_passed = false;

    println!();
    println!(
        "{}",
        style("Watching for changes...").dim()
    );
    println!();
    display_key_hints();

    // Main watch loop
    loop {
        // Check for file changes
        match file_rx.recv_timeout(Duration::from_millis(100)) {
            Ok(Ok(events)) => {
                for event in events {
                    if event.kind == DebouncedEventKind::Any {
                        println!();
                        println!("{}", style("File changed, re-running exercise...").dim());
                        println!();

                        // Re-run the current exercise
                        match runner.run_exercise(&exercise_id, false).await {
                            Ok(result) => {
                                current_passed = result.passed;
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
                // Timeout - continue to check keyboard
            }
        }

        // Check for keyboard input
        match key_rx.try_recv() {
            Ok(KeyboardEvent::Hint) => {
                handle_hint(&runner, &exercise_id)?;
            }
            Ok(KeyboardEvent::Next) => {
                if current_passed {
                    // Mark current as completed and move to next
                    progress.mark_completed(&exercise_id);
                    save_progress(&progress)?;
                }

                // Find next exercise
                progress = load_progress().unwrap_or_default();
                current_exercise = find_current_exercise(&runner, &progress)?;

                if current_exercise.is_none() {
                    term.clear_screen()?;
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

                exercise_id = current_exercise.clone().unwrap();
                current_passed = false;

                // Clear screen and show new exercise
                term.clear_screen()?;
                println!("{}", style("🎯 Vibelings - Watch Mode").cyan().bold());
                println!();
                display_exercise_info(&runner, &exercise_id)?;
                println!();
                println!("{}", style("Watching for changes...").dim());
                println!();
                display_key_hints();
            }
            Ok(KeyboardEvent::List) => {
                handle_list(&runner, &progress)?;
            }
            Ok(KeyboardEvent::Quit) => {
                println!();
                println!("{}", style("👋 Goodbye!").cyan());
                return Ok(());
            }
            Ok(KeyboardEvent::Unknown) => {
                // Ignore unknown keys
            }
            Err(TryRecvError::Empty) => {
                // No keyboard input available
            }
            Err(TryRecvError::Disconnected) => {
                // Keyboard listener thread ended unexpectedly
                eprintln!("{}", style("Keyboard listener disconnected").red());
                break;
            }
        }
    }

    Ok(())
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

/// Display keyboard shortcut hints.
fn display_key_hints() {
    println!(
        "  {} hint  {} next  {} list  {} quit",
        style("[h]").cyan(),
        style("[n]").cyan(),
        style("[l]").cyan(),
        style("[q]").cyan(),
    );
}

/// Handle the hint command (h key).
fn handle_hint(runner: &ExerciseRunner, exercise_id: &str) -> Result<()> {
    println!();
    println!(
        "{}",
        style(format!("💡 Hints for: {}", exercise_id)).yellow().bold()
    );
    println!();

    let hints = runner.get_hints(exercise_id)?;

    if hints.is_empty() {
        println!("{}", style("No hints available for this exercise.").dim());
        println!();
        return Ok(());
    }

    // Show all hints progressively
    for (i, hint) in hints.iter().enumerate() {
        println!(
            "{} {}",
            style(format!("Hint {}:", i + 1)).yellow().bold(),
            hint
        );
        println!();
    }

    display_key_hints();
    Ok(())
}

/// Handle the list command (l key).
fn handle_list(runner: &ExerciseRunner, progress: &UserProgress) -> Result<()> {
    println!();
    println!("{}", style("📚 Exercises").cyan().bold());
    println!();

    let exercises = runner.discover_exercises()?;
    let completed = progress.completed_exercises();

    let mut current_track = String::new();
    let mut exercise_count = 0;
    let mut completed_count = 0;

    for exercise in &exercises {
        let track_name = exercise.manifest.exercise.track.dir_name();

        // Check if prerequisites are met
        let prerequisites_met = exercise.prerequisites_met(&completed);

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
        "Progress: {}/{} completed",
        style(completed_count).green(),
        style(exercise_count).white()
    );
    println!();
    display_key_hints();

    Ok(())
}
