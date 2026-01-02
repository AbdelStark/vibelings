//! Fallback provider chain implementation.
//!
//! This module provides a provider wrapper that tries multiple providers
//! in sequence, falling back to the next one if the current one fails.

use super::traits::ModelProvider;
use super::{CompletionRequest, CompletionResponse};
use crate::error::ProviderError;
use crate::{Error, Result};
use async_trait::async_trait;
use std::sync::Arc;

/// A provider that chains multiple providers together.
///
/// If the primary provider fails with a retryable error, the fallback
/// provider(s) will be tried in order. Non-retryable errors (like
/// bad requests) are returned immediately.
pub struct FallbackProvider {
    /// The list of providers to try, in order
    providers: Vec<Arc<dyn ModelProvider>>,
}

impl FallbackProvider {
    /// Create a new fallback provider from a list of providers.
    ///
    /// The providers are tried in order: first provider is primary,
    /// subsequent providers are fallbacks.
    pub fn new(providers: Vec<Arc<dyn ModelProvider>>) -> Self {
        assert!(!providers.is_empty(), "At least one provider is required");
        Self { providers }
    }

    /// Check if an error should trigger a fallback to the next provider.
    ///
    /// We fall back on:
    /// - Rate limiting (429)
    /// - Server errors (5xx)
    /// - Connection/timeout errors
    /// - Authentication errors (the next provider might have valid credentials)
    fn should_fallback(error: &Error) -> bool {
        match error {
            Error::Provider(ProviderError::RateLimited(_)) => true,
            Error::Provider(ProviderError::HttpError(msg)) => {
                // Connection errors, timeouts
                msg.contains("connection")
                    || msg.contains("timeout")
                    || msg.contains("timed out")
                    || msg.contains("refused")
            }
            Error::Provider(ProviderError::ResponseError { status, .. }) => {
                // Rate limiting, server errors, auth errors (try next provider)
                *status == 429 || *status >= 500 || *status == 401 || *status == 403
            }
            Error::Provider(ProviderError::ApiKeyNotConfigured(_)) => {
                // Missing API key - try next provider
                true
            }
            _ => false,
        }
    }
}

#[async_trait]
impl ModelProvider for FallbackProvider {
    async fn complete(&self, request: CompletionRequest) -> Result<CompletionResponse> {
        let mut last_error = None;

        for (i, provider) in self.providers.iter().enumerate() {
            match provider.complete(request.clone()).await {
                Ok(response) => {
                    if i > 0 {
                        tracing::info!(
                            "Request succeeded with fallback provider: {}",
                            provider.name()
                        );
                    }
                    return Ok(response);
                }
                Err(e) => {
                    if Self::should_fallback(&e) && i < self.providers.len() - 1 {
                        tracing::warn!(
                            "Provider {} failed, trying fallback: {}",
                            provider.name(),
                            e
                        );
                        last_error = Some(e);
                        continue;
                    } else {
                        // Not retryable or last provider - return the error
                        return Err(e);
                    }
                }
            }
        }

        // Should not reach here, but return last error if we do
        Err(last_error.unwrap_or_else(|| {
            Error::Provider(ProviderError::HttpError(
                "All fallback providers exhausted".to_string(),
            ))
        }))
    }

    fn name(&self) -> &str {
        // Return the primary provider's name
        self.providers
            .first()
            .map(|p| p.name())
            .unwrap_or("fallback")
    }

    fn default_model(&self) -> &str {
        // Return the primary provider's default model
        self.providers
            .first()
            .map(|p| p.default_model())
            .unwrap_or("")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_should_fallback_rate_limited() {
        let error = Error::Provider(ProviderError::RateLimited("rate limited".to_string()));
        assert!(FallbackProvider::should_fallback(&error));
    }

    #[test]
    fn test_should_fallback_server_error() {
        let error = Error::Provider(ProviderError::ResponseError {
            status: 500,
            message: "Internal server error".to_string(),
        });
        assert!(FallbackProvider::should_fallback(&error));
    }

    #[test]
    fn test_should_fallback_auth_error() {
        let error = Error::Provider(ProviderError::ResponseError {
            status: 401,
            message: "Unauthorized".to_string(),
        });
        assert!(FallbackProvider::should_fallback(&error));
    }

    #[test]
    fn test_should_fallback_api_key_not_configured() {
        let error = Error::Provider(ProviderError::ApiKeyNotConfigured("openrouter".to_string()));
        assert!(FallbackProvider::should_fallback(&error));
    }

    #[test]
    fn test_should_not_fallback_bad_request() {
        let error = Error::Provider(ProviderError::ResponseError {
            status: 400,
            message: "Bad request".to_string(),
        });
        assert!(!FallbackProvider::should_fallback(&error));
    }
}
