//! Verify command implementation.

use crate::config::load_progress;
use crate::runner::ExerciseRunner;
use crate::Result;
use console::style;

/// Run the verify command.
pub async fn run(exercise: Option<&str>) -> Result<()> {
    println!("{}", style("🔍 Verifying exercises...").cyan().bold());
    println!();

    let runner = ExerciseRunner::new()?;
    let progress = load_progress().unwrap_or_default();

    let exercises_to_verify: Vec<String> = if let Some(id) = exercise {
        vec![id.to_string()]
    } else {
        // Verify all completed exercises
        progress.completed_exercises().into_iter().collect()
    };

    if exercises_to_verify.is_empty() {
        println!("{}", style("No completed exercises to verify.").dim());
        println!("Complete some exercises first!");
        return Ok(());
    }

    let mut passed = 0;
    let mut failed = 0;

    for exercise_id in &exercises_to_verify {
        print!("  {} ", exercise_id);

        match runner.run_exercise(exercise_id, false).await {
            Ok(result) if result.passed => {
                println!("{}", style("✓").green());
                passed += 1;
            }
            Ok(result) => {
                println!("{}", style("✗").red());
                if let Some(error) = result.error_message {
                    println!("    {}", style(error).red().dim());
                }
                failed += 1;
            }
            Err(e) => {
                println!("{}", style("✗").red());
                println!("    {}", style(e.to_string()).red().dim());
                failed += 1;
            }
        }
    }

    println!();
    if failed == 0 {
        println!(
            "{}",
            style(format!("✅ All {} exercises passed!", passed))
                .green()
                .bold()
        );
    } else {
        println!(
            "{}",
            style(format!("⚠️  {} passed, {} failed", passed, failed))
                .yellow()
                .bold()
        );
    }

    Ok(())
}
