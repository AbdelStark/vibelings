//! Model provider abstraction.
//!
//! All model interactions go through a unified provider trait, with an OpenAI-compatible
//! schema internally. Supported backends:
//! - **OpenRouter** (default): Unified API with BYOK support, privacy controls, ZDR
//! - **Anthropic**: Direct Anthropic (Claude) API
//! - **OpenAI**: Direct OpenAI API
//! - **Local**: Any OpenAI-compatible server (Ollama, vLLM, LM Studio, etc.)

mod anthropic;
mod local;
mod openai;
mod openrouter;
mod request;
mod response;
mod retry;
mod traits;

pub use anthropic::AnthropicProvider;
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

/// Create a provider instance based on the configuration.
///
/// This creates the raw provider without retry wrapping.
/// Use `create_provider_with_retry` for production use.
pub fn create_provider(config: &UserConfig) -> Result<Arc<dyn ModelProvider>> {
    match config.model.provider {
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

/// Create a provider instance with retry logic for rate limiting.
///
/// This wraps the provider in a retry layer that handles rate limiting
/// with exponential backoff.
pub fn create_provider_with_retry(config: &UserConfig) -> Result<Arc<dyn ModelProvider>> {
    let inner = create_provider(config)?;
    let retry_config = RetryConfig::default();
    Ok(Arc::new(RetryingProvider::new(inner, retry_config)))
}
