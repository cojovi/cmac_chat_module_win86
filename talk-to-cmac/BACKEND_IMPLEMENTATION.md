# Talk to CMAC - Backend Implementation

## Overview

This document describes the complete Rust backend implementation for the Talk to CMAC voice assistant application.

## Architecture

The backend is organized into the following modules:

```
src-tauri/src/
├── lib.rs           # Main application entry point
├── main.rs          # Binary entry point
├── error.rs         # Comprehensive error handling
├── config.rs        # Configuration management
├── state.rs         # Application state management
├── commands.rs      # Tauri command handlers
└── api/
    ├── mod.rs       # API module exports
    ├── whisper.rs   # Whisper API client
    ├── openwebui.rs # OpenWebUI API client
    └── elevenlabs.rs # ElevenLabs API client
```

## Module Details

### 1. Error Handling (`error.rs`)

Implements comprehensive error types using `thiserror`:

- **AppError**: Main error type that wraps all other errors
- **WhisperError**: Whisper API-specific errors
- **OpenWebUiError**: OpenWebUI API-specific errors
- **ElevenLabsError**: ElevenLabs API-specific errors
- **NetworkError**: Network-related errors
- **ConfigError**: Configuration errors
- **AudioError**: Audio processing errors

Features:
- Automatic error conversion from standard library types
- Serialization support for Tauri commands
- Descriptive error messages

### 2. Configuration Management (`config.rs`)

Manages application configuration with secure API key storage:

**Structures:**
- `AppConfig`: Main configuration container
- `WhisperConfig`: Whisper API settings
- `OpenWebUiConfig`: OpenWebUI settings
- `ElevenLabsConfig`: ElevenLabs settings
- `AudioConfig`: Audio recording preferences
- `UiConfig`: UI preferences

**ConfigManager Features:**
- Load/save configuration to disk (JSON format)
- Secure API key storage using system keyring
- Environment variable support for API keys
- Platform-specific config directories:
  - macOS: `~/Library/Application Support/com.cmac.talk-to-cmac/`
  - Windows: `%APPDATA%/TalkToCMAC/`
  - Linux: `~/.config/talk-to-cmac/`

### 3. Application State (`state.rs`)

Thread-safe state management using `Arc<Mutex<T>>`:

**AppState Features:**
- Current application status (Idle, Recording, Transcribing, Thinking, Speaking, Error)
- Conversation context with message history
- Automatic message trimming (max 20 messages)
- API connectivity status tracking
- Configuration and API key management

**State Flow:**
```
Idle -> Recording -> Transcribing -> Thinking -> Speaking -> Idle
                                                              ↓
                                                            Error
```

### 4. API Clients

#### Whisper Client (`api/whisper.rs`)

Speech-to-text transcription client:

**Features:**
- Multipart file upload (16kHz WAV format)
- File size validation (25MB limit)
- Retry logic with exponential backoff (3 attempts)
- Timeout support (configurable)
- Bearer token authentication
- Connectivity checking

**Main Method:**
```rust
async fn transcribe_audio(
    &self,
    audio_data: Vec<u8>,
    filename: &str,
) -> AppResult<String>
```

#### OpenWebUI Client (`api/openwebui.rs`)

LLM interaction client:

**Features:**
- Conversation context management
- Context length validation
- Temperature and max_tokens configuration
- Model selection support
- Retry logic with exponential backoff
- Bearer token authentication
- Streaming support (placeholder for future implementation)

**Main Method:**
```rust
async fn send_message(
    &self,
    messages: Vec<(String, String)>,
) -> AppResult<String>
```

#### ElevenLabs Client (`api/elevenlabs.rs`)

Text-to-speech synthesis client:

**Features:**
- Voice selection with voice ID
- Customizable voice settings (stability, similarity_boost, style)
- Character limit validation (5000 chars)
- MP3 audio output
- Voice listing capability
- API key authentication via header
- Quota tracking

**Main Method:**
```rust
async fn synthesize_speech(
    &self,
    text: &str,
) -> AppResult<Vec<u8>>
```

### 5. Tauri Commands (`commands.rs`)

All commands exposed to the frontend:

#### Audio Processing
- `process_audio(audio_data, filename)` - Transcribe audio to text
- `synthesize_speech(text)` - Convert text to audio

#### LLM Interaction
- `send_message(message)` - Send text to LLM and get response

#### Complete Pipeline
- `process_voice_query(audio_data, filename)` - Full voice assistant pipeline:
  1. Transcribe audio
  2. Send to LLM
  3. Convert response to speech

Returns: `VoiceQueryResponse { transcription, llm_response, audio_response }`

#### Configuration
- `load_config()` - Get current configuration
- `save_config(config)` - Save configuration
- `update_api_key(service, api_key)` - Store API key securely

#### State Management
- `get_app_state()` - Get current application state
- `get_conversation()` - Get conversation history
- `clear_conversation()` - Clear conversation history

#### Connectivity
- `check_connectivity()` - Check all service connections

#### Voice Management
- `list_voices()` - Get available ElevenLabs voices
- `update_voice_settings(settings)` - Update voice synthesis settings

### 6. Main Library (`lib.rs`)

Application initialization and setup:

**Features:**
- Logger initialization (env_logger)
- Plugin registration:
  - `tauri_plugin_opener`
  - `tauri_plugin_dialog`
  - `tauri_plugin_global_shortcut`
- System tray setup (macOS/Windows/Linux)
- Configuration loading
- Application state initialization
- Command handler registration
- Window configuration

## Data Flow

### Voice Query Pipeline

```
User speaks
    ↓
Frontend records audio (16kHz WAV)
    ↓
process_voice_query(audio_data, filename)
    ↓
State: Transcribing
    ↓
Whisper API → transcription text
    ↓
State: Thinking
    ↓
OpenWebUI API (with conversation context) → LLM response
    ↓
State: Speaking
    ↓
ElevenLabs API → MP3 audio
    ↓
State: Idle
    ↓
Return VoiceQueryResponse to frontend
    ↓
Frontend plays audio response
```

## Configuration Example

```json
{
  "whisper": {
    "endpoint": "https://api.openai.com/v1/audio/transcriptions",
    "model": "whisper-1",
    "language": "en",
    "temperature": 0.0,
    "timeout_secs": 30
  },
  "openwebui": {
    "endpoint": "http://localhost:3000/api/chat",
    "model": "llama3.2",
    "max_context_length": 4096,
    "temperature": 0.7,
    "max_tokens": 1024,
    "stream": false,
    "timeout_secs": 60
  },
  "elevenlabs": {
    "endpoint": "https://api.elevenlabs.io/v1/text-to-speech",
    "voice_id": "21m00Tcm4TlvDq8ikWAM",
    "model_id": "eleven_monolingual_v1",
    "voice_settings": {
      "stability": 0.5,
      "similarity_boost": 0.75,
      "style": 0.0,
      "use_speaker_boost": true
    },
    "timeout_secs": 30
  },
  "audio": {
    "sample_rate": 16000,
    "bit_depth": 16,
    "channels": 1,
    "format": "wav",
    "silence_threshold": 0.01,
    "silence_duration": 2.0,
    "max_duration": 300
  },
  "ui": {
    "theme": "dark",
    "show_transcription": true,
    "show_thinking": true,
    "auto_minimize": false,
    "always_on_top": true,
    "global_hotkey": "CommandOrControl+Shift+Space"
  }
}
```

## Environment Variables

API keys can be provided via environment variables (takes precedence over keyring):

- `WHISPER_API_KEY`
- `OPENWEBUI_API_KEY`
- `ELEVENLABS_API_KEY`

## Security Features

1. **Secure Key Storage**: API keys stored in system keyring (not in config file)
2. **No Hardcoded Secrets**: All sensitive data loaded from keyring or environment
3. **HTTPS by Default**: All API clients use TLS
4. **Input Validation**: File size limits, character limits, context length validation
5. **Error Sanitization**: Errors don't leak sensitive information

## Error Handling Strategy

1. **Retry Logic**: All API calls retry up to 3 times with exponential backoff
2. **Timeout Support**: Configurable timeouts for all network operations
3. **Graceful Degradation**: Services fail independently without crashing the app
4. **User-Friendly Messages**: Technical errors converted to readable messages
5. **Logging**: All errors logged with context for debugging

## Performance Considerations

1. **Thread Safety**: All shared state uses Arc<Mutex<T>>
2. **Async/Await**: All blocking operations are async
3. **Connection Pooling**: Reqwest client reused for all requests
4. **Memory Management**: Message history automatically trimmed
5. **Timeout Protection**: Prevents hanging on unresponsive services

## Testing

Each module includes unit tests:
- Error type conversions
- Configuration serialization/deserialization
- State management operations
- Input validation
- Message trimming logic

Run tests with:
```bash
cd src-tauri
cargo test
```

## Building

Build the application:
```bash
cd src-tauri
cargo build --release
```

## Dependencies

Key dependencies (see Cargo.toml):
- `tauri` (v2): Application framework
- `tokio`: Async runtime
- `reqwest`: HTTP client with multipart and TLS support
- `serde` / `serde_json`: Serialization
- `thiserror`: Error handling
- `keyring`: Secure credential storage
- `log` / `env_logger`: Logging

## Future Enhancements

1. **Streaming Support**: Implement SSE for OpenWebUI streaming responses
2. **Voice Activity Detection**: Automatic silence detection during recording
3. **Audio Processing**: Built-in audio conversion and normalization
4. **Conversation Persistence**: Save/load conversations to disk
5. **Multi-Language Support**: Dynamic language switching
6. **Voice Cloning**: Custom voice training support
7. **Metrics**: Performance and usage tracking
8. **Health Checks**: Periodic background connectivity checks

## Troubleshooting

### Common Issues

1. **API Key Not Found**
   - Set environment variable: `export WHISPER_API_KEY=your_key`
   - Or store via command: `update_api_key("whisper", "your_key")`

2. **Connection Refused**
   - Check OpenWebUI is running: `http://localhost:3000`
   - Verify endpoint in configuration

3. **Transcription Timeout**
   - Increase timeout_secs in config
   - Check audio file size (must be < 25MB)
   - Verify audio format (16kHz WAV)

4. **Context Limit Exceeded**
   - Clear conversation: `clear_conversation()`
   - Reduce max_context_length in config

## File Locations

All created backend files with absolute paths:

1. `/Users/cojovi/dev/windows_gpt/talk-to-cmac/src-tauri/src/error.rs`
2. `/Users/cojovi/dev/windows_gpt/talk-to-cmac/src-tauri/src/config.rs`
3. `/Users/cojovi/dev/windows_gpt/talk-to-cmac/src-tauri/src/state.rs`
4. `/Users/cojovi/dev/windows_gpt/talk-to-cmac/src-tauri/src/api/mod.rs`
5. `/Users/cojovi/dev/windows_gpt/talk-to-cmac/src-tauri/src/api/whisper.rs`
6. `/Users/cojovi/dev/windows_gpt/talk-to-cmac/src-tauri/src/api/openwebui.rs`
7. `/Users/cojovi/dev/windows_gpt/talk-to-cmac/src-tauri/src/api/elevenlabs.rs`
8. `/Users/cojovi/dev/windows_gpt/talk-to-cmac/src-tauri/src/commands.rs`
9. `/Users/cojovi/dev/windows_gpt/talk-to-cmac/src-tauri/src/lib.rs` (updated)

## License

[Your License Here]

## Contributors

[Your Name/Team]

---

For frontend integration examples, see the frontend documentation.
