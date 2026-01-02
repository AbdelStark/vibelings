//! Doctor command implementation - beautiful health checks.

use crate::cli::commands::json_output::{print_json, DoctorOutput, HealthCheck};
use crate::cli::ui::{self, icons};
use crate::config::{load_or_create_config, ProviderType};
use crate::provider::{create_provider, CompletionRequest, Message, ModelProvider};
use crate::Result;
use console::style;

/// Run the doctor command.
///
/// If `full` is true, performs an actual API connectivity test.
pub async fn run(full: bool, json_output: bool) -> Result<()> {
    let mut checks: Vec<HealthCheck> = Vec::new();
    let mut all_ok = true;

    // Check 1: Configuration
    let config_result = load_or_create_config();
    match &config_result {
        Ok(_) => {
            checks.push(HealthCheck {
                name: "Configuration".to_string(),
                passed: true,
                detail: None,
                warning: None,
            });
        }
        Err(e) => {
            checks.push(HealthCheck {
                name: "Configuration".to_string(),
                passed: false,
                detail: Some(e.to_string()),
                warning: None,
            });
            all_ok = false;
        }
    }

    if let Ok(config) = &config_result {
        // Check 2: Provider configuration
        let provider_name = format!("Provider ({})", config.model.provider);
        let api_key_ok = match config.model.provider {
            ProviderType::OpenRouter => {
                if std::env::var(&config.openrouter.api_key_env).is_ok() {
                    checks.push(HealthCheck {
                        name: provider_name,
                        passed: true,
                        detail: None,
                        warning: None,
                    });
                    true
                } else {
                    checks.push(HealthCheck {
                        name: provider_name,
                        passed: false,
                        detail: Some(format!(
                            "Set {} environment variable",
                            config.openrouter.api_key_env
                        )),
                        warning: None,
                    });
                    all_ok = false;
                    false
                }
            }
            ProviderType::OpenAI => {
                if std::env::var("OPENAI_API_KEY").is_ok() {
                    checks.push(HealthCheck {
                        name: provider_name,
                        passed: true,
                        detail: None,
                        warning: None,
                    });
                    true
                } else {
                    checks.push(HealthCheck {
                        name: provider_name,
                        passed: false,
                        detail: Some("Set OPENAI_API_KEY environment variable".to_string()),
                        warning: None,
                    });
                    all_ok = false;
                    false
                }
            }
            ProviderType::Anthropic => {
                if std::env::var("ANTHROPIC_API_KEY").is_ok() {
                    checks.push(HealthCheck {
                        name: provider_name,
                        passed: true,
                        detail: None,
                        warning: None,
                    });
                    true
                } else {
                    checks.push(HealthCheck {
                        name: provider_name,
                        passed: false,
                        detail: Some("Set ANTHROPIC_API_KEY environment variable".to_string()),
                        warning: None,
                    });
                    all_ok = false;
                    false
                }
            }
            ProviderType::Local => {
                checks.push(HealthCheck {
                    name: provider_name,
                    passed: true,
                    detail: None,
                    warning: None,
                });
                true
            }
        };

        // Check 3: Model access
        match create_provider(config) {
            Ok(provider) => {
                checks.push(HealthCheck {
                    name: "Model access".to_string(),
                    passed: true,
                    detail: Some(config.model.model.clone()),
                    warning: None,
                });

                // Check 4: API connectivity (if full mode and API key is configured)
                if full {
                    if api_key_ok {
                        if !json_output {
                            // For human output, show spinner
                            let spinner = ui::create_spinner("Testing connection...");
                            match test_api_connectivity(&*provider, &config.model.model).await {
                                Ok(tokens) => {
                                    spinner.finish_and_clear();
                                    checks.push(HealthCheck {
                                        name: "API connectivity".to_string(),
                                        passed: true,
                                        detail: Some(format!("{} tokens used", tokens)),
                                        warning: None,
                                    });
                                }
                                Err(e) => {
                                    spinner.finish_and_clear();
                                    checks.push(HealthCheck {
                                        name: "API connectivity".to_string(),
                                        passed: false,
                                        detail: Some(e),
                                        warning: None,
                                    });
                                    all_ok = false;
                                }
                            }
                        } else {
                            // For JSON output, no spinner
                            match test_api_connectivity(&*provider, &config.model.model).await {
                                Ok(tokens) => {
                                    checks.push(HealthCheck {
                                        name: "API connectivity".to_string(),
                                        passed: true,
                                        detail: Some(format!("{} tokens used", tokens)),
                                        warning: None,
                                    });
                                }
                                Err(e) => {
                                    checks.push(HealthCheck {
                                        name: "API connectivity".to_string(),
                                        passed: false,
                                        detail: Some(e),
                                        warning: None,
                                    });
                                    all_ok = false;
                                }
                            }
                        }
                    } else {
                        checks.push(HealthCheck {
                            name: "API connectivity".to_string(),
                            passed: true,
                            detail: None,
                            warning: Some("Skipped (no API key)".to_string()),
                        });
                    }
                }
            }
            Err(e) => {
                checks.push(HealthCheck {
                    name: "Model access".to_string(),
                    passed: false,
                    detail: Some(e.to_string()),
                    warning: None,
                });
                all_ok = false;
            }
        }
    }

    // Check exercises directory
    if std::path::Path::new("exercises").exists() {
        let count = std::fs::read_dir("exercises")
            .map(|d| d.count())
            .unwrap_or(0);
        checks.push(HealthCheck {
            name: "Exercises directory".to_string(),
            passed: true,
            detail: Some(format!("{} tracks", count)),
            warning: None,
        });
    } else {
        checks.push(HealthCheck {
            name: "Exercises directory".to_string(),
            passed: false,
            detail: Some("Run 'vibelings init' first".to_string()),
            warning: None,
        });
        all_ok = false;
    }

    // Check optional tools
    let jq_exists = which_exists("jq");
    checks.push(HealthCheck {
        name: "jq (JSON processor)".to_string(),
        passed: jq_exists,
        detail: None,
        warning: if jq_exists {
            None
        } else {
            Some("Optional - install for better JSON handling".to_string())
        },
    });

    // Calculate summary
    let passed_count = checks.iter().filter(|c| c.passed).count();
    let total_count = checks.len();

    if json_output {
        let output = DoctorOutput {
            healthy: all_ok,
            checks,
            passed: passed_count,
            total: total_count,
        };
        return print_json(&output);
    }

    // Human-readable output
    ui::print_command_header(icons::STETHOSCOPE, "System Health Check");

    for check in &checks {
        print!("  {} {}... ", icons::BULLET, check.name);
        if check.passed {
            if let Some(ref detail) = check.detail {
                println!("{} ({})", style(icons::CHECK).green(), style(detail).dim());
            } else if let Some(ref warning) = check.warning {
                println!(
                    "{} {}",
                    style(icons::WARNING).yellow(),
                    style(warning).yellow().dim()
                );
            } else {
                println!("{}", style(icons::CHECK).green());
            }
        } else {
            let reason = check.detail.as_deref().unwrap_or("Unknown error");
            println!(
                "{} {}",
                style(icons::CROSS).red(),
                style(reason).red().dim()
            );
        }
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
            style(passed_count).green().bold(),
            style(total_count).white()
        );
    } else {
        println!(
            "  {} {}",
            icons::WARNING,
            style("Some checks failed. Please fix the issues above.")
                .yellow()
                .bold()
        );

        let filled = (passed_count * 20) / total_count;
        let empty = 20 - filled;
        println!();
        println!(
            "     [{}{}] {}/{}",
            style("━".repeat(filled)).green(),
            style("─".repeat(empty)).dim(),
            style(passed_count).yellow().bold(),
            style(total_count).white()
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
