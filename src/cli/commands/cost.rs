//! Cost command implementation.

use crate::config::load_progress;
use crate::Result;
use console::style;

/// Run the cost command.
pub async fn run(exercise: Option<&str>) -> Result<()> {
    println!("{}", style("💰 Token Costs").cyan().bold());
    println!();

    let progress = load_progress().unwrap_or_default();

    if progress.exercises.is_empty() {
        println!("{}", style("No exercises attempted yet.").dim());
        return Ok(());
    }

    let mut total_tokens: u64 = 0;
    let mut total_cost: f64 = 0.0;

    // Header
    println!(
        "{:30} {:>12} {:>12}",
        style("Exercise").bold(),
        style("Tokens").bold(),
        style("Cost (USD)").bold()
    );
    println!("{}", "-".repeat(56));

    for (id, data) in &progress.exercises {
        // Filter by exercise if specified
        if let Some(filter) = exercise {
            if id != filter {
                continue;
            }
        }

        println!(
            "{:30} {:>12} {:>12.4}",
            id, data.total_tokens, data.total_cost
        );

        total_tokens += data.total_tokens;
        total_cost += data.total_cost;
    }

    println!("{}", "-".repeat(56));
    println!(
        "{:30} {:>12} {:>12.4}",
        style("Total").bold(),
        style(total_tokens).bold(),
        style(format!("${:.4}", total_cost)).bold()
    );

    Ok(())
}
