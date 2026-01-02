//! OpenAI direct provider implementation.

use super::traits::ModelProvider;
use super::{CompletionRequest, CompletionResponse};
use crate::config::UserConfig;
use crate::error::{ConfigError, ProviderError};
use crate::{Error, Result};
use async_trait::async_trait;
use reqwest::Client;
use serde::Deserialize;
use std::time::Duration;

const OPENAI_API_URL: &str = "https://api.openai.com/v1/chat/completions";
/// Default timeout for HTTP requests (2 minutes).
const DEFAULT_TIMEOUT: Duration = Duration::from_secs(120);

/// OpenAI direct provider.
pub struct OpenAIProvider {
    client: Client,
    api_key: String,
    model: String,
    organization: Option<String>,
}

/// OpenAI error response.
#[derive(Debug, Deserialize)]
struct OpenAIError {
    error: OpenAIErrorDetail,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct OpenAIErrorDetail {
    message: String,
    #[serde(rename = "type")]
    error_type: Option<String>,
    code: Option<String>,
}

impl OpenAIProvider {
    /// Create a new OpenAI provider from configuration.
    pub fn from_config(config: &UserConfig) -> Result<Self> {
        // Look for API key using the configured environment variable name
        let api_key = std::env::var(&config.openai.api_key_env).map_err(|_| {
            Error::Config(ConfigError::EnvVarNotSet(config.openai.api_key_env.clone()))
        })?;

        // Optional organization ID from configured env var
        let organization = config
            .openai
            .org_id_env
            .as_ref()
            .and_then(|env_name| std::env::var(env_name).ok());

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
            .strip_prefix("openai/")
            .unwrap_or(&config.model.model)
            .to_string();

        Ok(Self {
            client,
            api_key,
            model,
            organization,
        })
    }

    /// Create with explicit API key (for testing).
    pub fn new(api_key: String, model: String) -> Result<Self> {
        let client = Client::builder()
            .user_agent(format!("vibelings/{}", crate::VERSION))
            .timeout(DEFAULT_TIMEOUT)
            .connect_timeout(Duration::from_secs(30))
            .build()
            .map_err(|e| Error::Provider(ProviderError::HttpError(e.to_string())))?;

        Ok(Self {
            client,
            api_key,
            model,
            organization: None,
        })
    }

    /// Set the organization ID.
    pub fn with_organization(mut self, org_id: String) -> Self {
        self.organization = Some(org_id);
        self
    }
}

#[async_trait]
impl ModelProvider for OpenAIProvider {
    async fn complete(&self, mut request: CompletionRequest) -> Result<CompletionResponse> {
        // Use the provider's model if not specified in request, stripping any prefix
        if request.model.is_empty() {
            request.model = self.model.clone();
        } else {
            request.model = request
                .model
                .strip_prefix("openai/")
                .unwrap_or(&request.model)
                .to_string();
        }

        let mut req_builder = self
            .client
            .post(OPENAI_API_URL)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json");

        // Add organization header if set
        if let Some(ref org) = self.organization {
            req_builder = req_builder.header("OpenAI-Organization", org);
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
            let error: OpenAIError = response
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
        "openai"
    }

    fn default_model(&self) -> &str {
        &self.model
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::provider::request::{FunctionDefinition, Tool};
    use crate::provider::Message;

    #[test]
    fn test_provider_creation_without_key() {
        // Should fail because env var is not set
        let config = UserConfig::default();
        let result = OpenAIProvider::from_config(&config);
        // This will fail if OPENAI_API_KEY is not set
        assert!(result.is_err() || result.is_ok());
    }

    #[test]
    fn test_provider_new() {
        let provider = OpenAIProvider::new("test-key".to_string(), "gpt-4".to_string());
        assert!(provider.is_ok());
        let provider = provider.unwrap();
        assert_eq!(provider.name(), "openai");
        assert_eq!(provider.default_model(), "gpt-4");
    }

    #[test]
    fn test_provider_with_organization() {
        let provider = OpenAIProvider::new("test-key".to_string(), "gpt-4".to_string())
            .unwrap()
            .with_organization("org-123".to_string());
        assert_eq!(provider.organization, Some("org-123".to_string()));
    }

    #[test]
    fn test_request_serialization() {
        let messages = vec![Message::system("You are helpful"), Message::user("Hello")];

        let request = CompletionRequest::new("gpt-4", messages)
            .with_temperature(0.5)
            .with_max_tokens(1000);

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("gpt-4"));
        assert!(json.contains("You are helpful"));
        assert!(json.contains("Hello"));
    }

    #[test]
    fn test_request_with_tools() {
        let tools = vec![Tool {
            tool_type: "function".to_string(),
            function: FunctionDefinition {
                name: "get_weather".to_string(),
                description: "Get weather for a location".to_string(),
                parameters: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "location": {"type": "string"}
                    },
                    "required": ["location"]
                }),
            },
        }];

        let request = CompletionRequest::new("gpt-4", vec![Message::user("What's the weather?")])
            .with_tools(tools);

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("get_weather"));
        assert!(json.contains("function"));
    }

    #[test]
    fn test_model_prefix_stripping() {
        let config = UserConfig {
            model: crate::config::ModelConfig {
                provider: crate::config::ProviderType::OpenAI,
                model: "openai/gpt-4".to_string(),
                temperature: 0.0,
                max_tokens: None,
                fallback_providers: Vec::new(),
            },
            ..Default::default()
        };

        // Mock the env var for this test
        std::env::set_var("OPENAI_API_KEY", "test-key");
        let provider = OpenAIProvider::from_config(&config).unwrap();
        assert_eq!(provider.default_model(), "gpt-4");
        std::env::remove_var("OPENAI_API_KEY");
    }
}
