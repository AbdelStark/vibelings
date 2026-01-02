//! Replay command implementation.

use crate::trace::TraceStore;
use crate::Result;
use console::style;

/// Run the replay command.
pub async fn run(run_id: &str) -> Result<()> {
    println!(
        "{}",
        style(format!("🔄 Replaying run: {}", run_id)).cyan().bold()
    );
    println!();

    let store = TraceStore::new()?;
    let trace = store.load(run_id)?;

    println!("{}: {}", style("Exercise").bold(), trace.exercise_id);
    println!("{}: {}", style("Timestamp").bold(), trace.timestamp);
    println!("{}: {:.2}s", style("Duration").bold(), trace.duration_secs);
    println!();

    println!("{}", style("═══ Request ═══").bold());
    for message in &trace.messages {
        println!(
            "  [{}] {}",
            style(&message.role).cyan(),
            truncate(&message.content, 100)
        );
    }

    println!();
    println!("{}", style("═══ Response ═══").bold());
    if let Some(ref response) = trace.response {
        println!("  {}", response);
    }

    if !trace.tool_calls.is_empty() {
        println!();
        println!("{}", style("═══ Tool Calls ═══").bold());
        for (i, call) in trace.tool_calls.iter().enumerate() {
            println!(
                "  {}. {} -> {}",
                i + 1,
                style(&call.name).yellow(),
                truncate(&call.result, 50)
            );
        }
    }

    println!();
    println!(
        "{}: {}",
        style("Result").bold(),
        if trace.passed {
            style("PASSED").green()
        } else {
            style("FAILED").red()
        }
    );

    Ok(())
}

fn truncate(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[..max_len])
    }
}
