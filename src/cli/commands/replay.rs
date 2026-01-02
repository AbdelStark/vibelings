//! Replay command implementation - trace visualization.

use crate::cli::ui::{self, icons};
use crate::trace::TraceStore;
use crate::Result;
use console::style;

/// Run the replay command.
pub async fn run(run_id: &str) -> Result<()> {
    ui::print_command_header(icons::REFRESH, &format!("Replay: {}", run_id));

    let store = TraceStore::new()?;
    let trace = store.load(run_id)?;

    // Exercise info card
    println!(
        "  {} {}",
        style("Exercise:").dim(),
        style(&trace.exercise_id).white().bold()
    );
    println!(
        "  {} {}",
        style("Timestamp:").dim(),
        style(&trace.timestamp).cyan()
    );
    println!(
        "  {} {:.2}s",
        style("Duration:").dim(),
        trace.duration_secs
    );
    println!(
        "  {} {}",
        style("Result:").dim(),
        if trace.passed {
            style("PASSED").green().bold()
        } else {
            style("FAILED").red().bold()
        }
    );

    // Request messages
    println!();
    ui::section_header("Request Messages");
    println!();

    for (i, message) in trace.messages.iter().enumerate() {
        let role_icon = match message.role.as_str() {
            "system" => icons::GEAR,
            "user" => icons::ARROW_RIGHT,
            "assistant" => icons::LIGHTBULB,
            _ => icons::BULLET,
        };

        let role_color = match message.role.as_str() {
            "system" => style(&message.role).magenta(),
            "user" => style(&message.role).cyan(),
            "assistant" => style(&message.role).green(),
            _ => style(&message.role).white(),
        };

        println!(
            "  {} {} {}",
            style(format!("[{}]", i + 1)).dim(),
            role_icon,
            role_color.bold()
        );

        // Show content (truncated if long)
        let content_lines: Vec<&str> = message.content.lines().take(5).collect();
        for line in &content_lines {
            println!("     {}", style(truncate(line, 70)).dim());
        }
        if message.content.lines().count() > 5 {
            println!("     {}", style("...").dim());
        }
        println!();
    }

    // Response
    println!();
    ui::section_header("Response");
    println!();

    if let Some(ref response) = trace.response {
        let response_lines: Vec<&str> = response.lines().take(10).collect();
        for line in &response_lines {
            println!("  {}", style(line).white());
        }
        if response.lines().count() > 10 {
            println!("  {}", style("...").dim());
        }
    } else {
        println!("  {}", style("(No response recorded)").dim());
    }

    // Tool calls
    if !trace.tool_calls.is_empty() {
        println!();
        ui::section_header("Tool Calls");
        println!();

        for (i, call) in trace.tool_calls.iter().enumerate() {
            println!(
                "  {} {} {}",
                style(format!("{}.", i + 1)).dim(),
                icons::GEAR,
                style(&call.name).yellow().bold()
            );

            // Show arguments if any
            if !call.arguments.is_empty() {
                println!(
                    "     {} {}",
                    style("Args:").dim(),
                    style(truncate(&call.arguments, 60)).dim()
                );
            }

            // Show result
            println!(
                "     {} {}",
                style("Result:").dim(),
                style(truncate(&call.result, 60)).cyan()
            );
            println!();
        }
    }

    // Summary box
    println!();
    let result_text = if trace.passed { "PASSED" } else { "FAILED" };
    let result_style = if trace.passed {
        style(result_text).green().bold()
    } else {
        style(result_text).red().bold()
    };

    println!(
        "  {} Replay complete: {}",
        icons::CHECK,
        result_style
    );
    println!();

    Ok(())
}

fn truncate(s: &str, max_len: usize) -> String {
    let s = s.trim();
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[..max_len.saturating_sub(3)])
    }
}
