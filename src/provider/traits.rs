//! Provider traits.

use super::{CompletionRequest, CompletionResponse};
use crate::Result;
use async_trait::async_trait;

/// Trait for model providers.
///
/// All model interactions go through this unified interface.
#[async_trait]
pub trait ModelProvider: Send + Sync {
    /// Send a completion request and get a response.
    async fn complete(&self, request: CompletionRequest) -> Result<CompletionResponse>;

    /// Get the provider name.
    fn name(&self) -> &str;

    /// Check if the provider supports tool calling.
    fn supports_tools(&self) -> bool {
        true
    }

    /// Check if the provider supports JSON mode.
    fn supports_json_mode(&self) -> bool {
        true
    }

    /// Get the default model for this provider.
    fn default_model(&self) -> &str;
}
