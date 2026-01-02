//! Response types for model completion.

use serde::{Deserialize, Serialize};

/// Reason for completion finishing.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum FinishReason {
    /// Natural stop (end of generation)
    Stop,
    /// Hit max tokens limit
    Length,
    /// Model wants to call tools
    ToolCalls,
    /// Content was filtered
    ContentFilter,
    /// Function call (legacy)
    FunctionCall,
}

/// Token usage information.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Usage {
    /// Tokens in the prompt
    pub prompt_tokens: u32,
    /// Tokens in the completion
    pub completion_tokens: u32,
    /// Total tokens used
    pub total_tokens: u32,
}

impl Usage {
    /// Calculate approximate cost in USD based on model pricing.
    pub fn estimate_cost(&self, price_per_1k_prompt: f64, price_per_1k_completion: f64) -> f64 {
        let prompt_cost = (self.prompt_tokens as f64 / 1000.0) * price_per_1k_prompt;
        let completion_cost = (self.completion_tokens as f64 / 1000.0) * price_per_1k_completion;
        prompt_cost + completion_cost
    }
}

/// A tool call in the response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCallResult {
    /// Unique ID for this tool call
    pub id: String,
    /// Type of call (always "function" for now)
    #[serde(rename = "type")]
    pub call_type: String,
    /// Function call details
    pub function: FunctionCallResult,
}

/// Function call result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionCallResult {
    /// Name of the function to call
    pub name: String,
    /// JSON-encoded arguments
    pub arguments: String,
}

/// A choice in the completion response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Choice {
    /// Index of this choice
    pub index: u32,
    /// The message content
    pub message: ChoiceMessage,
    /// Why generation finished
    pub finish_reason: Option<FinishReason>,
}

/// Message content in a choice.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChoiceMessage {
    /// Role of the message
    pub role: String,
    /// Text content (may be null if tool_calls present)
    pub content: Option<String>,
    /// Tool calls requested by the model
    #[serde(default)]
    pub tool_calls: Option<Vec<ToolCallResult>>,
}

/// A completion response from the model.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletionResponse {
    /// Unique response ID
    pub id: String,
    /// Object type (always "chat.completion")
    pub object: String,
    /// Unix timestamp of creation
    pub created: u64,
    /// Model used
    pub model: String,
    /// Generated choices
    pub choices: Vec<Choice>,
    /// Token usage
    pub usage: Option<Usage>,
}

impl CompletionResponse {
    /// Get the primary text content (first choice).
    pub fn text(&self) -> Option<&str> {
        self.choices
            .first()
            .and_then(|c| c.message.content.as_deref())
    }

    /// Get the finish reason (first choice).
    pub fn finish_reason(&self) -> Option<&FinishReason> {
        self.choices.first().and_then(|c| c.finish_reason.as_ref())
    }

    /// Check if the model wants to call tools.
    pub fn has_tool_calls(&self) -> bool {
        self.choices
            .first()
            .and_then(|c| c.message.tool_calls.as_ref())
            .is_some_and(|tc| !tc.is_empty())
    }

    /// Get the tool calls from the response.
    pub fn tool_calls(&self) -> Option<&[ToolCallResult]> {
        self.choices
            .first()
            .and_then(|c| c.message.tool_calls.as_deref())
    }

    /// Get usage information.
    pub fn usage(&self) -> Usage {
        self.usage.clone().unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_usage_cost_calculation() {
        let usage = Usage {
            prompt_tokens: 1000,
            completion_tokens: 500,
            total_tokens: 1500,
        };

        // Example: $0.01 per 1K prompt, $0.03 per 1K completion
        let cost = usage.estimate_cost(0.01, 0.03);
        assert!((cost - 0.025).abs() < 0.0001);
    }

    #[test]
    fn test_response_parsing() {
        let json = r#"{
            "id": "chatcmpl-123",
            "object": "chat.completion",
            "created": 1677652288,
            "model": "gpt-4",
            "choices": [{
                "index": 0,
                "message": {
                    "role": "assistant",
                    "content": "Hello!"
                },
                "finish_reason": "stop"
            }],
            "usage": {
                "prompt_tokens": 10,
                "completion_tokens": 5,
                "total_tokens": 15
            }
        }"#;

        let response: CompletionResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.text(), Some("Hello!"));
        assert_eq!(response.finish_reason(), Some(&FinishReason::Stop));
        assert!(!response.has_tool_calls());
    }
}
