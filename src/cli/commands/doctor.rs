//! Doctor command implementation.

use crate::config::{load_or_create_config, ProviderType};
use crate::provider::{create_provider, CompletionRequest, Message, ModelProvider};
use crate::Result;
use console::style;

/// Run the doctor command.
///
/// If `full` is true, performs an actual API connectivity test.
pub async fn run(full: bool) -> Result<()> {
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
            let api_key_ok = match config.model.provider {
                ProviderType::OpenRouter => {
                    if std::env::var(&config.openrouter.api_key_env).is_ok() {
                        println!("{}", style("✓").green());
                        true
                    } else {
                        println!(
                            "{} (set {} environment variable)",
                            style("✗").red(),
                            config.openrouter.api_key_env
                        );
                        all_ok = false;
                        false
                    }
                }
                ProviderType::OpenAI => {
                    if std::env::var("OPENAI_API_KEY").is_ok() {
                        println!("{}", style("✓").green());
                        true
                    } else {
                        println!(
                            "{} (set OPENAI_API_KEY environment variable)",
                            style("✗").red()
                        );
                        all_ok = false;
                        false
                    }
                }
                ProviderType::Anthropic => {
                    if std::env::var("ANTHROPIC_API_KEY").is_ok() {
                        println!("{}", style("✓").green());
                        true
                    } else {
                        println!(
                            "{} (set ANTHROPIC_API_KEY environment variable)",
                            style("✗").red()
                        );
                        all_ok = false;
                        false
                    }
                }
                ProviderType::Local => {
                    println!("{}", style("✓").green());
                    true
                }
            };

            // Try to create provider
            print!("  Model access... ");
            match create_provider(&config) {
                Ok(provider) => {
                    println!("{} ({})", style("✓").green(), config.model.model);

                    // Perform full API test if requested and API key is configured
                    if full && api_key_ok {
                        print!("  API connectivity... ");
                        match test_api_connectivity(&*provider, &config.model.model).await {
                            Ok(tokens) => {
                                println!("{} ({} tokens used)", style("✓").green(), tokens);
                            }
                            Err(e) => {
                                println!("{} ({})", style("✗").red(), e);
                                all_ok = false;
                            }
                        }
                    } else if full && !api_key_ok {
                        println!(
                            "  API connectivity... {} (skipped, no API key)",
                            style("⚠").yellow()
                        );
                    }
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

    if !full {
        println!();
        println!(
            "{}",
            style("Tip: Run 'vibelings doctor --full' to test API connectivity").dim()
        );
    }

    Ok(())
}

/// Test API connectivity with a minimal request.
async fn test_api_connectivity(
    provider: &dyn ModelProvider,
    model: &str,
) -> std::result::Result<u32, String> {
    let request = CompletionRequest::new(
        model,
        vec![
            Message::system("You are a helpful assistant."),
            Message::user("Reply with exactly: OK"),
        ],
    )
    .with_max_tokens(5)
    .with_temperature(0.0);

    match provider.complete(request).await {
        Ok(response) => {
            let usage = response.usage();
            Ok(usage.total_tokens)
        }
        Err(e) => Err(e.to_string()),
    }
}

fn which_exists(cmd: &str) -> bool {
    std::process::Command::new("which")
        .arg(cmd)
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}
