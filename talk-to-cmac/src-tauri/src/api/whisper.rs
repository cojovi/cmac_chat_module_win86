//! Whisper API client for speech-to-text transcription
//!
//! Handles audio file upload and transcription using OpenAI's Whisper API
//! or compatible endpoints with retry logic and timeout support.

use crate::config::WhisperConfig;
use crate::error::{AppResult, WhisperError};
use reqwest::multipart::{Form, Part};
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Whisper API client
pub struct WhisperClient {
    client: reqwest::Client,
    config: WhisperConfig,
    api_key: Option<String>,
}

/// Whisper API transcription response
#[derive(Debug, Deserialize, Serialize)]
pub struct TranscriptionResponse {
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<f32>,
}

/// Whisper API error response
#[derive(Debug, Deserialize)]
struct WhisperErrorResponse {
    error: WhisperErrorDetail,
}

#[derive(Debug, Deserialize)]
struct WhisperErrorDetail {
    message: String,
    #[serde(rename = "type")]
    error_type: Option<String>,
}

impl WhisperClient {
    /// Create a new Whisper client
    pub fn new(config: WhisperConfig, api_key: Option<String>) -> AppResult<Self> {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(config.timeout_secs))
            .build()
            .map_err(|e| WhisperError::TranscriptionFailed(e.to_string()))?;

        Ok(Self {
            client,
            config,
            api_key,
        })
    }

    /// Transcribe audio from bytes
    ///
    /// # Arguments
    /// * `audio_data` - Raw audio bytes (should be 16kHz WAV format)
    /// * `filename` - Filename for the upload
    ///
    /// # Returns
    /// Transcribed text from the audio
    pub async fn transcribe_audio(
        &self,
        audio_data: Vec<u8>,
        filename: &str,
    ) -> AppResult<String> {
        // Validate audio size (25MB limit)
        const MAX_SIZE: usize = 25 * 1024 * 1024;
        if audio_data.len() > MAX_SIZE {
            return Err(WhisperError::AudioFileTooLarge.into());
        }

        log::info!("Transcribing audio file: {} ({} bytes)", filename, audio_data.len());

        // Attempt transcription with retry logic
        let max_retries = 3;
        let mut last_error = None;

        for attempt in 1..=max_retries {
            match self.try_transcribe(&audio_data, filename).await {
                Ok(result) => {
                    log::info!("Transcription successful: '{}'", result.text);
                    return Ok(result.text);
                }
                Err(e) => {
                    last_error = Some(e);
                    if attempt < max_retries {
                        let delay = Duration::from_secs(2u64.pow(attempt as u32 - 1));
                        log::warn!("Transcription attempt {} failed, retrying in {:?}", attempt, delay);
                        tokio::time::sleep(delay).await;
                    }
                }
            }
        }

        Err(last_error.unwrap())
    }

    /// Internal transcription attempt
    async fn try_transcribe(
        &self,
        audio_data: &[u8],
        filename: &str,
    ) -> AppResult<TranscriptionResponse> {
        // Create multipart form
        let audio_part = Part::bytes(audio_data.to_vec())
            .file_name(filename.to_string())
            .mime_str("audio/wav")
            .map_err(|e| WhisperError::TranscriptionFailed(e.to_string()))?;

        let mut form = Form::new()
            .part("file", audio_part)
            .text("model", self.config.model.clone())
            .text("response_format", "json");

        // Add optional parameters
        if let Some(language) = &self.config.language {
            form = form.text("language", language.clone());
        }

        if self.config.temperature > 0.0 {
            form = form.text("temperature", self.config.temperature.to_string());
        }

        // Build request
        let mut request = self.client
            .post(&self.config.endpoint)
            .multipart(form);

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
                    WhisperError::Timeout
                } else {
                    WhisperError::TranscriptionFailed(e.to_string())
                }
            })?;

        // Check status
        let status = response.status();
        if !status.is_success() {
            // Try to parse error response
            if let Ok(error_response) = response.json::<WhisperErrorResponse>().await {
                return Err(match status.as_u16() {
                    401 | 403 => WhisperError::AuthenticationFailed,
                    429 => WhisperError::RateLimitExceeded,
                    _ => WhisperError::TranscriptionFailed(error_response.error.message),
                }.into());
            }
            return Err(WhisperError::TranscriptionFailed(format!("HTTP {}", status)).into());
        }

        // Parse successful response
        let result = response
            .json::<TranscriptionResponse>()
            .await
            .map_err(|e| WhisperError::TranscriptionFailed(e.to_string()))?;

        // Validate response
        if result.text.trim().is_empty() {
            return Err(WhisperError::EmptyResponse.into());
        }

        Ok(result)
    }

    /// Check connectivity to Whisper API
    pub async fn check_connectivity(&self) -> AppResult<bool> {
        // Simple connectivity check - try to make a minimal request
        // This is a basic implementation; you might want to use a dedicated health endpoint
        let response = self.client
            .get(&self.config.endpoint)
            .timeout(Duration::from_secs(5))
            .send()
            .await;

        match response {
            Ok(resp) => {
                // Even 404 is fine - it means the server is reachable
                Ok(resp.status().as_u16() < 500)
            }
            Err(e) => {
                log::warn!("Whisper connectivity check failed: {}", e);
                Ok(false)
            }
        }
    }

    /// Update configuration
    pub fn update_config(&mut self, config: WhisperConfig) {
        self.config = config;
    }

    /// Update API key
    pub fn update_api_key(&mut self, api_key: Option<String>) {
        self.api_key = api_key;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_whisper_client_creation() {
        let config = WhisperConfig {
            endpoint: "https://api.openai.com/v1/audio/transcriptions".to_string(),
            model: "whisper-1".to_string(),
            language: Some("en".to_string()),
            temperature: 0.0,
            timeout_secs: 30,
        };

        let client = WhisperClient::new(config, None);
        assert!(client.is_ok());
    }

    #[test]
    fn test_audio_size_validation() {
        // Test that files over 25MB are rejected
        let config = WhisperConfig {
            endpoint: "https://api.openai.com/v1/audio/transcriptions".to_string(),
            model: "whisper-1".to_string(),
            language: Some("en".to_string()),
            temperature: 0.0,
            timeout_secs: 30,
        };

        let client = WhisperClient::new(config, None).unwrap();
        let large_audio = vec![0u8; 26 * 1024 * 1024]; // 26MB

        let runtime = tokio::runtime::Runtime::new().unwrap();
        let result = runtime.block_on(client.transcribe_audio(large_audio, "test.wav"));

        assert!(result.is_err());
    }
}
