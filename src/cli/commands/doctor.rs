//! Doctor command implementation - beautiful health checks.

use crate::cli::ui::{self, icons};
use crate::config::{load_or_create_config, ProviderType};
use crate::provider::{create_provider, CompletionRequest, Message, ModelProvider};
use crate::Result;
use console::style;

/// Run the doctor command.
///
/// If `full` is true, performs an actual API connectivity test.
pub async fn run(full: bool) -> Result<()> {
    ui::print_command_header(icons::STETHOSCOPE, "System Health Check");

    let mut all_ok = true;
    let mut checks_passed = 0;
    let total_checks = if full { 5 } else { 4 };

    // Check 1: Configuration
    print_check_start("Configuration");
    match load_or_create_config() {
        Ok(config) => {
            print_check_pass(None);
            checks_passed += 1;

            // Check 2: Provider configuration
            print_check_start(&format!("Provider ({})", config.model.provider));
            let api_key_ok = match config.model.provider {
                ProviderType::OpenRouter => {
                    if std::env::var(&config.openrouter.api_key_env).is_ok() {
                        print_check_pass(None);
                        checks_passed += 1;
                        true
                    } else {
                        print_check_fail(&format!(
                            "Set {} environment variable",
                            config.openrouter.api_key_env
                        ));
                        all_ok = false;
                        false
                    }
                }
                ProviderType::OpenAI => {
                    if std::env::var("OPENAI_API_KEY").is_ok() {
                        print_check_pass(None);
                        checks_passed += 1;
                        true
                    } else {
                        print_check_fail("Set OPENAI_API_KEY environment variable");
                        all_ok = false;
                        false
                    }
                }
                ProviderType::Anthropic => {
                    if std::env::var("ANTHROPIC_API_KEY").is_ok() {
                        print_check_pass(None);
                        checks_passed += 1;
                        true
                    } else {
                        print_check_fail("Set ANTHROPIC_API_KEY environment variable");
                        all_ok = false;
                        false
                    }
                }
                ProviderType::Local => {
                    print_check_pass(None);
                    checks_passed += 1;
                    true
                }
            };

            // Check 3: Model access
            print_check_start("Model access");
            match create_provider(&config) {
                Ok(provider) => {
                    print_check_pass(Some(&config.model.model));
                    checks_passed += 1;

                    // Check 4: API connectivity (if full mode and API key is configured)
                    if full {
                        if api_key_ok {
                            print_check_start("API connectivity");
                            let spinner = ui::create_spinner("Testing connection...");
                            match test_api_connectivity(&*provider, &config.model.model).await {
                                Ok(tokens) => {
                                    spinner.finish_and_clear();
                                    print_check_pass(Some(&format!("{} tokens used", tokens)));
                                    checks_passed += 1;
                                }
                                Err(e) => {
                                    spinner.finish_and_clear();
                                    print_check_fail(&e);
                                    all_ok = false;
                                }
                            }
                        } else {
                            print_check_start("API connectivity");
                            print_check_warn("Skipped (no API key)");
                        }
                    }
                }
                Err(e) => {
                    print_check_fail(&e.to_string());
                    all_ok = false;
                }
            }
        }
        Err(e) => {
            print_check_fail(&e.to_string());
            all_ok = false;
        }
    }

    // Check 4/5: Exercises directory
    print_check_start("Exercises directory");
    if std::path::Path::new("exercises").exists() {
        let count = std::fs::read_dir("exercises")
            .map(|d| d.count())
            .unwrap_or(0);
        print_check_pass(Some(&format!("{} tracks", count)));
        checks_passed += 1;
    } else {
        print_check_fail("Run 'vibelings init' first");
        all_ok = false;
    }

    // Check 5/6: Optional tools
    print_check_start("jq (JSON processor)");
    if which_exists("jq") {
        print_check_pass(None);
    } else {
        print_check_warn("Optional - install for better JSON handling");
    }

    // Summary
    println!();
    ui::section_header("Summary");
    println!();

    if all_ok {
        println!(
            "  {} {}",
            icons::TROPHY,
            style("All checks passed! You're ready to go.")
                .green()
                .bold()
        );

        // Progress bar showing checks
        let bar_width = 20;
        let filled: String = "━".repeat(bar_width);
        println!();
        println!(
            "     [{}] {}/{}",
            style(filled).green(),
            style(checks_passed).green().bold(),
            style(total_checks).white()
        );
    } else {
        println!(
            "  {} {}",
            icons::WARNING,
            style("Some checks failed. Please fix the issues above.")
                .yellow()
                .bold()
        );

        let filled = (checks_passed * 20) / total_checks;
        let empty = 20 - filled;
        println!();
        println!(
            "     [{}{}] {}/{}",
            style("━".repeat(filled)).green(),
            style("─".repeat(empty)).dim(),
            style(checks_passed).yellow().bold(),
            style(total_checks).white()
        );
    }

    if !full {
        println!();
        println!(
            "  {} Run {} for full API connectivity test",
            icons::INFO,
            style("vibelings doctor --full").cyan()
        );
    }

    println!();

    Ok(())
}

fn print_check_start(name: &str) {
    print!("  {} {}... ", icons::BULLET, name);
}

fn print_check_pass(detail: Option<&str>) {
    if let Some(d) = detail {
        println!("{} ({})", style(icons::CHECK).green(), style(d).dim());
    } else {
        println!("{}", style(icons::CHECK).green());
    }
}

fn print_check_fail(reason: &str) {
    println!(
        "{} {}",
        style(icons::CROSS).red(),
        style(reason).red().dim()
    );
}

fn print_check_warn(reason: &str) {
    println!(
        "{} {}",
        style(icons::WARNING).yellow(),
        style(reason).yellow().dim()
    );
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
