//! Hint command implementation.

use crate::runner::ExerciseRunner;
use crate::Result;
use console::style;

/// Run the hint command.
pub async fn run(exercise: Option<&str>, level: u8) -> Result<()> {
    let runner = ExerciseRunner::new()?;

    let exercise_id = if let Some(id) = exercise {
        id.to_string()
    } else {
        // Get current exercise from progress
        runner.get_current_exercise()?
    };

    println!(
        "{}",
        style(format!("💡 Hint for: {}", exercise_id)).cyan().bold()
    );
    println!();

    let hints = runner.get_hints(&exercise_id)?;

    if hints.is_empty() {
        println!("{}", style("No hints available for this exercise.").dim());
        return Ok(());
    }

    // Show hints up to the requested level
    let level = level.min(hints.len() as u8);

    for (i, hint) in hints.iter().take(level as usize).enumerate() {
        println!(
            "{} {}",
            style(format!("Hint {}:", i + 1)).yellow().bold(),
            hint
        );
        println!();
    }

    if (level as usize) < hints.len() {
        println!(
            "{}",
            style(format!(
                "Use --level {} for more hints ({} remaining)",
                level + 1,
                hints.len() - level as usize
            ))
            .dim()
        );
    }

    Ok(())
}
