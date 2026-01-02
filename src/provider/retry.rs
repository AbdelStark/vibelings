//! Retry logic for handling rate limiting and transient failures.
//!
//! This module provides a retry wrapper for model providers that handles
//! rate limiting (429 errors) with exponential backoff and jitter.
//!
//! The jitter helps prevent the "thundering herd" problem where multiple
//! clients retry at exactly the same time after receiving rate limit errors.

use super::traits::ModelProvider;
use super::{CompletionRequest, CompletionResponse};
use crate::error::ProviderError;
use crate::{Error, Result};
use async_trait::async_trait;
use rand::Rng;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;

/// Configuration for retry behavior.
#[derive(Debug, Clone)]
pub struct RetryConfig {
    /// Maximum number of retry attempts (not including the initial attempt)
    pub max_retries: u32,

    /// Initial delay before first retry (in milliseconds)
    pub initial_delay_ms: u64,

    /// Maximum delay between retries (in milliseconds)
    pub max_delay_ms: u64,

    /// Multiplier for exponential backoff (typically 2.0)
    pub backoff_multiplier: f64,

    /// Whether to add jitter to delays (prevents thundering herd)
    pub use_jitter: bool,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_retries: 3,
            initial_delay_ms: 1000, // 1 second
            max_delay_ms: 30000,    // 30 seconds
            backoff_multiplier: 2.0,
            use_jitter: true, // Enable jitter by default
        }
    }
}

impl RetryConfig {
    /// Create a retry config with custom values.
    pub fn new(max_retries: u32, initial_delay_ms: u64) -> Self {
        Self {
            max_retries,
            initial_delay_ms,
            ..Default::default()
        }
    }

    /// Create a retry config without jitter (for testing or deterministic behavior).
    pub fn without_jitter(mut self) -> Self {
        self.use_jitter = false;
        self
    }

    /// Calculate base delay for a given attempt (0-indexed), before jitter.
    fn base_delay_for_attempt(&self, attempt: u32) -> u64 {
        let delay_ms =
            (self.initial_delay_ms as f64) * self.backoff_multiplier.powi(attempt as i32);
        delay_ms.min(self.max_delay_ms as f64) as u64
    }

    /// Calculate delay for a given attempt (0-indexed), with optional jitter.
    ///
    /// When jitter is enabled, uses "full jitter" which picks a random value
    /// between 0 and the calculated delay. This provides the best distribution
    /// for avoiding thundering herd issues.
    fn delay_for_attempt(&self, attempt: u32) -> Duration {
        let base_delay_ms = self.base_delay_for_attempt(attempt);

        let delay_ms = if self.use_jitter {
            // Full jitter: random value in [0, base_delay]
            let mut rng = rand::rng();
            rng.random_range(0..=base_delay_ms)
        } else {
            base_delay_ms
        };

        Duration::from_millis(delay_ms)
    }
}

/// A provider wrapper that adds retry logic with exponential backoff.
///
/// This wrapper catches rate limiting errors (429) and transient connection
/// errors, retrying the request with exponential backoff.
pub struct RetryingProvider {
    inner: Arc<dyn ModelProvider>,
    config: RetryConfig,
}

impl RetryingProvider {
    /// Create a new retrying provider wrapper.
    pub fn new(inner: Arc<dyn ModelProvider>, config: RetryConfig) -> Self {
        Self { inner, config }
    }

    /// Check if an error is retryable.
    fn is_retryable(error: &Error) -> bool {
        match error {
            Error::Provider(ProviderError::RateLimited(_)) => true,
            Error::Provider(ProviderError::HttpError(msg)) => {
                // Retry on connection errors
                msg.contains("connection") || msg.contains("timeout") || msg.contains("timed out")
            }
            Error::Provider(ProviderError::ResponseError { status, .. }) => {
                // Retry on server errors (5xx) and rate limiting (429)
                *status == 429 || *status >= 500
            }
            _ => false,
        }
    }
}

#[async_trait]
impl ModelProvider for RetryingProvider {
    async fn complete(&self, request: CompletionRequest) -> Result<CompletionResponse> {
        let mut last_error = None;

        for attempt in 0..=self.config.max_retries {
            match self.inner.complete(request.clone()).await {
                Ok(response) => return Ok(response),
                Err(e) => {
                    if Self::is_retryable(&e) && attempt < self.config.max_retries {
                        let delay = self.config.delay_for_attempt(attempt);
                        tracing::warn!(
                            "Request failed (attempt {}/{}), retrying in {:?}: {}",
                            attempt + 1,
                            self.config.max_retries + 1,
                            delay,
                            e
                        );
                        sleep(delay).await;
                        last_error = Some(e);
                    } else {
                        return Err(e);
                    }
                }
            }
        }

        // Should not reach here, but return last error if we do
        Err(last_error.unwrap_or_else(|| {
            Error::Provider(ProviderError::HttpError(
                "Retry exhausted without error".to_string(),
            ))
        }))
    }

    fn name(&self) -> &str {
        self.inner.name()
    }

    fn default_model(&self) -> &str {
        self.inner.default_model()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = RetryConfig::default();
        assert_eq!(config.max_retries, 3);
        assert_eq!(config.initial_delay_ms, 1000);
        assert_eq!(config.max_delay_ms, 30000);
        assert_eq!(config.backoff_multiplier, 2.0);
        assert!(config.use_jitter);
    }

    #[test]
    fn test_base_delay_calculation() {
        let config = RetryConfig {
            max_retries: 5,
            initial_delay_ms: 1000,
            max_delay_ms: 10000,
            backoff_multiplier: 2.0,
            use_jitter: false,
        };

        // Attempt 0: 1000ms * 2^0 = 1000ms
        assert_eq!(config.base_delay_for_attempt(0), 1000);

        // Attempt 1: 1000ms * 2^1 = 2000ms
        assert_eq!(config.base_delay_for_attempt(1), 2000);

        // Attempt 2: 1000ms * 2^2 = 4000ms
        assert_eq!(config.base_delay_for_attempt(2), 4000);

        // Attempt 3: 1000ms * 2^3 = 8000ms
        assert_eq!(config.base_delay_for_attempt(3), 8000);

        // Attempt 4: 1000ms * 2^4 = 16000ms, but capped at 10000ms
        assert_eq!(config.base_delay_for_attempt(4), 10000);
    }

    #[test]
    fn test_delay_without_jitter() {
        let config = RetryConfig::new(3, 1000).without_jitter();

        // Without jitter, delay_for_attempt should return the base delay
        assert_eq!(config.delay_for_attempt(0), Duration::from_millis(1000));
        assert_eq!(config.delay_for_attempt(1), Duration::from_millis(2000));
        assert_eq!(config.delay_for_attempt(2), Duration::from_millis(4000));
    }

    #[test]
    fn test_delay_with_jitter_is_bounded() {
        let config = RetryConfig {
            max_retries: 3,
            initial_delay_ms: 1000,
            max_delay_ms: 30000,
            backoff_multiplier: 2.0,
            use_jitter: true,
        };

        // With jitter, delay should be between 0 and base_delay
        for _ in 0..100 {
            let delay = config.delay_for_attempt(0);
            assert!(delay <= Duration::from_millis(1000));

            let delay = config.delay_for_attempt(1);
            assert!(delay <= Duration::from_millis(2000));
        }
    }

    #[test]
    fn test_is_retryable_rate_limited() {
        let error = Error::Provider(ProviderError::RateLimited("rate limited".to_string()));
        assert!(RetryingProvider::is_retryable(&error));
    }

    #[test]
    fn test_is_retryable_server_error() {
        let error = Error::Provider(ProviderError::ResponseError {
            status: 500,
            message: "Internal server error".to_string(),
        });
        assert!(RetryingProvider::is_retryable(&error));
    }

    #[test]
    fn test_is_retryable_429() {
        let error = Error::Provider(ProviderError::ResponseError {
            status: 429,
            message: "Too many requests".to_string(),
        });
        assert!(RetryingProvider::is_retryable(&error));
    }

    #[test]
    fn test_not_retryable_400() {
        let error = Error::Provider(ProviderError::ResponseError {
            status: 400,
            message: "Bad request".to_string(),
        });
        assert!(!RetryingProvider::is_retryable(&error));
    }

    #[test]
    fn test_not_retryable_auth() {
        let error = Error::Provider(ProviderError::ResponseError {
            status: 401,
            message: "Unauthorized".to_string(),
        });
        assert!(!RetryingProvider::is_retryable(&error));
    }

    #[test]
    fn test_retryable_connection_error() {
        let error = Error::Provider(ProviderError::HttpError("connection refused".to_string()));
        assert!(RetryingProvider::is_retryable(&error));
    }

    #[test]
    fn test_retryable_timeout() {
        let error = Error::Provider(ProviderError::HttpError("request timed out".to_string()));
        assert!(RetryingProvider::is_retryable(&error));
    }
}
