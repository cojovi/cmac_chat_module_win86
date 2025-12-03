//! Configuration management for Talk to CMAC
//!
//! Handles loading, saving, and managing application configuration including
//! API endpoints, preferences, and secure storage of API keys using the system keyring.

use crate::error::{AppResult, AppError, ConfigError};
use keyring::Entry;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

/// Application configuration structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    /// Whisper API configuration
    pub whisper: WhisperConfig,

    /// OpenWebUI configuration
    pub openwebui: OpenWebUiConfig,

    /// ElevenLabs configuration
    pub elevenlabs: ElevenLabsConfig,

    /// Audio preferences
    pub audio: AudioConfig,

    /// UI preferences
    pub ui: UiConfig,
}

/// Whisper API configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhisperConfig {
    /// Whisper API endpoint
    pub endpoint: String,

    /// Model to use (e.g., "whisper-1")
    pub model: String,

    /// Language code (optional, e.g., "en")
    pub language: Option<String>,

    /// Temperature for sampling
    pub temperature: f32,

    /// Timeout in seconds
    pub timeout_secs: u64,
}

/// OpenWebUI configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenWebUiConfig {
    /// OpenWebUI API endpoint
    pub endpoint: String,

    /// Model to use
    pub model: String,

    /// Maximum context length
    pub max_context_length: usize,

    /// Temperature for generation
    pub temperature: f32,

    /// Maximum tokens to generate
    pub max_tokens: Option<usize>,

    /// Enable streaming responses
    pub stream: bool,

    /// Timeout in seconds
    pub timeout_secs: u64,
}

/// ElevenLabs configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElevenLabsConfig {
    /// ElevenLabs API endpoint
    pub endpoint: String,

    /// Voice ID to use
    pub voice_id: String,

    /// Model ID (e.g., "eleven_monolingual_v1")
    pub model_id: String,

    /// Voice settings
    pub voice_settings: VoiceSettings,

    /// Timeout in seconds
    pub timeout_secs: u64,
}

/// Voice synthesis settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceSettings {
    /// Stability (0.0-1.0)
    pub stability: f32,

    /// Similarity boost (0.0-1.0)
    pub similarity_boost: f32,

    /// Style (0.0-1.0)
    pub style: Option<f32>,

    /// Use speaker boost
    pub use_speaker_boost: bool,
}

/// Audio configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioConfig {
    /// Sample rate for recording (Hz)
    pub sample_rate: u32,

    /// Bit depth
    pub bit_depth: u16,

    /// Number of channels (1 = mono, 2 = stereo)
    pub channels: u16,

    /// Recording format
    pub format: String,

    /// Silence detection threshold (0.0-1.0)
    pub silence_threshold: f32,

    /// Silence duration before stopping (seconds)
    pub silence_duration: f32,

    /// Maximum recording duration (seconds)
    pub max_duration: u32,
}

/// UI preferences
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiConfig {
    /// Theme ("light" or "dark")
    pub theme: String,

    /// Show transcription in real-time
    pub show_transcription: bool,

    /// Show thinking indicator
    pub show_thinking: bool,

    /// Auto-minimize after response
    pub auto_minimize: bool,

    /// Window always on top
    pub always_on_top: bool,

    /// Global hotkey
    pub global_hotkey: Option<String>,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            whisper: WhisperConfig {
                endpoint: std::env::var("WHISPER_BASE_URL")
                    .map(|url| format!("{}/audio/transcriptions", url))
                    .unwrap_or_else(|_| "https://api.openai.com/v1/audio/transcriptions".to_string()),
                model: std::env::var("WHISPER_MODEL")
                    .unwrap_or_else(|_| "whisper-1".to_string()),
                language: Some("en".to_string()),
                temperature: 0.0,
                timeout_secs: 30,
            },
            openwebui: OpenWebUiConfig {
                endpoint: std::env::var("OPENWEBUI_BASE_URL")
                    .map(|url| format!("{}/api/chat", url))
                    .unwrap_or_else(|_| "http://localhost:3000/api/chat".to_string()),
                model: std::env::var("OPENWEBUI_MODEL_NAME")
                    .unwrap_or_else(|_| "llama3.2".to_string()),
                max_context_length: 4096,
                temperature: 0.7,
                max_tokens: Some(1024),
                stream: false,
                timeout_secs: 60,
            },
            elevenlabs: ElevenLabsConfig {
                endpoint: std::env::var("ELEVENLABS_BASE_URL")
                    .map(|url| format!("{}/text-to-speech", url))
                    .unwrap_or_else(|_| "https://api.elevenlabs.io/v1/text-to-speech".to_string()),
                voice_id: std::env::var("ELEVENLABS_VOICE_ID")
                    .unwrap_or_else(|_| "21m00Tcm4TlvDq8ikWAM".to_string()), // Default voice
                model_id: "eleven_monolingual_v1".to_string(),
                voice_settings: VoiceSettings {
                    stability: 0.5,
                    similarity_boost: 0.75,
                    style: Some(0.0),
                    use_speaker_boost: true,
                },
                timeout_secs: 30,
            },
            audio: AudioConfig {
                sample_rate: 16000,
                bit_depth: 16,
                channels: 1,
                format: "wav".to_string(),
                silence_threshold: 0.01,
                silence_duration: 2.0,
                max_duration: 300,
            },
            ui: UiConfig {
                theme: "dark".to_string(),
                show_transcription: true,
                show_thinking: true,
                auto_minimize: false,
                always_on_top: true,
                global_hotkey: Some("CommandOrControl+Shift+Space".to_string()),
            },
        }
    }
}

/// Configuration manager with secure key storage
pub struct ConfigManager {
    config_path: PathBuf,
    keyring_service: String,
}

impl ConfigManager {
    /// Create a new configuration manager
    pub fn new() -> AppResult<Self> {
        let config_dir = Self::get_config_dir()?;
        fs::create_dir_all(&config_dir)
            .map_err(|e| ConfigError::SaveFailed(e.to_string()))?;

        Ok(Self {
            config_path: config_dir.join("config.json"),
            keyring_service: "com.cmac.talk-to-cmac".to_string(),
        })
    }

    /// Get the configuration directory
    fn get_config_dir() -> AppResult<PathBuf> {
        #[cfg(target_os = "macos")]
        {
            let home = std::env::var("HOME")
                .map_err(|_| ConfigError::MissingConfig("HOME environment variable".to_string()))?;
            Ok(PathBuf::from(home).join("Library/Application Support/com.cmac.talk-to-cmac"))
        }

        #[cfg(target_os = "windows")]
        {
            let appdata = std::env::var("APPDATA")
                .map_err(|_| ConfigError::MissingConfig("APPDATA environment variable".to_string()))?;
            Ok(PathBuf::from(appdata).join("TalkToCMAC"))
        }

        #[cfg(target_os = "linux")]
        {
            let home = std::env::var("HOME")
                .map_err(|_| ConfigError::MissingConfig("HOME environment variable".to_string()))?;
            Ok(PathBuf::from(home).join(".config/talk-to-cmac"))
        }
    }

    /// Load configuration from disk
    pub fn load(&self) -> AppResult<AppConfig> {
        if !self.config_path.exists() {
            log::info!("Config file not found, using defaults");
            return Ok(AppConfig::default());
        }

        let contents = fs::read_to_string(&self.config_path)
            .map_err(|e| ConfigError::LoadFailed(e.to_string()))?;

        let config: AppConfig = serde_json::from_str(&contents)
            .map_err(|e| ConfigError::ParseError(e.to_string()))?;

        log::info!("Configuration loaded successfully");
        Ok(config)
    }

    /// Save configuration to disk
    pub fn save(&self, config: &AppConfig) -> AppResult<()> {
        let contents = serde_json::to_string_pretty(config)
            .map_err(|e| ConfigError::SaveFailed(e.to_string()))?;

        fs::write(&self.config_path, contents)
            .map_err(|e| ConfigError::SaveFailed(e.to_string()))?;

        log::info!("Configuration saved successfully");
        Ok(())
    }

    /// Store API key securely in system keyring
    pub fn store_api_key(&self, service: &str, api_key: &str) -> AppResult<()> {
        let entry = Entry::new(&self.keyring_service, service)
            .map_err(|e| ConfigError::KeyringError(e.to_string()))?;

        entry.set_password(api_key)
            .map_err(|e| ConfigError::KeyringError(e.to_string()))?;

        log::info!("API key stored for service: {}", service);
        Ok(())
    }

    /// Retrieve API key from system keyring
    pub fn get_api_key(&self, service: &str) -> AppResult<String> {
        // PRIORITY 1: Check standard environment variables (from .env file or system)
        let env_var_name = format!("{}_API_KEY", service.to_uppercase().replace('-', "_"));
        if let Ok(key) = std::env::var(&env_var_name) {
            if !key.is_empty() && key != "your-openai-api-key-here" && key != "your-openwebui-api-key-here" && key != "your-elevenlabs-api-key-here" {
                log::info!("✓ Using {} from environment variable: {}", service, env_var_name);
                return Ok(key);
            }
        }

        // PRIORITY 2: Check alternative environment variable names
        let alt_names = match service {
            "whisper" => vec!["OPENAI_API_KEY"],
            "openwebui" => vec!["OPENWEBUI_API_KEY"],
            "elevenlabs" => vec!["ELEVENLABS_API_KEY"],
            _ => vec![],
        };

        for alt_name in alt_names {
            if let Ok(key) = std::env::var(alt_name) {
                if !key.is_empty() && !key.contains("your-") && !key.contains("-api-key-here") {
                    log::info!("✓ Using {} from environment variable: {}", service, alt_name);
                    return Ok(key);
                }
            }
        }

        // PRIORITY 3: Try keyring
        let entry = Entry::new(&self.keyring_service, service)
            .map_err(|e| ConfigError::KeyringError(e.to_string()))?;

        match entry.get_password() {
            Ok(key) => {
                log::info!("✓ Using {} from system keyring", service);
                Ok(key)
            }
            Err(e) => {
                log::error!("✗ No API key found for {} (tried env vars and keyring): {}", service, e);
                Err(AppError::Config(ConfigError::MissingConfig(format!(
                    "API key not found for '{}'. Set {} environment variable or store via app settings.",
                    service,
                    env_var_name
                ))))
            }
        }
    }

    /// Delete API key from system keyring
    pub fn delete_api_key(&self, service: &str) -> AppResult<()> {
        let entry = Entry::new(&self.keyring_service, service)
            .map_err(|e| ConfigError::KeyringError(e.to_string()))?;

        entry.delete_credential()
            .map_err(|e| ConfigError::KeyringError(e.to_string()))?;

        log::info!("API key deleted for service: {}", service);
        Ok(())
    }

    /// Load configuration with API keys
    pub fn load_with_keys(&self) -> AppResult<(AppConfig, ApiKeys)> {
        let config = self.load()?;
        let keys = ApiKeys {
            whisper: self.get_api_key("whisper").ok(),
            openwebui: self.get_api_key("openwebui").ok(),
            elevenlabs: self.get_api_key("elevenlabs").ok(),
        };
        Ok((config, keys))
    }
}

/// API keys structure (not stored in config file)
#[derive(Debug, Clone)]
pub struct ApiKeys {
    pub whisper: Option<String>,
    pub openwebui: Option<String>,
    pub elevenlabs: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = AppConfig::default();
        assert_eq!(config.audio.sample_rate, 16000);
        assert_eq!(config.ui.theme, "dark");
    }

    #[test]
    fn test_config_serialization() {
        let config = AppConfig::default();
        let json = serde_json::to_string(&config).unwrap();
        let deserialized: AppConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(config.whisper.model, deserialized.whisper.model);
    }
}
