//! Reset command implementation.

use crate::runner::ExerciseRunner;
use crate::Result;
use console::style;
use dialoguer::Confirm;

/// Run the reset command.
pub async fn run(exercise: &str, force: bool) -> Result<()> {
    println!("{}", style(format!("🔄 Reset: {}", exercise)).cyan().bold());
    println!();

    if !force {
        let confirmed = Confirm::new()
            .with_prompt(format!(
                "This will reset '{}' to its starter state. Are you sure?",
                exercise
            ))
            .default(false)
            .interact()
            .unwrap_or(false);

        if !confirmed {
            println!("{}", style("Reset cancelled.").dim());
            return Ok(());
        }
    }

    let runner = ExerciseRunner::new()?;
    runner.reset_exercise(exercise)?;

    println!(
        "{}",
        style(format!("✅ Exercise '{}' has been reset.", exercise))
            .green()
            .bold()
    );

    Ok(())
}
