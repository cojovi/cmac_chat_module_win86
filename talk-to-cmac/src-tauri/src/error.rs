//! Error types for the Talk to CMAC application
//!
//! This module defines comprehensive error handling for all backend operations
//! including API interactions, configuration management, and audio processing.

use thiserror::Error;

/// Main error type for the application
#[derive(Error, Debug)]
pub enum AppError {
    /// Errors related to Whisper API interactions
    #[error("Whisper API error: {0}")]
    WhisperApi(#[from] WhisperError),

    /// Errors related to OpenWebUI API interactions
    #[error("OpenWebUI API error: {0}")]
    OpenWebUi(#[from] OpenWebUiError),

    /// Errors related to ElevenLabs API interactions
    #[error("ElevenLabs API error: {0}")]
    ElevenLabs(#[from] ElevenLabsError),

    /// Network-related errors
    #[error("Network error: {0}")]
    Network(#[from] NetworkError),

    /// Configuration-related errors
    #[error("Configuration error: {0}")]
    Config(#[from] ConfigError),

    /// Audio processing errors
    #[error("Audio error: {0}")]
    Audio(#[from] AudioError),

    /// State management errors
    #[error("State error: {0}")]
    State(String),

    /// Generic errors
    #[error("Application error: {0}")]
    Generic(String),
}

/// Errors specific to Whisper API operations
#[derive(Error, Debug)]
pub enum WhisperError {
    #[error("Failed to transcribe audio: {0}")]
    TranscriptionFailed(String),

    #[error("Invalid audio format: expected 16kHz WAV")]
    InvalidAudioFormat,

    #[error("Audio file too large: maximum size is 25MB")]
    AudioFileTooLarge,

    #[error("Whisper API authentication failed")]
    AuthenticationFailed,

    #[error("Whisper API returned empty response")]
    EmptyResponse,

    #[error("Whisper API timeout")]
    Timeout,

    #[error("Whisper API rate limit exceeded")]
    RateLimitExceeded,
}

/// Errors specific to OpenWebUI API operations
#[derive(Error, Debug)]
pub enum OpenWebUiError {
    #[error("Failed to send message: {0}")]
    MessageSendFailed(String),

    #[error("Failed to parse response: {0}")]
    ResponseParseFailed(String),

    #[error("OpenWebUI API authentication failed")]
    AuthenticationFailed,

    #[error("Model not found: {0}")]
    ModelNotFound(String),

    #[error("OpenWebUI API timeout")]
    Timeout,

    #[error("Context limit exceeded")]
    ContextLimitExceeded,

    #[error("OpenWebUI API rate limit exceeded")]
    RateLimitExceeded,

    #[error("Streaming not supported")]
    StreamingNotSupported,
}

/// Errors specific to ElevenLabs API operations
#[derive(Error, Debug)]
pub enum ElevenLabsError {
    #[error("Failed to synthesize speech: {0}")]
    SynthesisFailed(String),

    #[error("Voice not found: {0}")]
    VoiceNotFound(String),

    #[error("Invalid voice settings")]
    InvalidVoiceSettings,

    #[error("ElevenLabs API authentication failed")]
    AuthenticationFailed,

    #[error("ElevenLabs API timeout")]
    Timeout,

    #[error("Character limit exceeded")]
    CharacterLimitExceeded,

    #[error("ElevenLabs API rate limit exceeded")]
    RateLimitExceeded,

    #[error("Quota exceeded")]
    QuotaExceeded,
}

/// Network-related errors
#[derive(Error, Debug)]
pub enum NetworkError {
    #[error("HTTP request failed: {0}")]
    RequestFailed(String),

    #[error("Connection timeout")]
    Timeout,

    #[error("No internet connection")]
    NoConnection,

    #[error("DNS resolution failed")]
    DnsResolutionFailed,

    #[error("SSL/TLS error: {0}")]
    TlsError(String),

    #[error("Connection refused")]
    ConnectionRefused,

    #[error("Too many redirects")]
    TooManyRedirects,
}

/// Configuration-related errors
#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Failed to load configuration: {0}")]
    LoadFailed(String),

    #[error("Failed to save configuration: {0}")]
    SaveFailed(String),

    #[error("Missing required configuration: {0}")]
    MissingConfig(String),

    #[error("Invalid configuration value: {0}")]
    InvalidValue(String),

    #[error("Failed to access keyring: {0}")]
    KeyringError(String),

    #[error("Configuration file not found")]
    FileNotFound,

    #[error("Configuration parse error: {0}")]
    ParseError(String),
}

/// Audio processing errors
#[derive(Error, Debug)]
pub enum AudioError {
    #[error("Failed to read audio file: {0}")]
    ReadFailed(String),

    #[error("Failed to write audio file: {0}")]
    WriteFailed(String),

    #[error("Invalid audio format: {0}")]
    InvalidFormat(String),

    #[error("Audio conversion failed: {0}")]
    ConversionFailed(String),

    #[error("Audio device error: {0}")]
    DeviceError(String),

    #[error("Audio buffer overflow")]
    BufferOverflow,

    #[error("Audio buffer underflow")]
    BufferUnderflow,
}

/// Convert AppError to a Tauri-compatible error string
impl serde::Serialize for AppError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

/// Result type alias for convenience
pub type AppResult<T> = Result<T, AppError>;

/// Convert reqwest errors to NetworkError
impl From<reqwest::Error> for NetworkError {
    fn from(err: reqwest::Error) -> Self {
        if err.is_timeout() {
            NetworkError::Timeout
        } else if err.is_connect() {
            NetworkError::ConnectionRefused
        } else if err.is_redirect() {
            NetworkError::TooManyRedirects
        } else {
            NetworkError::RequestFailed(err.to_string())
        }
    }
}

/// Convert keyring errors to ConfigError
impl From<keyring::Error> for ConfigError {
    fn from(err: keyring::Error) -> Self {
        ConfigError::KeyringError(err.to_string())
    }
}

/// Convert std::io::Error to appropriate error types
impl From<std::io::Error> for AudioError {
    fn from(err: std::io::Error) -> Self {
        AudioError::ReadFailed(err.to_string())
    }
}

impl From<std::io::Error> for ConfigError {
    fn from(err: std::io::Error) -> Self {
        ConfigError::LoadFailed(err.to_string())
    }
}

/// Convert serde_json errors to ConfigError
impl From<serde_json::Error> for ConfigError {
    fn from(err: serde_json::Error) -> Self {
        ConfigError::ParseError(err.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let error = AppError::WhisperApi(WhisperError::TranscriptionFailed("test".to_string()));
        assert!(error.to_string().contains("Whisper API error"));
    }

    #[test]
    fn test_error_conversion() {
        let network_error = NetworkError::Timeout;
        let app_error: AppError = network_error.into();
        assert!(matches!(app_error, AppError::Network(_)));
    }
}
