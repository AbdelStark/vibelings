//! Model provider abstraction.
//!
//! All model interactions go through a unified provider trait, with an OpenAI-compatible
//! schema internally. Supported backends:
//! - **OpenRouter** (default): Unified API with BYOK support, privacy controls, ZDR
//! - **Anthropic**: Direct Anthropic (Claude) API
//! - **OpenAI**: Direct OpenAI API
//! - **Local**: Any OpenAI-compatible server (Ollama, vLLM, LM Studio, etc.)

mod anthropic;
mod fallback;
mod local;
mod openai;
mod openrouter;
mod request;
mod response;
mod retry;
mod traits;

pub use anthropic::AnthropicProvider;
pub use fallback::FallbackProvider;
pub use local::LocalProvider;
pub use openai::OpenAIProvider;
pub use openrouter::OpenRouterProvider;
pub use request::{
    CompletionRequest, FunctionCall, Message, MessageContent, MessageRole, Tool, ToolChoice,
};
pub use response::{CompletionResponse, FinishReason, ToolCallResult, Usage};
pub use retry::{RetryConfig, RetryingProvider};
pub use traits::ModelProvider;

use crate::config::{ProviderType, UserConfig};
use crate::Result;
use std::sync::Arc;

/// Create a provider instance for a specific provider type.
///
/// This creates a raw provider without retry wrapping.
fn create_provider_for_type(
    provider_type: &ProviderType,
    config: &UserConfig,
) -> Result<Arc<dyn ModelProvider>> {
    match provider_type {
        ProviderType::OpenRouter => {
            let provider = OpenRouterProvider::from_config(config)?;
            Ok(Arc::new(provider))
        }
        ProviderType::OpenAI => {
            let provider = OpenAIProvider::from_config(config)?;
            Ok(Arc::new(provider))
        }
        ProviderType::Anthropic => {
            let provider = AnthropicProvider::from_config(config)?;
            Ok(Arc::new(provider))
        }
        ProviderType::Local => {
            let provider = LocalProvider::from_config(config)?;
            Ok(Arc::new(provider))
        }
    }
}

/// Create a provider instance based on the configuration.
///
/// This creates the raw provider without retry wrapping.
/// Use `create_provider_with_retry` for production use.
pub fn create_provider(config: &UserConfig) -> Result<Arc<dyn ModelProvider>> {
    create_provider_for_type(&config.model.provider, config)
}

/// Create a provider instance with retry logic and fallback support.
///
/// This wraps the provider in:
/// 1. A fallback layer (if fallback_providers is configured)
/// 2. A retry layer that handles rate limiting with exponential backoff
pub fn create_provider_with_retry(config: &UserConfig) -> Result<Arc<dyn ModelProvider>> {
    let retry_config = RetryConfig::default();

    // Check if fallback providers are configured
    if config.model.fallback_providers.is_empty() {
        // No fallback - just wrap primary in retry
        let primary = create_provider(config)?;
        Ok(Arc::new(RetryingProvider::new(primary, retry_config)))
    } else {
        // Build the fallback chain
        let mut providers: Vec<Arc<dyn ModelProvider>> = Vec::new();

        // Primary provider (wrapped in retry)
        let primary = create_provider(config)?;
        providers.push(Arc::new(RetryingProvider::new(
            primary,
            retry_config.clone(),
        )));

        // Fallback providers (each wrapped in retry)
        for fallback_type in &config.model.fallback_providers {
            // Skip if same as primary (no point in retrying same provider)
            if fallback_type == &config.model.provider {
                continue;
            }

            // Try to create the fallback provider
            // If it fails (e.g., missing API key), skip it
            match create_provider_for_type(fallback_type, config) {
                Ok(provider) => {
                    providers.push(Arc::new(RetryingProvider::new(
                        provider,
                        retry_config.clone(),
                    )));
                    tracing::debug!("Added fallback provider: {}", fallback_type);
                }
                Err(e) => {
                    tracing::debug!(
                        "Skipping fallback provider {} (not configured): {}",
                        fallback_type,
                        e
                    );
                }
            }
        }

        // If we only have the primary, return it directly
        if providers.len() == 1 {
            Ok(providers.pop().unwrap())
        } else {
            Ok(Arc::new(FallbackProvider::new(providers)))
        }
    }
}
