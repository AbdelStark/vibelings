//! Initialize command implementation.

use crate::config::load_or_create_config;
use crate::Result;
use console::style;
use std::fs;
use std::path::Path;

/// Run the init command.
pub async fn run(track: Option<&str>) -> Result<()> {
    println!(
        "{}",
        style("🎯 Initializing vibelings workspace...")
            .cyan()
            .bold()
    );

    // Create exercises directory
    let exercises_dir = Path::new("exercises");
    if !exercises_dir.exists() {
        fs::create_dir_all(exercises_dir)?;
        println!("  {} Created exercises/ directory", style("✓").green());
    }

    // Create workspace directory structure
    let tracks = ["fundamentals", "mcp", "workflows", "production"];
    for track_name in &tracks {
        let track_dir = exercises_dir.join(track_name);
        if !track_dir.exists() {
            fs::create_dir_all(&track_dir)?;
            println!("  {} Created exercises/{}/", style("✓").green(), track_name);
        }
    }

    // Create or load config
    let _config = load_or_create_config()?;
    println!("  {} Configuration ready", style("✓").green());

    // Create local config file if it doesn't exist
    let local_config_path = Path::new(".vibelings.toml");
    if !local_config_path.exists() {
        let local_config = r#"# Local vibelings configuration
# This file can override global settings for this workspace

# Uncomment to use a different model for this project
# [model]
# model = "openai/gpt-4o"

# Uncomment to adjust sandbox settings
# [sandbox]
# timeout_seconds = 60
"#;
        fs::write(local_config_path, local_config)?;
        println!("  {} Created .vibelings.toml", style("✓").green());
    }

    // Create README for exercises
    let exercises_readme = exercises_dir.join("README.md");
    if !exercises_readme.exists() {
        let readme_content = r#"# Vibelings Exercises

This directory contains the exercise content for your vibelings learning journey.

## Tracks

1. **fundamentals/** - Agentic Fundamentals
   - Core primitives: structured output, tool calling, error recovery

2. **mcp/** - MCP in Practice
   - Model Context Protocol implementation

3. **workflows/** - Workflow Orchestration
   - Integration with workflow tools

4. **production/** - Production Engineering
   - Reliability, security, and observability at scale

## Getting Started

Run `vibelings` in this directory to start the interactive learning experience.
"#;
        fs::write(&exercises_readme, readme_content)?;
        println!("  {} Created exercises/README.md", style("✓").green());
    }

    println!();
    println!(
        "{}",
        style("✅ Workspace initialized successfully!")
            .green()
            .bold()
    );
    println!();

    if let Some(track_name) = track {
        println!("Starting with track: {}", style(track_name).yellow().bold());
    } else {
        println!("Run {} to start learning!", style("vibelings").cyan());
    }

    println!();
    println!("{}:", style("Quick start").bold());
    println!("  {} - Check your setup", style("vibelings doctor").cyan());
    println!("  {} - List all exercises", style("vibelings list").cyan());
    println!("  {} - Start watch mode", style("vibelings").cyan());

    Ok(())
}
