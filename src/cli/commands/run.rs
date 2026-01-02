//! Run command implementation.

use crate::runner::ExerciseRunner;
use crate::Result;
use console::style;

/// Run the run command.
pub async fn run(exercise: &str, verbose: bool) -> Result<()> {
    println!(
        "{}",
        style(format!("━━━ Exercise: {} ━━━", exercise))
            .cyan()
            .bold()
    );
    println!();

    let runner = ExerciseRunner::new()?;
    let result = runner.run_exercise(exercise, verbose).await?;

    // Display results
    if result.passed {
        println!(
            "{}",
            style(format!(
                "✅ PASSED ({:.1}s, ${:.4})",
                result.duration_secs, result.cost_usd
            ))
            .green()
            .bold()
        );
    } else {
        println!(
            "{}",
            style(format!(
                "❌ FAILED ({:.1}s, ${:.4})",
                result.duration_secs, result.cost_usd
            ))
            .red()
            .bold()
        );
        if let Some(ref error) = result.error_message {
            println!();
            println!("{}: {}", style("Error").red(), error);
        }
    }

    println!();
    println!(
        "Tool calls: {} | Tokens: {} in / {} out",
        result.tool_calls, result.tokens_in, result.tokens_out
    );

    if let Some(ref grading) = result.grading_details {
        println!();
        println!("{}", grading);
    }

    Ok(())
}
