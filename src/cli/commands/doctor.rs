//! Doctor command implementation.

use crate::config::{load_or_create_config, ProviderType};
use crate::provider::create_provider;
use crate::Result;
use console::style;

/// Run the doctor command.
pub async fn run() -> Result<()> {
    println!("{}", style("🩺 Vibelings Doctor").cyan().bold());
    println!();

    let mut all_ok = true;

    // Check configuration
    print!("  Configuration... ");
    match load_or_create_config() {
        Ok(config) => {
            println!("{}", style("✓").green());

            // Check provider configuration
            print!("  Provider ({})... ", config.model.provider);
            match config.model.provider {
                ProviderType::OpenRouter => {
                    if std::env::var(&config.openrouter.api_key_env).is_ok() {
                        println!("{}", style("✓").green());
                    } else {
                        println!(
                            "{} (set {} environment variable)",
                            style("✗").red(),
                            config.openrouter.api_key_env
                        );
                        all_ok = false;
                    }
                }
                ProviderType::OpenAI => {
                    if std::env::var("OPENAI_API_KEY").is_ok() {
                        println!("{}", style("✓").green());
                    } else {
                        println!(
                            "{} (set OPENAI_API_KEY environment variable)",
                            style("✗").red()
                        );
                        all_ok = false;
                    }
                }
                ProviderType::Anthropic => {
                    if std::env::var("ANTHROPIC_API_KEY").is_ok() {
                        println!("{}", style("✓").green());
                    } else {
                        println!(
                            "{} (set ANTHROPIC_API_KEY environment variable)",
                            style("✗").red()
                        );
                        all_ok = false;
                    }
                }
                ProviderType::Local => {
                    println!("{}", style("✓").green());
                }
            }

            // Try to create provider
            print!("  Model access... ");
            match create_provider(&config) {
                Ok(_provider) => {
                    println!("{} ({})", style("✓").green(), config.model.model);

                    // TODO: Actually test the connection with a simple request
                }
                Err(e) => {
                    println!("{} ({})", style("✗").red(), e);
                    all_ok = false;
                }
            }
        }
        Err(e) => {
            println!("{} ({})", style("✗").red(), e);
            all_ok = false;
        }
    }

    // Check exercises directory
    print!("  Exercises directory... ");
    if std::path::Path::new("exercises").exists() {
        let count = std::fs::read_dir("exercises")
            .map(|d| d.count())
            .unwrap_or(0);
        println!("{} ({} tracks)", style("✓").green(), count);
    } else {
        println!("{} (run 'vibelings init' first)", style("✗").red());
        all_ok = false;
    }

    // Check required tools
    print!("  jq (JSON processor)... ");
    if which_exists("jq") {
        println!("{}", style("✓").green());
    } else {
        println!(
            "{} (optional, install for better JSON handling)",
            style("⚠").yellow()
        );
    }

    println!();
    if all_ok {
        println!(
            "{}",
            style("✅ All checks passed! You're ready to go.")
                .green()
                .bold()
        );
    } else {
        println!(
            "{}",
            style("⚠️  Some checks failed. Please fix the issues above.")
                .yellow()
                .bold()
        );
    }

    Ok(())
}

fn which_exists(cmd: &str) -> bool {
    std::process::Command::new("which")
        .arg(cmd)
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}
