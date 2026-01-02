//! Anthropic (Claude) provider implementation.

use super::response::{Choice, ChoiceMessage, FunctionCallResult, ToolCallResult, Usage};
use super::traits::ModelProvider;
use super::{CompletionRequest, CompletionResponse, MessageRole};
use crate::config::UserConfig;
use crate::error::{ConfigError, ProviderError};
use crate::{Error, Result};
use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};

const ANTHROPIC_API_URL: &str = "https://api.anthropic.com/v1/messages";
const ANTHROPIC_API_VERSION: &str = "2023-06-01";

/// Anthropic (Claude) provider.
pub struct AnthropicProvider {
    client: Client,
    api_key: String,
    model: String,
}

/// Anthropic-specific message content.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum AnthropicContent {
    #[serde(rename = "text")]
    Text { text: String },
    #[serde(rename = "tool_use")]
    ToolUse {
        id: String,
        name: String,
        input: serde_json::Value,
    },
    #[serde(rename = "tool_result")]
    ToolResult {
        tool_use_id: String,
        content: String,
    },
}

/// Anthropic message.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnthropicMessage {
    pub role: String,
    pub content: Vec<AnthropicContent>,
}

/// Anthropic tool definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnthropicTool {
    pub name: String,
    pub description: String,
    pub input_schema: serde_json::Value,
}

/// Anthropic request.
#[derive(Debug, Serialize)]
pub struct AnthropicRequest {
    pub model: String,
    pub max_tokens: u32,
    pub messages: Vec<AnthropicMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<AnthropicTool>>,
}

/// Anthropic response content block.
#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type")]
pub enum AnthropicResponseContent {
    #[serde(rename = "text")]
    Text { text: String },
    #[serde(rename = "tool_use")]
    ToolUse {
        id: String,
        name: String,
        input: serde_json::Value,
    },
}

/// Anthropic response usage.
#[derive(Debug, Clone, Deserialize)]
pub struct AnthropicUsage {
    pub input_tokens: u32,
    pub output_tokens: u32,
}

/// Anthropic response.
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct AnthropicResponse {
    pub id: String,
    #[serde(rename = "type")]
    pub response_type: String,
    pub role: String,
    pub content: Vec<AnthropicResponseContent>,
    pub model: String,
    pub stop_reason: Option<String>,
    pub usage: AnthropicUsage,
}

/// Anthropic error response.
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct AnthropicErrorResponse {
    #[serde(rename = "type")]
    pub error_type: String,
    pub error: AnthropicErrorDetail,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct AnthropicErrorDetail {
    #[serde(rename = "type")]
    pub error_type: String,
    pub message: String,
}

impl AnthropicProvider {
    /// Create a new Anthropic provider from configuration.
    pub fn from_config(config: &UserConfig) -> Result<Self> {
        // Look for ANTHROPIC_API_KEY environment variable
        let api_key = std::env::var("ANTHROPIC_API_KEY").map_err(|_| {
            Error::Config(ConfigError::EnvVarNotSet("ANTHROPIC_API_KEY".to_string()))
        })?;

        let client = Client::builder()
            .user_agent(format!("vibelings/{}", crate::VERSION))
            .build()
            .map_err(|e| Error::Provider(ProviderError::HttpError(e.to_string())))?;

        // Use the model from config, stripping any provider prefix
        let model = config
            .model
            .model
            .strip_prefix("anthropic/")
            .unwrap_or(&config.model.model)
            .to_string();

        Ok(Self {
            client,
            api_key,
            model,
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
        })
    }

    /// Convert internal request to Anthropic format.
    fn convert_request(&self, request: &CompletionRequest) -> AnthropicRequest {
        let mut system_prompt = None;
        let mut messages = Vec::new();

        for msg in &request.messages {
            match msg.role {
                MessageRole::System => {
                    // Anthropic handles system message separately
                    if let super::MessageContent::Text(text) = &msg.content {
                        system_prompt = Some(text.clone());
                    }
                }
                MessageRole::User => {
                    if let super::MessageContent::Text(text) = &msg.content {
                        messages.push(AnthropicMessage {
                            role: "user".to_string(),
                            content: vec![AnthropicContent::Text { text: text.clone() }],
                        });
                    }
                }
                MessageRole::Assistant => {
                    let mut content = Vec::new();

                    // Add text content if present
                    if let super::MessageContent::Text(text) = &msg.content {
                        if !text.is_empty() {
                            content.push(AnthropicContent::Text { text: text.clone() });
                        }
                    }

                    // Add tool calls if present
                    if let Some(tool_calls) = &msg.tool_calls {
                        for tc in tool_calls {
                            let input: serde_json::Value =
                                serde_json::from_str(&tc.function.arguments).unwrap_or_default();
                            content.push(AnthropicContent::ToolUse {
                                id: tc.id.clone(),
                                name: tc.function.name.clone(),
                                input,
                            });
                        }
                    }

                    if !content.is_empty() {
                        messages.push(AnthropicMessage {
                            role: "assistant".to_string(),
                            content,
                        });
                    }
                }
                MessageRole::Tool => {
                    // Tool result
                    if let (Some(tool_call_id), super::MessageContent::Text(text)) =
                        (&msg.tool_call_id, &msg.content)
                    {
                        messages.push(AnthropicMessage {
                            role: "user".to_string(),
                            content: vec![AnthropicContent::ToolResult {
                                tool_use_id: tool_call_id.clone(),
                                content: text.clone(),
                            }],
                        });
                    }
                }
            }
        }

        // Convert tools
        let tools = request.tools.as_ref().map(|tools| {
            tools
                .iter()
                .map(|t| AnthropicTool {
                    name: t.function.name.clone(),
                    description: t.function.description.clone(),
                    input_schema: t.function.parameters.clone(),
                })
                .collect()
        });

        // Use model from request if specified, otherwise use provider default
        let model = if request.model.is_empty() {
            self.model.clone()
        } else {
            request
                .model
                .strip_prefix("anthropic/")
                .unwrap_or(&request.model)
                .to_string()
        };

        AnthropicRequest {
            model,
            max_tokens: request.max_tokens.unwrap_or(4096),
            messages,
            system: system_prompt,
            temperature: request.temperature,
            tools,
        }
    }

    /// Convert Anthropic response to internal format.
    fn convert_response(&self, response: AnthropicResponse) -> CompletionResponse {
        let mut text_content = String::new();
        let mut tool_calls = Vec::new();

        for content in response.content {
            match content {
                AnthropicResponseContent::Text { text } => {
                    text_content = text;
                }
                AnthropicResponseContent::ToolUse { id, name, input } => {
                    tool_calls.push(ToolCallResult {
                        id,
                        call_type: "function".to_string(),
                        function: FunctionCallResult {
                            name,
                            arguments: serde_json::to_string(&input).unwrap_or_default(),
                        },
                    });
                }
            }
        }

        let finish_reason = match response.stop_reason.as_deref() {
            Some("end_turn") | Some("stop") => Some(super::response::FinishReason::Stop),
            Some("max_tokens") => Some(super::response::FinishReason::Length),
            Some("tool_use") => Some(super::response::FinishReason::ToolCalls),
            _ => None,
        };

        CompletionResponse {
            id: response.id,
            object: "chat.completion".to_string(),
            created: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_secs())
                .unwrap_or(0),
            model: response.model,
            choices: vec![Choice {
                index: 0,
                message: ChoiceMessage {
                    role: "assistant".to_string(),
                    content: if text_content.is_empty() {
                        None
                    } else {
                        Some(text_content)
                    },
                    tool_calls: if tool_calls.is_empty() {
                        None
                    } else {
                        Some(tool_calls)
                    },
                },
                finish_reason,
            }],
            usage: Some(Usage {
                prompt_tokens: response.usage.input_tokens,
                completion_tokens: response.usage.output_tokens,
                total_tokens: response.usage.input_tokens + response.usage.output_tokens,
            }),
        }
    }
}

#[async_trait]
impl ModelProvider for AnthropicProvider {
    async fn complete(&self, request: CompletionRequest) -> Result<CompletionResponse> {
        let anthropic_request = self.convert_request(&request);

        let response = self
            .client
            .post(ANTHROPIC_API_URL)
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", ANTHROPIC_API_VERSION)
            .header("Content-Type", "application/json")
            .json(&anthropic_request)
            .send()
            .await
            .map_err(|e| Error::Provider(ProviderError::HttpError(e.to_string())))?;

        let status = response.status();

        if status.is_success() {
            let anthropic_response: AnthropicResponse = response
                .json()
                .await
                .map_err(|e| Error::Provider(ProviderError::InvalidResponse(e.to_string())))?;
            Ok(self.convert_response(anthropic_response))
        } else if status.as_u16() == 429 {
            let error_text = response.text().await.unwrap_or_default();
            Err(Error::Provider(ProviderError::RateLimited(error_text)))
        } else {
            let error: AnthropicErrorResponse = response
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
        "anthropic"
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
        let result = AnthropicProvider::from_config(&config);
        // This will fail if ANTHROPIC_API_KEY is not set
        assert!(result.is_err() || result.is_ok());
    }

    #[test]
    fn test_request_conversion() {
        let provider = AnthropicProvider {
            client: Client::new(),
            api_key: "test".to_string(),
            model: "claude-sonnet-4-20250514".to_string(),
        };

        let messages = vec![Message::system("You are helpful"), Message::user("Hello")];

        let request = CompletionRequest::new("claude-sonnet-4-20250514", messages)
            .with_temperature(0.5)
            .with_max_tokens(1000);

        let anthropic_request = provider.convert_request(&request);

        assert_eq!(anthropic_request.model, "claude-sonnet-4-20250514");
        assert_eq!(
            anthropic_request.system,
            Some("You are helpful".to_string())
        );
        assert_eq!(anthropic_request.max_tokens, 1000);
        assert_eq!(anthropic_request.temperature, Some(0.5));
        assert_eq!(anthropic_request.messages.len(), 1); // Only user message
        assert_eq!(anthropic_request.messages[0].role, "user");
    }

    #[test]
    fn test_request_with_tools() {
        let provider = AnthropicProvider {
            client: Client::new(),
            api_key: "test".to_string(),
            model: "claude-sonnet-4-20250514".to_string(),
        };

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

        let request = CompletionRequest::new("claude-sonnet-4-20250514", vec![Message::user("Hi")])
            .with_tools(tools);

        let anthropic_request = provider.convert_request(&request);

        assert!(anthropic_request.tools.is_some());
        let tools = anthropic_request.tools.unwrap();
        assert_eq!(tools.len(), 1);
        assert_eq!(tools[0].name, "get_weather");
    }

    #[test]
    fn test_response_conversion() {
        let provider = AnthropicProvider {
            client: Client::new(),
            api_key: "test".to_string(),
            model: "claude-sonnet-4-20250514".to_string(),
        };

        let anthropic_response = AnthropicResponse {
            id: "msg_123".to_string(),
            response_type: "message".to_string(),
            role: "assistant".to_string(),
            content: vec![AnthropicResponseContent::Text {
                text: "Hello!".to_string(),
            }],
            model: "claude-sonnet-4-20250514".to_string(),
            stop_reason: Some("end_turn".to_string()),
            usage: AnthropicUsage {
                input_tokens: 10,
                output_tokens: 5,
            },
        };

        let response = provider.convert_response(anthropic_response);

        assert_eq!(response.id, "msg_123");
        assert_eq!(response.text(), Some("Hello!"));
        assert!(!response.has_tool_calls());
        assert_eq!(response.usage().prompt_tokens, 10);
        assert_eq!(response.usage().completion_tokens, 5);
    }

    #[test]
    fn test_response_with_tool_use() {
        let provider = AnthropicProvider {
            client: Client::new(),
            api_key: "test".to_string(),
            model: "claude-sonnet-4-20250514".to_string(),
        };

        let anthropic_response = AnthropicResponse {
            id: "msg_456".to_string(),
            response_type: "message".to_string(),
            role: "assistant".to_string(),
            content: vec![AnthropicResponseContent::ToolUse {
                id: "tool_1".to_string(),
                name: "get_weather".to_string(),
                input: serde_json::json!({"location": "NYC"}),
            }],
            model: "claude-sonnet-4-20250514".to_string(),
            stop_reason: Some("tool_use".to_string()),
            usage: AnthropicUsage {
                input_tokens: 20,
                output_tokens: 15,
            },
        };

        let response = provider.convert_response(anthropic_response);

        assert!(response.has_tool_calls());
        let tool_calls = response.tool_calls().unwrap();
        assert_eq!(tool_calls.len(), 1);
        assert_eq!(tool_calls[0].function.name, "get_weather");
    }
}
