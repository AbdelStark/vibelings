//! Local OpenAI-compatible provider implementation.
//!
//! Connects to any OpenAI-compatible endpoint such as:
//! - Ollama (http://localhost:11434/v1)
//! - vLLM (http://localhost:8000/v1)
//! - LM Studio
//! - LocalAI
//! - Any other OpenAI-compatible server

use super::traits::ModelProvider;
use super::{CompletionRequest, CompletionResponse};
use crate::config::UserConfig;
use crate::error::ProviderError;
use crate::{Error, Result};
use async_trait::async_trait;
use reqwest::Client;
use serde::Deserialize;
use std::time::Duration;

const DEFAULT_LOCAL_URL: &str = "http://localhost:11434/v1";
/// Default timeout for HTTP requests (2 minutes).
const DEFAULT_TIMEOUT: Duration = Duration::from_secs(120);

/// Local OpenAI-compatible provider.
///
/// Connects to a local LLM server that implements the OpenAI API.
pub struct LocalProvider {
    client: Client,
    base_url: String,
    api_key: Option<String>,
    model: String,
}

/// Local provider error response (OpenAI-compatible).
#[derive(Debug, Deserialize)]
struct LocalError {
    error: LocalErrorDetail,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct LocalErrorDetail {
    message: String,
    #[serde(rename = "type")]
    error_type: Option<String>,
    code: Option<String>,
}

impl LocalProvider {
    /// Create a new local provider from configuration.
    pub fn from_config(config: &UserConfig) -> Result<Self> {
        // Get base URL from LOCAL_API_URL env var, or use default
        let base_url =
            std::env::var("LOCAL_API_URL").unwrap_or_else(|_| DEFAULT_LOCAL_URL.to_string());

        // API key is optional for local providers
        let api_key = std::env::var("LOCAL_API_KEY").ok();

        let client = Client::builder()
            .user_agent(format!("vibelings/{}", crate::VERSION))
            .timeout(DEFAULT_TIMEOUT)
            .connect_timeout(Duration::from_secs(30))
            .build()
            .map_err(|e| Error::Provider(ProviderError::HttpError(e.to_string())))?;

        // Use the model from config, stripping any provider prefix
        let model = config
            .model
            .model
            .strip_prefix("local/")
            .unwrap_or(&config.model.model)
            .to_string();

        Ok(Self {
            client,
            base_url,
            api_key,
            model,
        })
    }

    /// Create with explicit configuration (for testing).
    pub fn new(base_url: String, model: String) -> Result<Self> {
        let client = Client::builder()
            .user_agent(format!("vibelings/{}", crate::VERSION))
            .timeout(DEFAULT_TIMEOUT)
            .connect_timeout(Duration::from_secs(30))
            .build()
            .map_err(|e| Error::Provider(ProviderError::HttpError(e.to_string())))?;

        Ok(Self {
            client,
            base_url,
            api_key: None,
            model,
        })
    }

    /// Set the API key.
    pub fn with_api_key(mut self, api_key: String) -> Self {
        self.api_key = Some(api_key);
        self
    }

    /// Get the chat completions endpoint URL.
    fn completions_url(&self) -> String {
        format!("{}/chat/completions", self.base_url.trim_end_matches('/'))
    }
}

#[async_trait]
impl ModelProvider for LocalProvider {
    async fn complete(&self, mut request: CompletionRequest) -> Result<CompletionResponse> {
        // Use the provider's model if not specified in request
        if request.model.is_empty() {
            request.model = self.model.clone();
        } else {
            request.model = request
                .model
                .strip_prefix("local/")
                .unwrap_or(&request.model)
                .to_string();
        }

        let mut req_builder = self
            .client
            .post(self.completions_url())
            .header("Content-Type", "application/json");

        // Add authorization header if API key is set
        if let Some(ref api_key) = self.api_key {
            req_builder = req_builder.header("Authorization", format!("Bearer {}", api_key));
        }

        let response = req_builder
            .json(&request)
            .send()
            .await
            .map_err(|e| Error::Provider(ProviderError::HttpError(e.to_string())))?;

        let status = response.status();

        if status.is_success() {
            let completion: CompletionResponse = response
                .json()
                .await
                .map_err(|e| Error::Provider(ProviderError::InvalidResponse(e.to_string())))?;
            Ok(completion)
        } else if status.as_u16() == 429 {
            let error_text = response.text().await.unwrap_or_default();
            Err(Error::Provider(ProviderError::RateLimited(error_text)))
        } else {
            // Try to parse as OpenAI-style error, fall back to raw text
            let error_text = response.text().await.unwrap_or_default();
            let message = serde_json::from_str::<LocalError>(&error_text)
                .map(|e| e.error.message)
                .unwrap_or(error_text);

            Err(Error::Provider(ProviderError::ResponseError {
                status: status.as_u16(),
                message,
            }))
        }
    }

    fn name(&self) -> &str {
        "local"
    }

    fn default_model(&self) -> &str {
        &self.model
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::provider::Message;

    #[test]
    fn test_provider_new() {
        let provider = LocalProvider::new(
            "http://localhost:11434/v1".to_string(),
            "llama3".to_string(),
        );
        assert!(provider.is_ok());
        let provider = provider.unwrap();
        assert_eq!(provider.name(), "local");
        assert_eq!(provider.default_model(), "llama3");
    }

    #[test]
    fn test_provider_with_api_key() {
        let provider = LocalProvider::new(
            "http://localhost:8000/v1".to_string(),
            "mistral".to_string(),
        )
        .unwrap()
        .with_api_key("test-key".to_string());

        assert_eq!(provider.api_key, Some("test-key".to_string()));
    }

    #[test]
    fn test_completions_url() {
        let provider = LocalProvider::new(
            "http://localhost:11434/v1".to_string(),
            "llama3".to_string(),
        )
        .unwrap();

        assert_eq!(
            provider.completions_url(),
            "http://localhost:11434/v1/chat/completions"
        );
    }

    #[test]
    fn test_completions_url_trailing_slash() {
        let provider = LocalProvider::new(
            "http://localhost:11434/v1/".to_string(),
            "llama3".to_string(),
        )
        .unwrap();

        assert_eq!(
            provider.completions_url(),
            "http://localhost:11434/v1/chat/completions"
        );
    }

    #[test]
    fn test_request_serialization() {
        let messages = vec![Message::system("You are helpful"), Message::user("Hello")];

        let request = CompletionRequest::new("llama3", messages)
            .with_temperature(0.7)
            .with_max_tokens(500);

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("llama3"));
        assert!(json.contains("You are helpful"));
        assert!(json.contains("Hello"));
    }

    #[test]
    fn test_model_prefix_stripping() {
        let config = UserConfig {
            model: crate::config::ModelConfig {
                provider: crate::config::ProviderType::Local,
                model: "local/llama3".to_string(),
                temperature: 0.0,
                max_tokens: None,
            },
            ..Default::default()
        };

        let provider = LocalProvider::from_config(&config).unwrap();
        assert_eq!(provider.default_model(), "llama3");
    }

    #[test]
    fn test_from_config_uses_defaults() {
        let config = UserConfig::default();
        let provider = LocalProvider::from_config(&config);
        assert!(provider.is_ok());
        let provider = provider.unwrap();
        // Base URL should be the default
        assert!(provider.base_url.contains("localhost"));
    }
}
