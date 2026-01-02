//! Verify command implementation - validate completed exercises.

use crate::cli::ui::{self, icons};
use crate::config::load_progress;
use crate::runner::ExerciseRunner;
use crate::Result;
use console::style;

/// Run the verify command.
pub async fn run(exercise: Option<&str>) -> Result<()> {
    ui::print_command_header(icons::MAGNIFIER, "Verifying Exercises");

    let runner = ExerciseRunner::new()?;
    let progress = load_progress().unwrap_or_default();

    let exercises_to_verify: Vec<String> = if let Some(id) = exercise {
        vec![id.to_string()]
    } else {
        progress.completed_exercises().into_iter().collect()
    };

    if exercises_to_verify.is_empty() {
        println!(
            "  {} {}",
            icons::INFO,
            style("No completed exercises to verify.").dim()
        );
        println!();
        println!(
            "     {}",
            style("Complete some exercises first, then run verify!").dim()
        );
        println!();
        return Ok(());
    }

    let total = exercises_to_verify.len();
    println!(
        "  {} Verifying {} exercise{}...",
        icons::BULLET,
        style(total).cyan().bold(),
        if total > 1 { "s" } else { "" }
    );
    println!();

    let mut passed = 0;
    let mut failed = 0;

    for (i, exercise_id) in exercises_to_verify.iter().enumerate() {
        // Progress indicator
        print!(
            "  [{}/{}] {} ",
            style(i + 1).dim(),
            style(total).dim(),
            exercise_id
        );

        let spinner = ui::create_dots_spinner("");

        match runner.run_exercise(exercise_id, false).await {
            Ok(result) if result.passed => {
                spinner.finish_and_clear();
                println!(
                    "  [{}/{}] {} {}",
                    style(i + 1).dim(),
                    style(total).dim(),
                    exercise_id,
                    style(icons::CHECK).green()
                );
                passed += 1;
            }
            Ok(result) => {
                spinner.finish_and_clear();
                println!(
                    "  [{}/{}] {} {}",
                    style(i + 1).dim(),
                    style(total).dim(),
                    exercise_id,
                    style(icons::CROSS).red()
                );
                if let Some(error) = result.error_message {
                    println!("       {} {}", icons::ARROW_RIGHT, style(error).red().dim());
                }
                failed += 1;
            }
            Err(e) => {
                spinner.finish_and_clear();
                println!(
                    "  [{}/{}] {} {}",
                    style(i + 1).dim(),
                    style(total).dim(),
                    exercise_id,
                    style(icons::CROSS).red()
                );
                println!(
                    "       {} {}",
                    icons::ARROW_RIGHT,
                    style(e.to_string()).red().dim()
                );
                failed += 1;
            }
        }
    }

    // Summary
    println!();
    ui::section_header("Results");
    println!();

    if failed == 0 {
        println!(
            "  {} {}",
            icons::TROPHY,
            style(format!("All {} exercises passed!", passed))
                .green()
                .bold()
        );

        // Full progress bar
        let bar: String = "━".repeat(30);
        println!();
        println!(
            "     [{}] {}/{}",
            style(bar).green(),
            style(passed).green().bold(),
            style(total).white()
        );
    } else {
        println!(
            "  {} {}",
            icons::WARNING,
            style(format!("{} passed, {} failed", passed, failed))
                .yellow()
                .bold()
        );

        // Partial progress bar
        let bar_width = 30;
        let filled = (passed * bar_width) / total;
        let empty = bar_width - filled;

        println!();
        println!(
            "     [{}{}] {}/{}",
            style("━".repeat(filled)).green(),
            style("─".repeat(empty)).red(),
            style(passed).yellow().bold(),
            style(total).white()
        );

        println!();
        println!(
            "     {} Re-run the failing exercises to fix them.",
            icons::INFO
        );
    }

    println!();

    Ok(())
}
