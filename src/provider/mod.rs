//! Model provider abstraction.
//!
//! All model interactions go through a unified provider trait, with an OpenAI-compatible
//! schema internally. Supported backends:
//! - **OpenRouter** (default): Unified API with BYOK support, privacy controls, ZDR
//! - **Anthropic**: Direct Anthropic (Claude) API
//! - **Direct providers**: OpenAI via their native APIs
//! - **Local endpoints**: Any OpenAI-compatible server (Ollama, vLLM, etc.)

mod anthropic;
mod openrouter;
mod request;
mod response;
mod traits;

pub use anthropic::AnthropicProvider;
pub use openrouter::OpenRouterProvider;
pub use request::{
    CompletionRequest, FunctionCall, Message, MessageContent, MessageRole, Tool, ToolChoice,
};
pub use response::{CompletionResponse, FinishReason, ToolCallResult, Usage};
pub use traits::ModelProvider;

use crate::config::{ProviderType, UserConfig};
use crate::Result;
use std::sync::Arc;

/// Create a provider instance based on the configuration.
pub fn create_provider(config: &UserConfig) -> Result<Arc<dyn ModelProvider>> {
    match config.model.provider {
        ProviderType::OpenRouter => {
            let provider = OpenRouterProvider::from_config(config)?;
            Ok(Arc::new(provider))
        }
        ProviderType::OpenAI => {
            // TODO: Implement OpenAI provider
            // For now, fall back to OpenRouter
            let provider = OpenRouterProvider::from_config(config)?;
            Ok(Arc::new(provider))
        }
        ProviderType::Anthropic => {
            let provider = AnthropicProvider::from_config(config)?;
            Ok(Arc::new(provider))
        }
        ProviderType::Local => {
            // TODO: Implement local provider
            // For now, fall back to OpenRouter
            let provider = OpenRouterProvider::from_config(config)?;
            Ok(Arc::new(provider))
        }
    }
}
