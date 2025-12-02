//! Application state management
//!
//! Manages the global application state including conversation context,
//! current processing state, and API connection status with thread-safe access.

use crate::config::{ApiKeys, AppConfig};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

/// Application state with thread-safe interior mutability
#[derive(Clone)]
pub struct AppState {
    inner: Arc<Mutex<AppStateInner>>,
}

/// Inner application state
struct AppStateInner {
    /// Current application status
    pub status: AppStatus,

    /// Conversation context (in-memory only)
    pub conversation: ConversationContext,

    /// Configuration
    pub config: AppConfig,

    /// API keys
    pub api_keys: ApiKeys,

    /// API connection status
    pub connectivity: ConnectivityStatus,
}

/// Application status enum
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum AppStatus {
    /// Application is idle
    Idle,

    /// Recording audio from microphone
    Recording,

    /// Listening for voice input (deprecated, use Recording)
    Listening,

    /// Transcribing audio via Whisper
    Transcribing,

    /// Processing message through LLM
    Thinking,

    /// Converting response to speech
    Speaking,

    /// Error state
    Error { message: String },
}

/// Conversation context structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationContext {
    /// Conversation ID
    pub id: String,

    /// Message history
    pub messages: Vec<Message>,

    /// Maximum number of messages to keep in context
    pub max_messages: usize,

    /// Started timestamp
    pub started_at: u64,

    /// Last updated timestamp
    pub updated_at: u64,
}

/// Message structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    /// Message role (user, assistant, system)
    pub role: MessageRole,

    /// Message content
    pub content: String,

    /// Timestamp
    pub timestamp: u64,
}

/// Message role enum
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum MessageRole {
    User,
    Assistant,
    System,
}

/// API connectivity status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectivityStatus {
    /// Whisper API status
    pub whisper: ServiceStatus,

    /// OpenWebUI API status
    pub openwebui: ServiceStatus,

    /// ElevenLabs API status
    pub elevenlabs: ServiceStatus,

    /// Last checked timestamp
    pub last_checked: u64,
}

/// Individual service status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ServiceStatus {
    /// Service is connected and operational
    Connected,

    /// Service connection is being checked
    Checking,

    /// Service is disconnected or unreachable
    Disconnected { reason: String },

    /// Service has not been checked yet
    Unknown,
}

impl AppState {
    /// Create a new application state
    pub fn new(config: AppConfig, api_keys: ApiKeys) -> Self {
        let now = current_timestamp();

        Self {
            inner: Arc::new(Mutex::new(AppStateInner {
                status: AppStatus::Idle,
                conversation: ConversationContext {
                    id: generate_id(),
                    messages: Vec::new(),
                    max_messages: 20,
                    started_at: now,
                    updated_at: now,
                },
                config,
                api_keys,
                connectivity: ConnectivityStatus {
                    whisper: ServiceStatus::Unknown,
                    openwebui: ServiceStatus::Unknown,
                    elevenlabs: ServiceStatus::Unknown,
                    last_checked: 0,
                },
            })),
        }
    }

    /// Get current status
    pub fn get_status(&self) -> AppStatus {
        let state = self.inner.lock().unwrap();
        state.status.clone()
    }

    /// Set application status
    pub fn set_status(&self, status: AppStatus) {
        let mut state = self.inner.lock().unwrap();
        log::info!("Status changed: {:?} -> {:?}", state.status, status);
        state.status = status;
    }

    /// Get configuration
    pub fn get_config(&self) -> AppConfig {
        let state = self.inner.lock().unwrap();
        state.config.clone()
    }

    /// Update configuration
    pub fn update_config(&self, config: AppConfig) {
        let mut state = self.inner.lock().unwrap();
        state.config = config;
    }

    /// Get API keys
    pub fn get_api_keys(&self) -> ApiKeys {
        let state = self.inner.lock().unwrap();
        state.api_keys.clone()
    }

    /// Update API keys
    pub fn update_api_keys(&self, api_keys: ApiKeys) {
        let mut state = self.inner.lock().unwrap();
        state.api_keys = api_keys;
    }

    /// Add message to conversation
    pub fn add_message(&self, role: MessageRole, content: String) {
        let mut state = self.inner.lock().unwrap();
        let now = current_timestamp();

        let message = Message {
            role,
            content,
            timestamp: now,
        };

        state.conversation.messages.push(message);
        state.conversation.updated_at = now;

        // Trim old messages if exceeding max
        let max = state.conversation.max_messages;
        if state.conversation.messages.len() > max {
            let start = state.conversation.messages.len() - max;
            state.conversation.messages = state.conversation.messages[start..].to_vec();
        }

        log::debug!("Message added. Total messages: {}", state.conversation.messages.len());
    }

    /// Get conversation history
    pub fn get_conversation(&self) -> ConversationContext {
        let state = self.inner.lock().unwrap();
        state.conversation.clone()
    }

    /// Clear conversation history
    pub fn clear_conversation(&self) {
        let mut state = self.inner.lock().unwrap();
        let now = current_timestamp();
        state.conversation = ConversationContext {
            id: generate_id(),
            messages: Vec::new(),
            max_messages: state.conversation.max_messages,
            started_at: now,
            updated_at: now,
        };
        log::info!("Conversation cleared");
    }

    /// Get messages for API context (formatted for LLM)
    pub fn get_api_messages(&self) -> Vec<(String, String)> {
        let state = self.inner.lock().unwrap();
        state.conversation.messages
            .iter()
            .map(|m| {
                let role = match m.role {
                    MessageRole::User => "user".to_string(),
                    MessageRole::Assistant => "assistant".to_string(),
                    MessageRole::System => "system".to_string(),
                };
                (role, m.content.clone())
            })
            .collect()
    }

    /// Update connectivity status
    pub fn update_connectivity(&self, connectivity: ConnectivityStatus) {
        let mut state = self.inner.lock().unwrap();
        state.connectivity = connectivity;
    }

    /// Get connectivity status
    pub fn get_connectivity(&self) -> ConnectivityStatus {
        let state = self.inner.lock().unwrap();
        state.connectivity.clone()
    }

    /// Update individual service status
    pub fn update_service_status(&self, service: &str, status: ServiceStatus) {
        let mut state = self.inner.lock().unwrap();
        match service {
            "whisper" => state.connectivity.whisper = status,
            "openwebui" => state.connectivity.openwebui = status,
            "elevenlabs" => state.connectivity.elevenlabs = status,
            _ => log::warn!("Unknown service: {}", service),
        }
        state.connectivity.last_checked = current_timestamp();
    }

    /// Check if all services are connected
    pub fn all_services_connected(&self) -> bool {
        let state = self.inner.lock().unwrap();
        matches!(state.connectivity.whisper, ServiceStatus::Connected)
            && matches!(state.connectivity.openwebui, ServiceStatus::Connected)
            && matches!(state.connectivity.elevenlabs, ServiceStatus::Connected)
    }
}

/// Get current Unix timestamp in seconds
fn current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

/// Generate a unique ID
fn generate_id() -> String {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let id: u64 = rng.gen();
    format!("{:x}", id)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_state_creation() {
        let config = AppConfig::default();
        let keys = ApiKeys {
            whisper: None,
            openwebui: None,
            elevenlabs: None,
        };
        let state = AppState::new(config, keys);
        assert_eq!(state.get_status(), AppStatus::Idle);
    }

    #[test]
    fn test_status_change() {
        let config = AppConfig::default();
        let keys = ApiKeys {
            whisper: None,
            openwebui: None,
            elevenlabs: None,
        };
        let state = AppState::new(config, keys);
        state.set_status(AppStatus::Listening);
        assert_eq!(state.get_status(), AppStatus::Listening);
    }

    #[test]
    fn test_message_addition() {
        let config = AppConfig::default();
        let keys = ApiKeys {
            whisper: None,
            openwebui: None,
            elevenlabs: None,
        };
        let state = AppState::new(config, keys);
        state.add_message(MessageRole::User, "Hello".to_string());
        let conversation = state.get_conversation();
        assert_eq!(conversation.messages.len(), 1);
        assert_eq!(conversation.messages[0].content, "Hello");
    }

    #[test]
    fn test_conversation_clearing() {
        let config = AppConfig::default();
        let keys = ApiKeys {
            whisper: None,
            openwebui: None,
            elevenlabs: None,
        };
        let state = AppState::new(config, keys);
        state.add_message(MessageRole::User, "Hello".to_string());
        state.clear_conversation();
        let conversation = state.get_conversation();
        assert_eq!(conversation.messages.len(), 0);
    }

    #[test]
    fn test_max_messages() {
        let config = AppConfig::default();
        let keys = ApiKeys {
            whisper: None,
            openwebui: None,
            elevenlabs: None,
        };
        let state = AppState::new(config, keys);

        // Add more messages than max
        for i in 0..25 {
            state.add_message(MessageRole::User, format!("Message {}", i));
        }

        let conversation = state.get_conversation();
        assert_eq!(conversation.messages.len(), 20); // Should be trimmed to max_messages
    }
}
