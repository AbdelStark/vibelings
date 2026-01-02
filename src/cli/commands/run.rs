//! Run command implementation - execute a single exercise.

use crate::cli::commands::json_output::{print_json, RunOutput};
use crate::cli::ui::{self, icons};
use crate::exercise::Exercise;
use crate::runner::ExerciseRunner;
use crate::Result;
use console::style;

/// Run the run command.
pub async fn run(exercise: &str, verbose: bool, dry_run: bool, json_output: bool) -> Result<()> {
    let runner = ExerciseRunner::new()?;

    // Get exercise details
    let ex = runner.get_exercise(exercise)?;

    // Handle dry-run mode (not supported with JSON output)
    if dry_run {
        if json_output {
            eprintln!("Warning: --json is not supported with --dry-run");
        }
        return run_dry_run(&ex);
    }

    if !json_output {
        // Show exercise header for human output
        ui::print_command_header(icons::TARGET, &format!("Exercise: {}", exercise));
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

        // Show spinner while running
        let spinner = ui::create_spinner("Running exercise...");
        let result = runner.run_exercise(exercise, verbose).await?;
        spinner.finish_and_clear();

        // Display results
        display_human_result(&result);
        Ok(())
    } else {
        // JSON output - no spinner, just run and output
        let result = runner.run_exercise(exercise, verbose).await?;
        let output = RunOutput {
            exercise: exercise.to_string(),
            result,
        };
        print_json(&output)
    }
}

/// Display human-readable result output.
fn display_human_result(result: &crate::runner::RunResult) {
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
}

/// Run in dry-run mode: show exercise details without executing.
fn run_dry_run(exercise: &Exercise) -> Result<()> {
    // Show exercise header
    ui::print_command_header(icons::TARGET, &format!("Exercise: {}", exercise.full_id()));
    println!(
        "  {} {}",
        style("Title:").dim(),
        style(&exercise.manifest.exercise.title).white().bold()
    );
    println!(
        "  {} {}",
        style("Track:").dim(),
        style(exercise.manifest.exercise.track.display_name()).cyan()
    );
    println!();

    println!(
        "  {} {}",
        style(icons::INFO).cyan(),
        style("DRY RUN - No API calls will be made").cyan().bold()
    );
    println!();

    // Show requirements
    ui::section_header("Requirements");
    println!();
    println!(
        "  {} JSON mode: {}",
        icons::ARROW_RIGHT,
        if exercise.manifest.requirements.json_mode {
            style("required").green()
        } else {
            style("not required").dim()
        }
    );
    println!(
        "  {} Tool calling: {}",
        icons::ARROW_RIGHT,
        if exercise.manifest.requirements.tool_calling {
            style("required").green()
        } else {
            style("not required").dim()
        }
    );
    println!(
        "  {} Min context: {} tokens",
        icons::ARROW_RIGHT,
        exercise.manifest.requirements.min_context_window
    );
    println!();

    // Show run configuration
    ui::section_header("Run Configuration");
    println!();
    println!(
        "  {} Max tool calls: {}",
        icons::ARROW_RIGHT,
        exercise.manifest.run.max_tool_calls
    );
    println!(
        "  {} Timeout: {}s",
        icons::ARROW_RIGHT,
        exercise.manifest.run.timeout_seconds
    );
    println!(
        "  {} Runs required: {}",
        icons::ARROW_RIGHT,
        exercise.manifest.run.runs
    );
    if let Some(required) = exercise.manifest.run.required_passes {
        println!("  {} Required passes: {}", icons::ARROW_RIGHT, required);
    }
    println!();

    // Show grader configuration
    ui::section_header("Grader");
    println!();
    println!(
        "  {} Type: {}",
        icons::ARROW_RIGHT,
        style(&exercise.manifest.grader.grader_type).yellow()
    );
    if let Some(ref schema) = exercise.manifest.grader.schema_path {
        println!("  {} Schema: {}", icons::ARROW_RIGHT, schema);

        // Try to show schema preview
        let schema_path = exercise.grader_path.join(schema);
        if let Ok(content) = std::fs::read_to_string(&schema_path) {
            if let Ok(schema_json) = serde_json::from_str::<serde_json::Value>(&content) {
                println!();
                ui::section_header("Expected Schema");
                println!();
                if let Ok(pretty) = serde_json::to_string_pretty(&schema_json) {
                    for line in pretty.lines().take(30) {
                        println!("  {}", style(line).dim());
                    }
                    let line_count = pretty.lines().count();
                    if line_count > 30 {
                        println!(
                            "  {} ... ({} more lines)",
                            style("...").dim(),
                            line_count - 30
                        );
                    }
                }
            }
        }
    }
    println!();

    // Show README preview
    if exercise.readme_path.exists() {
        if let Ok(content) = std::fs::read_to_string(&exercise.readme_path) {
            ui::section_header("Exercise Instructions (Preview)");
            println!();
            for line in content.lines().take(20) {
                println!("  {}", style(line).dim());
            }
            let line_count = content.lines().count();
            if line_count > 20 {
                println!(
                    "  {} ... ({} more lines)",
                    style("...").dim(),
                    line_count - 20
                );
            }
            println!();
        }
    }

    println!(
        "  {} Run without --dry-run to execute the exercise",
        style(icons::INFO).cyan()
    );
    println!();

    Ok(())
}
