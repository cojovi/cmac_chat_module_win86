//! OpenWebUI API client for LLM interactions
//!
//! Handles message sending to OpenWebUI with conversation context management,
//! streaming support, and proper error handling.

use crate::config::OpenWebUiConfig;
use crate::error::{AppResult, OpenWebUiError};
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// OpenWebUI API client
pub struct OpenWebUiClient {
    client: reqwest::Client,
    config: OpenWebUiConfig,
    api_key: Option<String>,
}

/// Chat message for API request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

/// Chat completion request
#[derive(Debug, Serialize)]
struct ChatCompletionRequest {
    model: String,
    messages: Vec<ChatMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tokens: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stream: Option<bool>,
}

/// Chat completion response
#[derive(Debug, Deserialize)]
pub struct ChatCompletionResponse {
    pub id: Option<String>,
    pub model: Option<String>,
    pub choices: Vec<ChatChoice>,
    pub usage: Option<ChatUsage>,
}

/// Chat choice from response
#[derive(Debug, Deserialize)]
pub struct ChatChoice {
    pub index: usize,
    pub message: ChatMessage,
    pub finish_reason: Option<String>,
}

/// Token usage information
#[derive(Debug, Deserialize)]
pub struct ChatUsage {
    pub prompt_tokens: usize,
    pub completion_tokens: usize,
    pub total_tokens: usize,
}

/// OpenWebUI error response
#[derive(Debug, Deserialize)]
struct OpenWebUiErrorResponse {
    error: OpenWebUiErrorDetail,
}

#[derive(Debug, Deserialize)]
struct OpenWebUiErrorDetail {
    message: String,
    #[serde(rename = "type")]
    error_type: Option<String>,
}

impl OpenWebUiClient {
    /// Create a new OpenWebUI client
    pub fn new(config: OpenWebUiConfig, api_key: Option<String>) -> AppResult<Self> {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(config.timeout_secs))
            .build()
            .map_err(|e| OpenWebUiError::MessageSendFailed(e.to_string()))?;

        Ok(Self {
            client,
            config,
            api_key,
        })
    }

    /// Send a message to the LLM with conversation context
    ///
    /// # Arguments
    /// * `messages` - Conversation history as (role, content) tuples
    ///
    /// # Returns
    /// The assistant's response text
    pub async fn send_message(&self, messages: Vec<(String, String)>) -> AppResult<String> {
        log::info!("Sending message to OpenWebUI with {} messages in context", messages.len());

        // Convert to ChatMessage format
        let chat_messages: Vec<ChatMessage> = messages
            .into_iter()
            .map(|(role, content)| ChatMessage { role, content })
            .collect();

        // Validate context length
        let total_chars: usize = chat_messages.iter().map(|m| m.content.len()).sum();
        if total_chars > self.config.max_context_length {
            log::warn!("Context length ({}) exceeds maximum ({})", total_chars, self.config.max_context_length);
            return Err(OpenWebUiError::ContextLimitExceeded.into());
        }

        // Attempt with retry logic
        let max_retries = 3;
        let mut last_error = None;

        for attempt in 1..=max_retries {
            match self.try_send_message(&chat_messages).await {
                Ok(response) => {
                    log::info!("Message sent successfully, response length: {} chars", response.len());
                    return Ok(response);
                }
                Err(e) => {
                    last_error = Some(e);
                    if attempt < max_retries {
                        let delay = Duration::from_secs(2u64.pow(attempt as u32 - 1));
                        log::warn!("Message send attempt {} failed, retrying in {:?}", attempt, delay);
                        tokio::time::sleep(delay).await;
                    }
                }
            }
        }

        Err(last_error.unwrap())
    }

    /// Internal message sending attempt
    async fn try_send_message(&self, messages: &[ChatMessage]) -> AppResult<String> {
        // Build request payload
        let request_body = ChatCompletionRequest {
            model: self.config.model.clone(),
            messages: messages.to_vec(),
            temperature: Some(self.config.temperature),
            max_tokens: self.config.max_tokens,
            stream: Some(self.config.stream),
        };

        log::debug!("Request payload: model={}, messages={}, stream={}",
                   request_body.model, messages.len(), self.config.stream);

        // Build HTTP request
        let mut request = self.client
            .post(&self.config.endpoint)
            .json(&request_body);

        // Add authorization if API key is present
        if let Some(api_key) = &self.api_key {
            request = request.bearer_auth(api_key);
        }

        // Send request
        let response = request
            .send()
            .await
            .map_err(|e| {
                if e.is_timeout() {
                    OpenWebUiError::Timeout
                } else {
                    OpenWebUiError::MessageSendFailed(e.to_string())
                }
            })?;

        // Check status
        let status = response.status();
        if !status.is_success() {
            // Try to parse error response
            if let Ok(error_response) = response.json::<OpenWebUiErrorResponse>().await {
                return Err(match status.as_u16() {
                    401 | 403 => OpenWebUiError::AuthenticationFailed,
                    404 => OpenWebUiError::ModelNotFound(self.config.model.clone()),
                    429 => OpenWebUiError::RateLimitExceeded,
                    _ => OpenWebUiError::MessageSendFailed(error_response.error.message),
                }.into());
            }
            return Err(OpenWebUiError::MessageSendFailed(format!("HTTP {}", status)).into());
        }

        // Handle streaming vs non-streaming responses
        if self.config.stream {
            // For now, streaming is not fully implemented
            // This would require handling Server-Sent Events (SSE)
            log::warn!("Streaming responses not yet implemented, falling back to non-streaming");
            return Err(OpenWebUiError::StreamingNotSupported.into());
        }

        // Parse non-streaming response
        let result = response
            .json::<ChatCompletionResponse>()
            .await
            .map_err(|e| OpenWebUiError::ResponseParseFailed(e.to_string()))?;

        // Extract message from first choice
        if let Some(choice) = result.choices.first() {
            let response_text = choice.message.content.clone();

            // Log usage if available
            if let Some(usage) = result.usage {
                log::debug!("Token usage - prompt: {}, completion: {}, total: {}",
                           usage.prompt_tokens, usage.completion_tokens, usage.total_tokens);
            }

            Ok(response_text)
        } else {
            Err(OpenWebUiError::ResponseParseFailed("No choices in response".to_string()).into())
        }
    }

    /// Check connectivity to OpenWebUI API
    pub async fn check_connectivity(&self) -> AppResult<bool> {
        // Try a minimal request to check if the service is available
        let test_messages = vec![
            ChatMessage {
                role: "user".to_string(),
                content: "test".to_string(),
            }
        ];

        let request_body = ChatCompletionRequest {
            model: self.config.model.clone(),
            messages: test_messages,
            temperature: Some(0.1),
            max_tokens: Some(5),
            stream: Some(false),
        };

        let mut request = self.client
            .post(&self.config.endpoint)
            .json(&request_body)
            .timeout(Duration::from_secs(5));

        if let Some(api_key) = &self.api_key {
            request = request.bearer_auth(api_key);
        }

        match request.send().await {
            Ok(resp) => {
                let status = resp.status().as_u16();
                // Consider 2xx and 4xx (except 404) as "connected"
                // 404 means model not found, but service is reachable
                Ok(status < 500 && status != 404)
            }
            Err(e) => {
                log::warn!("OpenWebUI connectivity check failed: {}", e);
                Ok(false)
            }
        }
    }

    /// Update configuration
    pub fn update_config(&mut self, config: OpenWebUiConfig) {
        self.config = config;
    }

    /// Update API key
    pub fn update_api_key(&mut self, api_key: Option<String>) {
        self.api_key = api_key;
    }

    /// Get current model
    pub fn get_model(&self) -> &str {
        &self.config.model
    }

    /// Set model
    pub fn set_model(&mut self, model: String) {
        self.config.model = model;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_openwebui_client_creation() {
        let config = OpenWebUiConfig {
            endpoint: "http://localhost:3000/api/chat".to_string(),
            model: "llama3.2".to_string(),
            max_context_length: 4096,
            temperature: 0.7,
            max_tokens: Some(1024),
            stream: false,
            timeout_secs: 60,
        };

        let client = OpenWebUiClient::new(config, None);
        assert!(client.is_ok());
    }

    #[test]
    fn test_chat_message_serialization() {
        let message = ChatMessage {
            role: "user".to_string(),
            content: "Hello".to_string(),
        };

        let json = serde_json::to_string(&message).unwrap();
        assert!(json.contains("user"));
        assert!(json.contains("Hello"));
    }

    #[test]
    fn test_context_length_validation() {
        let config = OpenWebUiConfig {
            endpoint: "http://localhost:3000/api/chat".to_string(),
            model: "llama3.2".to_string(),
            max_context_length: 100, // Very small for testing
            temperature: 0.7,
            max_tokens: Some(1024),
            stream: false,
            timeout_secs: 60,
        };

        let client = OpenWebUiClient::new(config, None).unwrap();

        // Create messages that exceed context length
        let messages = vec![
            ("user".to_string(), "a".repeat(200)),
        ];

        let runtime = tokio::runtime::Runtime::new().unwrap();
        let result = runtime.block_on(client.send_message(messages));

        assert!(result.is_err());
    }
}
