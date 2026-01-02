//! Run command implementation - execute a single exercise.

use crate::cli::ui::{self, icons};
use crate::runner::ExerciseRunner;
use crate::Result;
use console::style;

/// Run the run command.
pub async fn run(exercise: &str, verbose: bool) -> Result<()> {
    // Show exercise header
    ui::print_command_header(icons::TARGET, &format!("Exercise: {}", exercise));

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

    // Show spinner while running
    let spinner = ui::create_spinner("Running exercise...");

    let result = runner.run_exercise(exercise, verbose).await?;

    spinner.finish_and_clear();

    // Display results
    if result.passed {
        ui::celebrate_pass();
        println!();

        // Stats in a nice format
        println!(
            "  {}  {} {:.1}s",
            icons::CLOCK,
            style("Duration:").dim(),
            result.duration_secs
        );
        println!(
            "  {}  {} ${:.4}",
            icons::DOLLAR,
            style("Cost:").dim(),
            result.cost_usd
        );
        println!(
            "  {}  {} {}",
            icons::GEAR,
            style("Tool calls:").dim(),
            result.tool_calls
        );
        println!(
            "  {}  {} {} in / {} out",
            icons::ARROW_RIGHT,
            style("Tokens:").dim(),
            result.tokens_in,
            result.tokens_out
        );
    } else {
        println!();
        println!(
            "  {} {}",
            style(icons::CROSS).red(),
            style("FAILED").red().bold()
        );
        println!();

        // Stats
        println!(
            "  {}  {} {:.1}s",
            icons::CLOCK,
            style("Duration:").dim(),
            result.duration_secs
        );
        println!(
            "  {}  {} ${:.4}",
            icons::DOLLAR,
            style("Cost:").dim(),
            result.cost_usd
        );

        if let Some(ref error) = result.error_message {
            println!();
            println!("  {} {}", style("Error:").red().bold(), error);
        }
    }

    // Grading details if available
    if let Some(ref grading) = result.grading_details {
        println!();
        ui::section_header("Grading Details");
        println!();
        for line in grading.lines() {
            println!("  {}", style(line).dim());
        }
    }

    println!();

    Ok(())
}
