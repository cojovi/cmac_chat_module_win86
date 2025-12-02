//! Tauri commands for frontend interaction
//!
//! Defines all commands that can be invoked from the frontend, handling
//! the complete voice assistant pipeline and configuration management.

use crate::api::{ElevenLabsClient, OpenWebUiClient, WhisperClient};
use crate::config::{AppConfig, ConfigManager, VoiceSettings};
use crate::error::AppResult;
use crate::state::{AppState, AppStatus, MessageRole, ServiceStatus};
use serde::{Deserialize, Serialize};
use tauri::State;

/// Process audio file and return transcription
#[tauri::command]
pub async fn process_audio(
    audio_data: Vec<u8>,
    filename: String,
    state: State<'_, AppState>,
) -> Result<String, String> {
    log::info!("Processing audio: {} bytes", audio_data.len());

    // Update status
    state.set_status(AppStatus::Transcribing);

    // Get configuration and API keys
    let config = state.get_config();
    let api_keys = state.get_api_keys();

    // Create Whisper client
    let whisper_client = WhisperClient::new(config.whisper, api_keys.whisper)
        .map_err(|e| e.to_string())?;

    // Transcribe audio
    let result = whisper_client
        .transcribe_audio(audio_data, &filename)
        .await;

    // Reset status
    state.set_status(AppStatus::Idle);

    match result {
        Ok(text) => {
            log::info!("Transcription successful: '{}'", text);
            Ok(text)
        }
        Err(e) => {
            log::error!("Transcription failed: {}", e);
            state.set_status(AppStatus::Error {
                message: e.to_string(),
            });
            Err(e.to_string())
        }
    }
}

/// Send a text message to the LLM and get response
#[tauri::command]
pub async fn send_message(
    message: String,
    state: State<'_, AppState>,
) -> Result<String, String> {
    log::info!("Sending message to LLM: '{}'", message);

    // Update status
    state.set_status(AppStatus::Thinking);

    // Add user message to conversation
    state.add_message(MessageRole::User, message.clone());

    // Get configuration and API keys
    let config = state.get_config();
    let api_keys = state.get_api_keys();

    // Create OpenWebUI client
    let openwebui_client = OpenWebUiClient::new(config.openwebui, api_keys.openwebui)
        .map_err(|e| e.to_string())?;

    // Get conversation context
    let messages = state.get_api_messages();

    // Send message
    let result = openwebui_client.send_message(messages).await;

    // Reset status
    state.set_status(AppStatus::Idle);

    match result {
        Ok(response) => {
            log::info!("LLM response received: {} chars", response.len());
            // Add assistant response to conversation
            state.add_message(MessageRole::Assistant, response.clone());
            Ok(response)
        }
        Err(e) => {
            log::error!("LLM request failed: {}", e);
            state.set_status(AppStatus::Error {
                message: e.to_string(),
            });
            Err(e.to_string())
        }
    }
}

/// Convert text to speech
#[tauri::command]
pub async fn synthesize_speech(
    text: String,
    state: State<'_, AppState>,
) -> Result<Vec<u8>, String> {
    log::info!("Synthesizing speech: {} chars", text.len());

    // Update status
    state.set_status(AppStatus::Speaking);

    // Get configuration and API keys
    let config = state.get_config();
    let api_keys = state.get_api_keys();

    // Create ElevenLabs client
    let elevenlabs_client = ElevenLabsClient::new(config.elevenlabs, api_keys.elevenlabs)
        .map_err(|e| e.to_string())?;

    // Synthesize speech
    let result = elevenlabs_client.synthesize_speech(&text).await;

    // Reset status
    state.set_status(AppStatus::Idle);

    match result {
        Ok(audio_data) => {
            log::info!("Speech synthesis successful: {} bytes", audio_data.len());
            Ok(audio_data)
        }
        Err(e) => {
            log::error!("Speech synthesis failed: {}", e);
            state.set_status(AppStatus::Error {
                message: e.to_string(),
            });
            Err(e.to_string())
        }
    }
}

/// Process complete voice query pipeline: audio -> transcription -> LLM -> TTS
#[tauri::command]
pub async fn process_voice_query(
    audio_data: Vec<u8>,
    filename: String,
    state: State<'_, AppState>,
) -> Result<VoiceQueryResponse, String> {
    log::info!("Processing complete voice query pipeline");

    // Step 1: Transcribe audio
    state.set_status(AppStatus::Transcribing);
    let config = state.get_config();
    let api_keys = state.get_api_keys();

    let whisper_client = WhisperClient::new(config.whisper.clone(), api_keys.whisper.clone())
        .map_err(|e| e.to_string())?;

    let transcription = whisper_client
        .transcribe_audio(audio_data, &filename)
        .await
        .map_err(|e| {
            state.set_status(AppStatus::Error {
                message: e.to_string(),
            });
            e.to_string()
        })?;

    log::info!("Transcription: '{}'", transcription);

    // Step 2: Send to LLM
    state.set_status(AppStatus::Thinking);
    state.add_message(MessageRole::User, transcription.clone());

    let openwebui_client = OpenWebUiClient::new(config.openwebui.clone(), api_keys.openwebui.clone())
        .map_err(|e| e.to_string())?;

    let messages = state.get_api_messages();
    let llm_response = openwebui_client
        .send_message(messages)
        .await
        .map_err(|e| {
            state.set_status(AppStatus::Error {
                message: e.to_string(),
            });
            e.to_string()
        })?;

    log::info!("LLM response: {} chars", llm_response.len());
    state.add_message(MessageRole::Assistant, llm_response.clone());

    // Step 3: Convert to speech
    state.set_status(AppStatus::Speaking);
    let elevenlabs_client = ElevenLabsClient::new(config.elevenlabs, api_keys.elevenlabs)
        .map_err(|e| e.to_string())?;

    let audio_response = elevenlabs_client
        .synthesize_speech(&llm_response)
        .await
        .map_err(|e| {
            state.set_status(AppStatus::Error {
                message: e.to_string(),
            });
            e.to_string()
        })?;

    log::info!("Speech synthesis complete: {} bytes", audio_response.len());

    // Reset status
    state.set_status(AppStatus::Idle);

    Ok(VoiceQueryResponse {
        transcription,
        llm_response,
        audio_response,
    })
}

/// Response structure for voice query
#[derive(Debug, Serialize, Deserialize)]
pub struct VoiceQueryResponse {
    pub transcription: String,
    pub llm_response: String,
    pub audio_response: Vec<u8>,
}

/// Load application configuration
#[tauri::command]
pub async fn load_config(state: State<'_, AppState>) -> Result<AppConfig, String> {
    log::info!("Loading configuration");
    Ok(state.get_config())
}

/// Save application configuration
#[tauri::command]
pub async fn save_config(
    config: AppConfig,
    state: State<'_, AppState>,
) -> Result<(), String> {
    log::info!("Saving configuration");

    // Update state
    state.update_config(config.clone());

    // Persist to disk
    let config_manager = ConfigManager::new().map_err(|e| e.to_string())?;
    config_manager.save(&config).map_err(|e| e.to_string())?;

    log::info!("Configuration saved successfully");
    Ok(())
}

/// Update API key for a service
#[tauri::command]
pub async fn update_api_key(
    service: String,
    api_key: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    log::info!("Updating API key for service: {}", service);

    // Store in keyring
    let config_manager = ConfigManager::new().map_err(|e| e.to_string())?;
    config_manager
        .store_api_key(&service, &api_key)
        .map_err(|e| e.to_string())?;

    // Update state
    let mut api_keys = state.get_api_keys();
    match service.as_str() {
        "whisper" => api_keys.whisper = Some(api_key),
        "openwebui" => api_keys.openwebui = Some(api_key),
        "elevenlabs" => api_keys.elevenlabs = Some(api_key),
        _ => return Err(format!("Unknown service: {}", service)),
    }
    state.update_api_keys(api_keys);

    log::info!("API key updated for {}", service);
    Ok(())
}

/// Check connectivity to all services
#[tauri::command]
pub async fn check_connectivity(state: State<'_, AppState>) -> Result<ConnectivityResponse, String> {
    log::info!("Checking connectivity to all services");

    let config = state.get_config();
    let api_keys = state.get_api_keys();

    // Check Whisper
    state.update_service_status("whisper", ServiceStatus::Checking);
    let whisper_status = match WhisperClient::new(config.whisper.clone(), api_keys.whisper.clone()) {
        Ok(client) => {
            match client.check_connectivity().await {
                Ok(true) => ServiceStatus::Connected,
                Ok(false) => ServiceStatus::Disconnected {
                    reason: "Service unreachable".to_string(),
                },
                Err(e) => ServiceStatus::Disconnected {
                    reason: e.to_string(),
                },
            }
        }
        Err(e) => ServiceStatus::Disconnected {
            reason: e.to_string(),
        },
    };
    state.update_service_status("whisper", whisper_status.clone());

    // Check OpenWebUI
    state.update_service_status("openwebui", ServiceStatus::Checking);
    let openwebui_status = match OpenWebUiClient::new(config.openwebui.clone(), api_keys.openwebui.clone()) {
        Ok(client) => {
            match client.check_connectivity().await {
                Ok(true) => ServiceStatus::Connected,
                Ok(false) => ServiceStatus::Disconnected {
                    reason: "Service unreachable".to_string(),
                },
                Err(e) => ServiceStatus::Disconnected {
                    reason: e.to_string(),
                },
            }
        }
        Err(e) => ServiceStatus::Disconnected {
            reason: e.to_string(),
        },
    };
    state.update_service_status("openwebui", openwebui_status.clone());

    // Check ElevenLabs
    state.update_service_status("elevenlabs", ServiceStatus::Checking);
    let elevenlabs_status = match ElevenLabsClient::new(config.elevenlabs, api_keys.elevenlabs) {
        Ok(client) => {
            match client.check_connectivity().await {
                Ok(true) => ServiceStatus::Connected,
                Ok(false) => ServiceStatus::Disconnected {
                    reason: "Service unreachable".to_string(),
                },
                Err(e) => ServiceStatus::Disconnected {
                    reason: e.to_string(),
                },
            }
        }
        Err(e) => ServiceStatus::Disconnected {
            reason: e.to_string(),
        },
    };
    state.update_service_status("elevenlabs", elevenlabs_status.clone());

    log::info!("Connectivity check complete");

    Ok(ConnectivityResponse {
        whisper: whisper_status,
        openwebui: openwebui_status,
        elevenlabs: elevenlabs_status,
    })
}

/// Connectivity response structure
#[derive(Debug, Serialize, Deserialize)]
pub struct ConnectivityResponse {
    pub whisper: ServiceStatus,
    pub openwebui: ServiceStatus,
    pub elevenlabs: ServiceStatus,
}

/// Get current application state
#[tauri::command]
pub async fn get_app_state(state: State<'_, AppState>) -> Result<AppStateResponse, String> {
    log::debug!("Getting application state");

    let status = state.get_status();
    let conversation = state.get_conversation();
    let connectivity = state.get_connectivity();

    Ok(AppStateResponse {
        status,
        message_count: conversation.messages.len(),
        connectivity,
    })
}

/// Application state response
#[derive(Debug, Serialize, Deserialize)]
pub struct AppStateResponse {
    pub status: AppStatus,
    pub message_count: usize,
    pub connectivity: crate::state::ConnectivityStatus,
}

/// Clear conversation history
#[tauri::command]
pub async fn clear_conversation(state: State<'_, AppState>) -> Result<(), String> {
    log::info!("Clearing conversation history");
    state.clear_conversation();
    Ok(())
}

/// Get conversation history
#[tauri::command]
pub async fn get_conversation(state: State<'_, AppState>) -> Result<crate::state::ConversationContext, String> {
    log::debug!("Getting conversation history");
    Ok(state.get_conversation())
}

/// List available ElevenLabs voices
#[tauri::command]
pub async fn list_voices(state: State<'_, AppState>) -> Result<Vec<crate::api::elevenlabs::Voice>, String> {
    log::info!("Listing available voices");

    let config = state.get_config();
    let api_keys = state.get_api_keys();

    let elevenlabs_client = ElevenLabsClient::new(config.elevenlabs, api_keys.elevenlabs)
        .map_err(|e| e.to_string())?;

    elevenlabs_client
        .list_voices()
        .await
        .map_err(|e| e.to_string())
}

/// Update voice settings
#[tauri::command]
pub async fn update_voice_settings(
    settings: VoiceSettings,
    state: State<'_, AppState>,
) -> Result<(), String> {
    log::info!("Updating voice settings");

    let mut config = state.get_config();
    config.elevenlabs.voice_settings = settings;
    state.update_config(config.clone());

    // Persist to disk
    let config_manager = ConfigManager::new().map_err(|e| e.to_string())?;
    config_manager.save(&config).map_err(|e| e.to_string())?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{ApiKeys, AppConfig};

    #[tokio::test]
    async fn test_load_config() {
        let config = AppConfig::default();
        let api_keys = ApiKeys {
            whisper: None,
            openwebui: None,
            elevenlabs: None,
        };
        let state = AppState::new(config, api_keys);

        let result = load_config(tauri::State::from(&state)).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_clear_conversation() {
        let config = AppConfig::default();
        let api_keys = ApiKeys {
            whisper: None,
            openwebui: None,
            elevenlabs: None,
        };
        let state = AppState::new(config, api_keys);

        state.add_message(MessageRole::User, "Test".to_string());
        assert_eq!(state.get_conversation().messages.len(), 1);

        let result = clear_conversation(tauri::State::from(&state)).await;
        assert!(result.is_ok());
        assert_eq!(state.get_conversation().messages.len(), 0);
    }
}
