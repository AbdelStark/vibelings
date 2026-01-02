//! Watch mode implementation - the primary interactive experience.

use crate::cli::ui::{self, icons};
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
    Retry,
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
                    Key::Char('r') | Key::Char('R') => KeyboardEvent::Retry,
                    Key::Escape => KeyboardEvent::Quit,
                    _ => KeyboardEvent::Unknown,
                };

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
    term.clear_screen()?;

    // Show beautiful header
    ui::print_watch_header();

    let runner = ExerciseRunner::new()?;
    let mut progress = load_progress().unwrap_or_default();

    // Find the current or next exercise
    let mut current_exercise = find_current_exercise(&runner, &progress)?;

    if current_exercise.is_none() {
        ui::celebrate_completion();
        println!();
        ui::print_info(&format!(
            "Run {} to verify all exercises.",
            style("vibelings verify").cyan()
        ));
        println!();
        return Ok(());
    }

    let mut exercise_id = current_exercise.clone().unwrap();

    // Display current exercise info
    display_exercise_info(&runner, &exercise_id, &progress)?;

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
        "  {} {}",
        icons::CLOCK,
        style("Watching for changes...").dim()
    );
    print_key_hints_extended();

    // Main watch loop
    loop {
        // Check for file changes
        match file_rx.recv_timeout(Duration::from_millis(100)) {
            Ok(Ok(events)) => {
                for event in events {
                    if event.kind == DebouncedEventKind::Any {
                        // Run the exercise with a spinner
                        current_passed =
                            run_exercise_with_feedback(&runner, &exercise_id, &mut progress).await;
                    }
                }
            }
            Ok(Err(e)) => {
                eprintln!(
                    "  {} Watch error: {:?}",
                    style(icons::CROSS).red(),
                    e
                );
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
            Ok(KeyboardEvent::Retry) => {
                current_passed =
                    run_exercise_with_feedback(&runner, &exercise_id, &mut progress).await;
            }
            Ok(KeyboardEvent::Next) => {
                if current_passed {
                    progress.mark_completed(&exercise_id);
                    save_progress(&progress)?;
                }

                progress = load_progress().unwrap_or_default();
                current_exercise = find_current_exercise(&runner, &progress)?;

                if current_exercise.is_none() {
                    term.clear_screen()?;
                    ui::celebrate_completion();
                    println!();
                    ui::print_info(&format!(
                        "Run {} to verify all exercises.",
                        style("vibelings verify").cyan()
                    ));
                    println!();
                    return Ok(());
                }

                exercise_id = current_exercise.clone().unwrap();
                current_passed = false;

                // Clear screen and show new exercise
                term.clear_screen()?;
                ui::print_watch_header();
                display_exercise_info(&runner, &exercise_id, &progress)?;
                println!();
                println!(
                    "  {} {}",
                    icons::CLOCK,
                    style("Watching for changes...").dim()
                );
                print_key_hints_extended();
            }
            Ok(KeyboardEvent::List) => {
                handle_list(&runner, &progress)?;
            }
            Ok(KeyboardEvent::Quit) => {
                ui::print_goodbye();
                return Ok(());
            }
            Ok(KeyboardEvent::Unknown) => {}
            Err(TryRecvError::Empty) => {}
            Err(TryRecvError::Disconnected) => {
                ui::print_warning("Keyboard listener disconnected");
                break;
            }
        }
    }

    Ok(())
}

/// Run an exercise with visual feedback
async fn run_exercise_with_feedback(
    runner: &ExerciseRunner,
    exercise_id: &str,
    _progress: &mut UserProgress,
) -> bool {
    println!();
    let spinner = ui::create_spinner("Running exercise...");

    match runner.run_exercise(exercise_id, false).await {
        Ok(result) => {
            spinner.finish_and_clear();

            if result.passed {
                ui::celebrate_pass();
                println!();
                println!(
                    "     {} {:.1}s  {} ${:.4}",
                    style("Time:").dim(),
                    result.duration_secs,
                    style("Cost:").dim(),
                    result.cost_usd
                );
                println!();
                println!(
                    "  {} Press {} to continue to the next exercise",
                    icons::ARROW_RIGHT,
                    style("[n]").cyan().bold()
                );
            } else {
                println!();
                println!(
                    "  {} {}",
                    style(icons::CROSS).red(),
                    style("Not quite right. Keep trying!").red()
                );
                if let Some(ref error) = result.error_message {
                    println!();
                    println!("     {}", style(error).dim());
                }
                println!();
                println!(
                    "  {} Press {} for a hint",
                    icons::LIGHTBULB,
                    style("[h]").cyan()
                );
            }

            print_key_hints_extended();
            result.passed
        }
        Err(e) => {
            spinner.finish_and_clear();
            println!();
            println!(
                "  {} {}: {}",
                style(icons::CROSS).red(),
                style("Error").red().bold(),
                e
            );
            print_key_hints_extended();
            false
        }
    }
}

fn find_current_exercise(
    runner: &ExerciseRunner,
    progress: &UserProgress,
) -> Result<Option<String>> {
    if let Some(ref current) = progress.current_exercise {
        return Ok(Some(current.clone()));
    }

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

fn display_exercise_info(
    runner: &ExerciseRunner,
    exercise_id: &str,
    progress: &UserProgress,
) -> Result<()> {
    let exercise = runner.get_exercise(exercise_id)?;
    let exercises = runner.discover_exercises()?;
    let completed = progress.completed_exercises();

    // Count progress
    let total = exercises.len();
    let done = completed.len();

    // Progress indicator
    ui::print_progress_bar(done, total);

    println!();

    // Beautiful exercise card
    ui::print_exercise_card(
        &exercise.manifest.exercise.id,
        &exercise.manifest.exercise.title,
        exercise.manifest.exercise.track.display_name(),
        exercise.manifest.exercise.description.as_deref(),
        exercise.manifest.exercise.difficulty,
    );

    // README excerpt if exists
    if exercise.readme_path.exists() {
        if let Ok(readme) = std::fs::read_to_string(&exercise.readme_path) {
            let lines: Vec<&str> = readme.lines().take(8).collect();
            if !lines.is_empty() {
                println!();
                println!("  {}", style("Instructions:").white().bold());
                for line in &lines {
                    if !line.trim().is_empty() {
                        println!("  {}", style(line).dim());
                    }
                }
                if readme.lines().count() > 8 {
                    println!(
                        "  {}",
                        style("  ... (see README.md for full instructions)").dim()
                    );
                }
            }
        }
    }

    Ok(())
}

/// Display keyboard shortcut hints with extended options.
pub fn print_key_hints_extended() {
    println!();
    println!(
        "  {} {}  {} {}  {} {}  {} {}  {} {}",
        style("[h]").cyan(),
        style("hint").dim(),
        style("[r]").cyan(),
        style("retry").dim(),
        style("[n]").cyan(),
        style("next").dim(),
        style("[l]").cyan(),
        style("list").dim(),
        style("[q]").cyan(),
        style("quit").dim(),
    );
}

fn handle_hint(runner: &ExerciseRunner, exercise_id: &str) -> Result<()> {
    println!();
    ui::section_header(&format!("{} Hints", icons::LIGHTBULB));
    println!();

    let hints = runner.get_hints(exercise_id)?;

    if hints.is_empty() {
        println!(
            "  {}",
            style("No hints available for this exercise.").dim()
        );
        println!();
        return Ok(());
    }

    for (i, hint) in hints.iter().enumerate() {
        let star_rating: String = (0..=i)
            .map(|_| format!("{}", style(icons::STAR).yellow()))
            .collect();

        println!(
            "  {} {}  {}",
            star_rating,
            style(format!("Hint {}:", i + 1)).yellow().bold(),
            hint
        );
        println!();
    }

    print_key_hints_extended();
    Ok(())
}

fn handle_list(runner: &ExerciseRunner, progress: &UserProgress) -> Result<()> {
    println!();
    ui::section_header(&format!("{} Exercise List", icons::BOOK));
    println!();

    let exercises = runner.discover_exercises()?;
    let completed = progress.completed_exercises();

    let mut current_track = String::new();
    let mut exercise_count = 0;
    let mut completed_count = 0;

    for exercise in &exercises {
        let track_name = exercise.manifest.exercise.track.dir_name();
        let prerequisites_met = exercise.prerequisites_met(&completed);

        if track_name != current_track {
            if !current_track.is_empty() {
                println!();
            }
            println!(
                "  {}  {}",
                style(icons::ARROW_RIGHT).cyan(),
                style(exercise.manifest.exercise.track.display_name())
                    .white()
                    .bold()
            );
            println!();
            current_track = track_name.to_string();
        }

        exercise_count += 1;
        let status = progress.get_status(&exercise.full_id());

        if status == crate::ExerciseStatus::Completed {
            completed_count += 1;
        }

        let status_icon = ui::status_symbol(&status);
        let locked = if !prerequisites_met && status != crate::ExerciseStatus::Completed {
            format!(" {}", style(icons::LOCKED).dim())
        } else {
            String::new()
        };

        println!(
            "     {} {} {}{}",
            status_icon,
            style(&exercise.manifest.exercise.id).white(),
            style(&exercise.manifest.exercise.title).dim(),
            locked
        );
    }

    // Progress summary
    ui::print_progress_bar(completed_count, exercise_count);

    println!();
    print_key_hints_extended();

    Ok(())
}
