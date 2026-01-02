//! Hint command implementation - progressive hints for exercises.

use crate::cli::ui::{self, icons};
use crate::runner::ExerciseRunner;
use crate::Result;
use console::style;

/// Run the hint command.
pub async fn run(exercise: Option<&str>, level: u8) -> Result<()> {
    let runner = ExerciseRunner::new()?;

    let exercise_id = if let Some(id) = exercise {
        id.to_string()
    } else {
        runner.get_current_exercise()?
    };

    ui::print_command_header(icons::LIGHTBULB, &format!("Hints for {}", exercise_id));

    let hints = runner.get_hints(&exercise_id)?;

    if hints.is_empty() {
        println!(
            "  {} {}",
            icons::INFO,
            style("No hints available for this exercise.").dim()
        );
        println!();
        println!(
            "  {}",
            style("Try reading the README.md carefully for guidance.").dim()
        );
        println!();
        return Ok(());
    }

    // Show hints up to the requested level
    let level = level.min(hints.len() as u8);
    let total_hints = hints.len();

    // Progress indicator
    println!(
        "  {} Showing hint {} of {}",
        icons::BULLET,
        style(level).cyan().bold(),
        style(total_hints).dim()
    );
    println!();

    for (i, hint) in hints.iter().take(level as usize).enumerate() {
        // Difficulty indicator using stars
        let stars: String = (0..=i)
            .map(|_| format!("{}", style(icons::STAR).yellow()))
            .collect();

        // Hint box
        println!(
            "  {} {} {}",
            stars,
            style(format!("Hint {}:", i + 1)).yellow().bold(),
            hint
        );
        println!();
    }

    // Show remaining hints info
    if (level as usize) < hints.len() {
        let remaining = hints.len() - level as usize;
        println!(
            "  {} {} more hint{} available",
            icons::INFO,
            style(remaining).cyan(),
            if remaining > 1 { "s" } else { "" }
        );
        println!();
        println!(
            "     Use {} for the next hint",
            style(format!("vibelings hint --level {}", level + 1)).cyan()
        );
        println!();

        // Progress bar showing hint progression
        let bar_width = 20;
        let filled = (level as usize * bar_width) / total_hints;
        let empty = bar_width - filled;

        println!(
            "     [{}{}] {}/{}",
            style("━".repeat(filled)).yellow(),
            style("─".repeat(empty)).dim(),
            level,
            total_hints
        );
    } else {
        println!(
            "  {} {}",
            icons::STAR,
            style("All hints revealed!").yellow().bold()
        );
        println!();
        println!(
            "     {}",
            style("If you're still stuck, try re-reading the exercise requirements.").dim()
        );
    }

    println!();

    Ok(())
}
