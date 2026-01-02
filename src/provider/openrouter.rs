//! OpenRouter provider implementation.

use super::traits::ModelProvider;
use super::{CompletionRequest, CompletionResponse};
use crate::config::UserConfig;
use crate::error::{ConfigError, ProviderError};
use crate::{Error, Result};
use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};

const OPENROUTER_API_URL: &str = "https://openrouter.ai/api/v1/chat/completions";

/// OpenRouter provider.
pub struct OpenRouterProvider {
    client: Client,
    api_key: String,
    model: String,
    zdr: bool,
    data_collection: String,
}

/// OpenRouter-specific request wrapper.
#[derive(Debug, Serialize)]
struct OpenRouterRequest {
    #[serde(flatten)]
    inner: CompletionRequest,
    #[serde(skip_serializing_if = "Option::is_none")]
    transforms: Option<Vec<String>>,
}

/// OpenRouter error response.
#[derive(Debug, Deserialize)]
struct OpenRouterError {
    error: OpenRouterErrorDetail,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct OpenRouterErrorDetail {
    message: String,
    #[serde(rename = "type")]
    error_type: Option<String>,
    code: Option<i32>,
}

impl OpenRouterProvider {
    /// Create a new OpenRouter provider from configuration.
    pub fn from_config(config: &UserConfig) -> Result<Self> {
        let api_key = std::env::var(&config.openrouter.api_key_env).map_err(|_| {
            Error::Config(ConfigError::EnvVarNotSet(
                config.openrouter.api_key_env.clone(),
            ))
        })?;

        let client = Client::builder()
            .user_agent(format!("vibelings/{}", crate::VERSION))
            .build()
            .map_err(|e| Error::Provider(ProviderError::HttpError(e.to_string())))?;

        Ok(Self {
            client,
            api_key,
            model: config.model.model.clone(),
            zdr: config.openrouter.zdr,
            data_collection: config.openrouter.data_collection.clone(),
        })
    }

    /// Create with explicit API key (for testing).
    pub fn new(api_key: String, model: String) -> Result<Self> {
        let client = Client::builder()
            .user_agent(format!("vibelings/{}", crate::VERSION))
            .build()
            .map_err(|e| Error::Provider(ProviderError::HttpError(e.to_string())))?;

        Ok(Self {
            client,
            api_key,
            model,
            zdr: true,
            data_collection: "deny".to_string(),
        })
    }
}

#[async_trait]
impl ModelProvider for OpenRouterProvider {
    async fn complete(&self, mut request: CompletionRequest) -> Result<CompletionResponse> {
        // Use the provider's model if not specified in request
        if request.model.is_empty() {
            request.model = self.model.clone();
        }

        let openrouter_request = OpenRouterRequest {
            inner: request,
            transforms: None,
        };

        let mut req_builder = self
            .client
            .post(OPENROUTER_API_URL)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .header("HTTP-Referer", "https://github.com/AbdelStark/vibelings")
            .header("X-Title", "vibelings");

        // Add ZDR header if enabled
        if self.zdr {
            req_builder = req_builder.header("X-ZDR", "true");
        }

        // Add data collection header
        req_builder = req_builder.header("X-Data-Collection", self.data_collection.as_str());

        let response = req_builder
            .json(&openrouter_request)
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
            let error: OpenRouterError = response
                .json()
                .await
                .map_err(|e| Error::Provider(ProviderError::InvalidResponse(e.to_string())))?;
            Err(Error::Provider(ProviderError::ResponseError {
                status: status.as_u16(),
                message: error.error.message,
            }))
        }
    }

    fn name(&self) -> &str {
        "openrouter"
    }

    fn default_model(&self) -> &str {
        &self.model
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_provider_creation_without_key() {
        // Should fail because env var is not set
        let config = UserConfig::default();
        let result = OpenRouterProvider::from_config(&config);
        assert!(result.is_err());
    }
}
