//! ElevenLabs API client for text-to-speech synthesis
//!
//! Handles text-to-speech conversion using ElevenLabs API with voice selection,
//! voice settings customization, and proper error handling.

use crate::config::{ElevenLabsConfig, VoiceSettings};
use crate::error::{AppResult, ElevenLabsError};
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// ElevenLabs API client
pub struct ElevenLabsClient {
    client: reqwest::Client,
    config: ElevenLabsConfig,
    api_key: Option<String>,
}

/// Text-to-speech request
#[derive(Debug, Serialize)]
struct TtsRequest {
    text: String,
    model_id: String,
    voice_settings: VoiceSettingsRequest,
}

/// Voice settings for request
#[derive(Debug, Serialize)]
struct VoiceSettingsRequest {
    stability: f32,
    similarity_boost: f32,
    #[serde(skip_serializing_if = "Option::is_none")]
    style: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    use_speaker_boost: Option<bool>,
}

/// ElevenLabs error response
#[derive(Debug, Deserialize)]
struct ElevenLabsErrorResponse {
    detail: ElevenLabsErrorDetail,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum ElevenLabsErrorDetail {
    String(String),
    Object { message: String },
}

/// Voice information
#[derive(Debug, Deserialize, Serialize)]
pub struct Voice {
    pub voice_id: String,
    pub name: String,
    pub category: Option<String>,
    pub labels: Option<std::collections::HashMap<String, String>>,
}

/// Voices list response
#[derive(Debug, Deserialize)]
struct VoicesResponse {
    voices: Vec<Voice>,
}

impl ElevenLabsClient {
    /// Create a new ElevenLabs client
    pub fn new(config: ElevenLabsConfig, api_key: Option<String>) -> AppResult<Self> {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(config.timeout_secs))
            .build()
            .map_err(|e| ElevenLabsError::SynthesisFailed(e.to_string()))?;

        Ok(Self {
            client,
            config,
            api_key,
        })
    }

    /// Synthesize speech from text
    ///
    /// # Arguments
    /// * `text` - The text to convert to speech
    ///
    /// # Returns
    /// Audio bytes in MP3 format
    pub async fn synthesize_speech(&self, text: &str) -> AppResult<Vec<u8>> {
        // Validate text length (character limit varies by plan, typically ~5000)
        const MAX_CHARS: usize = 5000;
        if text.len() > MAX_CHARS {
            return Err(ElevenLabsError::CharacterLimitExceeded.into());
        }

        log::info!("Synthesizing speech for text ({} chars)", text.len());

        // Attempt synthesis with retry logic
        let max_retries = 3;
        let mut last_error = None;

        for attempt in 1..=max_retries {
            match self.try_synthesize_speech(text).await {
                Ok(audio_data) => {
                    log::info!("Speech synthesis successful ({} bytes)", audio_data.len());
                    return Ok(audio_data);
                }
                Err(e) => {
                    last_error = Some(e);
                    if attempt < max_retries {
                        let delay = Duration::from_secs(2u64.pow(attempt as u32 - 1));
                        log::warn!("Synthesis attempt {} failed, retrying in {:?}", attempt, delay);
                        tokio::time::sleep(delay).await;
                    }
                }
            }
        }

        Err(last_error.unwrap())
    }

    /// Internal synthesis attempt
    async fn try_synthesize_speech(&self, text: &str) -> AppResult<Vec<u8>> {
        // Check if API key is available
        if self.api_key.is_none() {
            return Err(ElevenLabsError::AuthenticationFailed.into());
        }

        // Build request payload
        let request_body = TtsRequest {
            text: text.to_string(),
            model_id: self.config.model_id.clone(),
            voice_settings: VoiceSettingsRequest {
                stability: self.config.voice_settings.stability,
                similarity_boost: self.config.voice_settings.similarity_boost,
                style: self.config.voice_settings.style,
                use_speaker_boost: Some(self.config.voice_settings.use_speaker_boost),
            },
        };

        // Build endpoint URL
        let endpoint = format!(
            "{}/{}",
            self.config.endpoint.trim_end_matches('/'),
            self.config.voice_id
        );

        log::debug!("Synthesis request: voice={}, model={}, length={}",
                   self.config.voice_id, self.config.model_id, text.len());

        // Build HTTP request
        let response = self.client
            .post(&endpoint)
            .header("xi-api-key", self.api_key.as_ref().unwrap())
            .json(&request_body)
            .send()
            .await
            .map_err(|e| {
                if e.is_timeout() {
                    ElevenLabsError::Timeout
                } else {
                    ElevenLabsError::SynthesisFailed(e.to_string())
                }
            })?;

        // Check status
        let status = response.status();
        if !status.is_success() {
            // Try to parse error response
            let error_text = response.text().await.unwrap_or_default();

            return Err(match status.as_u16() {
                401 | 403 => ElevenLabsError::AuthenticationFailed,
                404 => ElevenLabsError::VoiceNotFound(self.config.voice_id.clone()),
                429 => ElevenLabsError::RateLimitExceeded,
                402 => ElevenLabsError::QuotaExceeded,
                _ => {
                    // Try to parse structured error
                    if let Ok(error_response) = serde_json::from_str::<ElevenLabsErrorResponse>(&error_text) {
                        let message = match error_response.detail {
                            ElevenLabsErrorDetail::String(s) => s,
                            ElevenLabsErrorDetail::Object { message } => message,
                        };
                        ElevenLabsError::SynthesisFailed(message)
                    } else {
                        ElevenLabsError::SynthesisFailed(format!("HTTP {}: {}", status, error_text))
                    }
                }
            }.into());
        }

        // Get audio bytes
        let audio_bytes = response
            .bytes()
            .await
            .map_err(|e| ElevenLabsError::SynthesisFailed(e.to_string()))?;

        Ok(audio_bytes.to_vec())
    }

    /// List available voices
    pub async fn list_voices(&self) -> AppResult<Vec<Voice>> {
        if self.api_key.is_none() {
            return Err(ElevenLabsError::AuthenticationFailed.into());
        }

        let voices_endpoint = format!(
            "{}/voices",
            self.config.endpoint.trim_end_matches('/').replace("/text-to-speech", "")
        );

        log::debug!("Fetching voices from: {}", voices_endpoint);

        let response = self.client
            .get(&voices_endpoint)
            .header("xi-api-key", self.api_key.as_ref().unwrap())
            .send()
            .await
            .map_err(|e| ElevenLabsError::SynthesisFailed(e.to_string()))?;

        if !response.status().is_success() {
            return Err(ElevenLabsError::SynthesisFailed(
                format!("Failed to fetch voices: HTTP {}", response.status())
            ).into());
        }

        let voices_response = response
            .json::<VoicesResponse>()
            .await
            .map_err(|e| ElevenLabsError::SynthesisFailed(e.to_string()))?;

        Ok(voices_response.voices)
    }

    /// Check connectivity to ElevenLabs API
    pub async fn check_connectivity(&self) -> AppResult<bool> {
        if self.api_key.is_none() {
            return Ok(false);
        }

        // Try to list voices as a connectivity check
        let voices_endpoint = format!(
            "{}/voices",
            self.config.endpoint.trim_end_matches('/').replace("/text-to-speech", "")
        );

        let response = self.client
            .get(&voices_endpoint)
            .header("xi-api-key", self.api_key.as_ref().unwrap())
            .timeout(Duration::from_secs(5))
            .send()
            .await;

        match response {
            Ok(resp) => {
                let status = resp.status().as_u16();
                // Consider 2xx and 401 (auth issue but service is up) as "connected"
                Ok(status < 500)
            }
            Err(e) => {
                log::warn!("ElevenLabs connectivity check failed: {}", e);
                Ok(false)
            }
        }
    }

    /// Update configuration
    pub fn update_config(&mut self, config: ElevenLabsConfig) {
        self.config = config;
    }

    /// Update API key
    pub fn update_api_key(&mut self, api_key: Option<String>) {
        self.api_key = api_key;
    }

    /// Get current voice ID
    pub fn get_voice_id(&self) -> &str {
        &self.config.voice_id
    }

    /// Set voice ID
    pub fn set_voice_id(&mut self, voice_id: String) {
        self.config.voice_id = voice_id;
    }

    /// Update voice settings
    pub fn update_voice_settings(&mut self, settings: VoiceSettings) {
        self.config.voice_settings = settings;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_elevenlabs_client_creation() {
        let config = ElevenLabsConfig {
            endpoint: "https://api.elevenlabs.io/v1/text-to-speech".to_string(),
            voice_id: "21m00Tcm4TlvDq8ikWAM".to_string(),
            model_id: "eleven_monolingual_v1".to_string(),
            voice_settings: VoiceSettings {
                stability: 0.5,
                similarity_boost: 0.75,
                style: Some(0.0),
                use_speaker_boost: true,
            },
            timeout_secs: 30,
        };

        let client = ElevenLabsClient::new(config, None);
        assert!(client.is_ok());
    }

    #[test]
    fn test_character_limit_validation() {
        let config = ElevenLabsConfig {
            endpoint: "https://api.elevenlabs.io/v1/text-to-speech".to_string(),
            voice_id: "21m00Tcm4TlvDq8ikWAM".to_string(),
            model_id: "eleven_monolingual_v1".to_string(),
            voice_settings: VoiceSettings {
                stability: 0.5,
                similarity_boost: 0.75,
                style: Some(0.0),
                use_speaker_boost: true,
            },
            timeout_secs: 30,
        };

        let client = ElevenLabsClient::new(config, Some("test_key".to_string())).unwrap();
        let long_text = "a".repeat(6000);

        let runtime = tokio::runtime::Runtime::new().unwrap();
        let result = runtime.block_on(client.synthesize_speech(&long_text));

        assert!(result.is_err());
    }

    #[test]
    fn test_voice_settings_serialization() {
        let settings = VoiceSettingsRequest {
            stability: 0.5,
            similarity_boost: 0.75,
            style: Some(0.0),
            use_speaker_boost: Some(true),
        };

        let json = serde_json::to_string(&settings).unwrap();
        assert!(json.contains("stability"));
        assert!(json.contains("0.5"));
    }
}
