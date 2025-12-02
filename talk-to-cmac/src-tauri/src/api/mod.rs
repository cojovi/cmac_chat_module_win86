//! API clients module
//!
//! Contains clients for interacting with external services:
//! - Whisper: Speech-to-text transcription
//! - OpenWebUI: LLM interaction
//! - ElevenLabs: Text-to-speech synthesis

pub mod whisper;
pub mod openwebui;
pub mod elevenlabs;

// Re-export for convenience
pub use whisper::WhisperClient;
pub use openwebui::OpenWebUiClient;
pub use elevenlabs::ElevenLabsClient;
