//! Verify command implementation - validate completed exercises.

use crate::cli::commands::json_output::{print_json, VerifyOutput, VerifyResult, VerifySummary};
use crate::cli::ui::{self, icons};
use crate::config::load_progress;
use crate::runner::ExerciseRunner;
use crate::Result;
use console::style;

/// Run the verify command.
pub async fn run(exercise: Option<&str>, json_output: bool) -> Result<()> {
    let runner = ExerciseRunner::new()?;
    let progress = load_progress().unwrap_or_default();

    let exercises_to_verify: Vec<String> = if let Some(id) = exercise {
        vec![id.to_string()]
    } else {
        progress.completed_exercises().into_iter().collect()
    };

    if exercises_to_verify.is_empty() {
        if json_output {
            let output = VerifyOutput {
                success: true,
                results: vec![],
                summary: VerifySummary {
                    total: 0,
                    passed: 0,
                    failed: 0,
                },
            };
            return print_json(&output);
        }

        ui::print_command_header(icons::MAGNIFIER, "Verifying Exercises");
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
    let mut results: Vec<VerifyResult> = Vec::new();
    let mut passed_count = 0;
    let mut failed_count = 0;

    if !json_output {
        ui::print_command_header(icons::MAGNIFIER, "Verifying Exercises");
        println!(
            "  {} Verifying {} exercise{}...",
            icons::BULLET,
            style(total).cyan().bold(),
            if total > 1 { "s" } else { "" }
        );
        println!();
    }

    for (i, exercise_id) in exercises_to_verify.iter().enumerate() {
        if !json_output {
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
                    passed_count += 1;
                    results.push(VerifyResult {
                        id: exercise_id.clone(),
                        passed: true,
                        error: None,
                    });
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
                    if let Some(ref error) = result.error_message {
                        println!("       {} {}", icons::ARROW_RIGHT, style(error).red().dim());
                    }
                    failed_count += 1;
                    results.push(VerifyResult {
                        id: exercise_id.clone(),
                        passed: false,
                        error: result.error_message,
                    });
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
                    failed_count += 1;
                    results.push(VerifyResult {
                        id: exercise_id.clone(),
                        passed: false,
                        error: Some(e.to_string()),
                    });
                }
            }
        } else {
            // JSON mode - no spinners
            match runner.run_exercise(exercise_id, false).await {
                Ok(result) if result.passed => {
                    passed_count += 1;
                    results.push(VerifyResult {
                        id: exercise_id.clone(),
                        passed: true,
                        error: None,
                    });
                }
                Ok(result) => {
                    failed_count += 1;
                    results.push(VerifyResult {
                        id: exercise_id.clone(),
                        passed: false,
                        error: result.error_message,
                    });
                }
                Err(e) => {
                    failed_count += 1;
                    results.push(VerifyResult {
                        id: exercise_id.clone(),
                        passed: false,
                        error: Some(e.to_string()),
                    });
                }
            }
        }
    }

    if json_output {
        let output = VerifyOutput {
            success: failed_count == 0,
            results,
            summary: VerifySummary {
                total,
                passed: passed_count,
                failed: failed_count,
            },
        };
        return print_json(&output);
    }

    // Summary (human output)
    println!();
    ui::section_header("Results");
    println!();

    if failed_count == 0 {
        println!(
            "  {}{}{}  {}",
            icons::SPARKLE,
            icons::TROPHY,
            icons::SPARKLE,
            style(format!("All {} exercises passed!", passed_count))
                .green()
                .bold()
        );
        println!(
            "  {}",
            style("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━").green().dim()
        );

        // Full progress bar
        let bar: String = "█".repeat(30);
        println!();
        println!(
            "     [{}] {}/{}",
            style(bar).green(),
            style(passed_count).green().bold(),
            style(total).dim()
        );
        println!();
        println!(
            "  {} {}",
            icons::CHECKERED_FLAG,
            style("Your exercises are verified and ready!").cyan()
        );
    } else {
        println!(
            "  {} {}",
            icons::WARNING,
            style(format!("{} passed, {} failed", passed_count, failed_count))
                .yellow()
                .bold()
        );

        // Partial progress bar
        let bar_width = 30;
        let filled = (passed_count * bar_width) / total;
        let empty = bar_width - filled;

        println!();
        println!(
            "     [{}{}] {}/{}",
            style("█".repeat(filled)).green(),
            style("░".repeat(empty)).red(),
            style(passed_count).yellow().bold(),
            style(total).dim()
        );

        println!();
        println!(
            "  {} {}",
            icons::LIGHTBULB,
            style("Re-run the failing exercises to fix them").dim()
        );
        println!(
            "     {} {}",
            style("Run").dim(),
            style("vibelings run <exercise>").cyan()
        );
    }

    println!();

    Ok(())
}
