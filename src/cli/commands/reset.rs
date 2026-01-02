//! Reset command implementation - restore exercise to starter state.

use crate::cli::ui::{self, icons};
use crate::runner::ExerciseRunner;
use crate::Result;
use console::style;
use dialoguer::Confirm;

/// Run the reset command.
pub async fn run(exercise: &str, force: bool) -> Result<()> {
    ui::print_command_header(icons::REFRESH, &format!("Reset: {}", exercise));

    let runner = ExerciseRunner::new()?;

    // Get exercise details if possible
    if let Ok(ex) = runner.get_exercise(exercise) {
        println!(
            "  {} {}",
            style("Title:").dim(),
            style(&ex.manifest.exercise.title).white().bold()
        );
        println!(
            "  {} {}",
            style("Track:").dim(),
            style(ex.manifest.exercise.track.display_name()).cyan()
        );
        println!();
    }

    // Warning message
    println!(
        "  {} {}",
        icons::WARNING,
        style("This will reset the exercise to its starter state.").yellow()
    );
    println!(
        "     {}",
        style("All your changes will be lost!").yellow().dim()
    );
    println!();

    if !force {
        let confirmed = Confirm::new()
            .with_prompt(format!("  {} Reset '{}'?", icons::QUESTION, exercise))
            .default(false)
            .interact()
            .unwrap_or(false);

        if !confirmed {
            println!();
            println!("  {} {}", icons::INFO, style("Reset cancelled.").dim());
            println!();
            return Ok(());
        }
    }

    // Perform reset with spinner
    println!();
    let spinner = ui::create_spinner("Resetting exercise...");

    runner.reset_exercise(exercise)?;

    spinner.finish_and_clear();

    // Success message
    println!(
        "  {} {}",
        style(icons::CHECK).green(),
        style(format!("Exercise '{}' has been reset.", exercise))
            .green()
            .bold()
    );
    println!();
    println!(
        "     {} You can now start fresh with the starter files.",
        icons::ARROW_RIGHT
    );
    println!();

    Ok(())
}
