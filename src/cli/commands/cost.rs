//! Cost command implementation - token usage and cost tracking.

use crate::cli::commands::json_output::{print_json, CostOutput, CostSummary, ExerciseCost};
use crate::cli::ui::{self, icons};
use crate::config::load_progress;
use crate::Result;
use console::style;

/// Run the cost command.
pub async fn run(exercise: Option<&str>, json_output: bool) -> Result<()> {
    let progress = load_progress().unwrap_or_default();

    if progress.exercises.is_empty() {
        if json_output {
            let output = CostOutput {
                exercises: vec![],
                summary: CostSummary {
                    total_tokens: 0,
                    total_cost_usd: 0.0,
                    avg_tokens: 0,
                    avg_cost_usd: 0.0,
                    exercise_count: 0,
                },
            };
            return print_json(&output);
        }

        ui::print_command_header(icons::DOLLAR, "Token Usage & Costs");
        println!(
            "  {} {}",
            icons::INFO,
            style("No exercises attempted yet.").dim()
        );
        println!();
        println!(
            "     {}",
            style("Complete some exercises to track your usage!").dim()
        );
        println!();
        return Ok(());
    }

    // Collect and filter data
    let mut exercises: Vec<_> = progress
        .exercises
        .iter()
        .filter(|(id, _)| {
            if let Some(filter) = exercise {
                *id == filter
            } else {
                true
            }
        })
        .collect();

    if exercises.is_empty() {
        if json_output {
            let output = CostOutput {
                exercises: vec![],
                summary: CostSummary {
                    total_tokens: 0,
                    total_cost_usd: 0.0,
                    avg_tokens: 0,
                    avg_cost_usd: 0.0,
                    exercise_count: 0,
                },
            };
            return print_json(&output);
        }

        ui::print_command_header(icons::DOLLAR, "Token Usage & Costs");
        println!(
            "  {} {}",
            icons::INFO,
            style(format!("No data for exercise '{}'", exercise.unwrap_or(""))).dim()
        );
        println!();
        return Ok(());
    }

    // Sort by cost (highest first)
    exercises.sort_by(|a, b| {
        b.1.total_cost
            .partial_cmp(&a.1.total_cost)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    // Calculate totals
    let total_tokens: u64 = exercises.iter().map(|(_, d)| d.total_tokens).sum();
    let total_cost: f64 = exercises.iter().map(|(_, d)| d.total_cost).sum();
    let exercise_count = exercises.len();
    let avg_tokens = if exercise_count > 0 {
        total_tokens / exercise_count as u64
    } else {
        0
    };
    let avg_cost = if exercise_count > 0 {
        total_cost / exercise_count as f64
    } else {
        0.0
    };

    if json_output {
        let exercise_costs: Vec<ExerciseCost> = exercises
            .iter()
            .map(|(id, data)| ExerciseCost {
                id: (*id).clone(),
                tokens: data.total_tokens,
                cost_usd: data.total_cost,
            })
            .collect();

        let output = CostOutput {
            exercises: exercise_costs,
            summary: CostSummary {
                total_tokens,
                total_cost_usd: total_cost,
                avg_tokens,
                avg_cost_usd: avg_cost,
                exercise_count,
            },
        };
        return print_json(&output);
    }

    // Human-readable output
    ui::print_command_header(icons::DOLLAR, "Token Usage & Costs");

    // Table header
    println!(
        "  {:28} {:>12} {:>12}",
        style("Exercise").white().bold(),
        style("Tokens").white().bold(),
        style("Cost (USD)").white().bold()
    );
    ui::divider(54);

    // Table rows
    for (id, data) in &exercises {
        // Truncate long exercise IDs
        let id_display = if id.len() > 26 {
            format!("{}...", &id[..23])
        } else {
            (*id).clone()
        };

        // Cost color based on amount
        let cost_style = if data.total_cost > 0.1 {
            style(format!("${:.4}", data.total_cost)).yellow()
        } else if data.total_cost > 0.01 {
            style(format!("${:.4}", data.total_cost)).white()
        } else {
            style(format!("${:.4}", data.total_cost)).dim()
        };

        println!(
            "  {:28} {:>12} {:>12}",
            style(&id_display).dim(),
            style(format_tokens(data.total_tokens)).cyan(),
            cost_style
        );
    }

    // Total row
    ui::divider(54);
    println!(
        "  {:28} {:>12} {:>12}",
        style("Total").white().bold(),
        style(format_tokens(total_tokens)).cyan().bold(),
        style(format!("${:.4}", total_cost)).green().bold()
    );

    // Summary stats
    println!();
    ui::section_header("Summary");
    println!();

    println!(
        "  {} Exercises tracked:   {}",
        icons::BULLET,
        style(exercise_count).cyan().bold()
    );
    println!(
        "  {} Average tokens:      {}",
        icons::BULLET,
        style(format_tokens(avg_tokens)).cyan()
    );
    println!(
        "  {} Average cost:        {}",
        icons::BULLET,
        style(format!("${:.4}", avg_cost)).cyan()
    );

    // Cost visualization
    if total_cost > 0.0 {
        println!();
        println!("  {} Cost breakdown:", icons::BULLET);
        println!();

        // Show top 5 exercises by cost
        for (id, data) in exercises.iter().take(5) {
            let percentage = (data.total_cost / total_cost * 100.0) as usize;
            let bar_width = (percentage * 20) / 100;
            let bar: String = "█".repeat(bar_width);
            let empty: String = "░".repeat(20 - bar_width);

            let id_short = if id.len() > 20 {
                format!("{}...", &id[..17])
            } else {
                (*id).clone()
            };

            println!(
                "     {:20} [{}{}] {:>3}%",
                style(&id_short).dim(),
                style(&bar).cyan(),
                style(&empty).dim(),
                percentage
            );
        }
    }

    println!();

    Ok(())
}

/// Format token count with K/M suffixes for readability
fn format_tokens(tokens: u64) -> String {
    if tokens >= 1_000_000 {
        format!("{:.1}M", tokens as f64 / 1_000_000.0)
    } else if tokens >= 1_000 {
        format!("{:.1}K", tokens as f64 / 1_000.0)
    } else {
        tokens.to_string()
    }
}
