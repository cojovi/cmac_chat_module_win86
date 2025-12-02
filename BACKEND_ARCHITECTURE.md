# Backend Architecture Design
## Talk to CMAC - Windows Voice Assistant

---

## Table of Contents
1. [High-Level Architecture](#high-level-architecture)
2. [API Integration Layer](#api-integration-layer)
3. [State Management](#state-management)
4. [Audio Pipeline Architecture](#audio-pipeline-architecture)
5. [Rust Backend Command Structure](#rust-backend-command-structure)
6. [Configuration Management](#configuration-management)
7. [Error Handling Strategy](#error-handling-strategy)
8. [Security Considerations](#security-considerations)
9. [Recommended Rust Crates](#recommended-rust-crates)
10. [Implementation Roadmap](#implementation-roadmap)

---

## High-Level Architecture

### System Architecture Diagram (ASCII)

```
┌─────────────────────────────────────────────────────────────────────┐
│                         FRONTEND (WebView2)                          │
│  ┌────────────┐  ┌──────────────┐  ┌─────────────┐  ┌────────────┐ │
│  │ System     │  │ Chat UI      │  │ Audio       │  │ Status     │ │
│  │ Tray       │  │ Component    │  │ Visualizer  │  │ Indicators │ │
│  └─────┬──────┘  └──────┬───────┘  └──────┬──────┘  └─────┬──────┘ │
│        │                │                  │               │        │
│        └────────────────┴──────────────────┴───────────────┘        │
│                                 │                                   │
│                        Tauri IPC Commands                           │
└──────────────────────────────────┬──────────────────────────────────┘
                                   │
┌──────────────────────────────────┴──────────────────────────────────┐
│                      RUST BACKEND (Tauri Core)                      │
│                                                                      │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │                   Command Handler Layer                      │  │
│  │  ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌──────────────┐   │  │
│  │  │ Audio    │ │ AI Chat  │ │ Config   │ │ System       │   │  │
│  │  │ Commands │ │ Commands │ │ Commands │ │ Integration  │   │  │
│  │  └────┬─────┘ └────┬─────┘ └────┬─────┘ └──────┬───────┘   │  │
│  └───────┼────────────┼────────────┼───────────────┼───────────┘  │
│          │            │            │               │              │
│  ┌───────┴────────────┴────────────┴───────────────┴───────────┐  │
│  │                   State Manager (Arc<Mutex>)                │  │
│  │  - Conversation Context (in-memory)                         │  │
│  │  - Application State (idle/listening/thinking/speaking)     │  │
│  │  - API Connection Status                                    │  │
│  │  - Configuration Cache                                      │  │
│  └───────┬────────────┬────────────┬───────────────┬───────────┘  │
│          │            │            │               │              │
│  ┌───────┴────┐  ┌────┴─────┐  ┌──┴────────┐  ┌──┴──────────┐   │
│  │ Audio      │  │ API      │  │ Config    │  │ Crypto      │   │
│  │ Manager    │  │ Client   │  │ Manager   │  │ Service     │   │
│  │            │  │ Pool     │  │           │  │             │   │
│  └────┬───────┘  └────┬─────┘  └───┬───────┘  └──┬──────────┘   │
│       │               │             │             │              │
└───────┼───────────────┼─────────────┼─────────────┼──────────────┘
        │               │             │             │
        │               │             │             │
┌───────┴───────┐ ┌────┴──────────┐ ┌┴────────┐ ┌─┴──────────┐
│ OS Audio      │ │ External APIs │ │ Disk    │ │ Windows    │
│ System        │ │               │ │ Storage │ │ Keyring    │
│ - Mic Input   │ │ - Whisper STT │ │ (Config)│ │            │
│ - Speaker Out │ │ - OpenWebUI   │ └─────────┘ └────────────┘
└───────────────┘ │ - ElevenLabs  │
                  └───────────────┘
```

### Data Flow - Voice Query Pipeline

```
USER SPEAKS
    │
    ▼
[1] AUDIO CAPTURE
    │ Browser getUserMedia() → Tauri command
    │ Format: 16kHz, 16-bit PCM, Mono WAV
    │ Duration: User-controlled (push-to-talk)
    ▼
[2] SPEECH-TO-TEXT (Whisper API)
    │ POST audio → OpenAI Whisper endpoint
    │ Timeout: 30s
    │ Response: { "text": "..." }
    │ Error Handling: Retry once, then fail gracefully
    ▼
[3] AI REASONING (OpenWebUI)
    │ POST /api/chat
    │ Headers: { Authorization: Bearer <token> }
    │ Body: {
    │   "model": "<model_name>",
    │   "messages": [
    │     { "role": "user", "content": "<transcribed_text>" }
    │   ],
    │   "knowledge_ids": ["<optional_collection_id>"]
    │ }
    │ Timeout: 60s
    │ Response: { "choices": [{ "message": { "content": "..." }}]}
    ▼
[4] TEXT-TO-SPEECH (ElevenLabs)
    │ POST /v1/text-to-speech/<voice_id>
    │ Headers: { xi-api-key: <key> }
    │ Body: { "text": "...", "model_id": "eleven_monolingual_v1" }
    │ Timeout: 45s
    │ Response: Audio stream (MP3)
    ▼
[5] AUDIO PLAYBACK
    │ Store audio in memory buffer
    │ Play via HTML5 Audio element or native audio
    │ Support interrupt/stop
    ▼
USER HEARS RESPONSE
```

---

## API Integration Layer

### 1. OpenAI Whisper API (Speech-to-Text)

**Endpoint**: `https://api.openai.com/v1/audio/transcriptions`

**Request Specification**:
```
Method: POST
Content-Type: multipart/form-data

Parameters:
- file: audio file (binary) - WAV, MP3, or M4A
- model: "whisper-1"
- language: "en" (optional, improves accuracy)
- response_format: "json" (default) or "verbose_json" for timestamps
```

**Audio Format Conversion**:
- **Input from browser**: WebM or OGG (browser default)
- **Required format**: WAV, MP3, M4A, or FLAC
- **Conversion strategy**:
  - Option A: Use browser MediaRecorder with MIME type `audio/wav`
  - Option B: Convert in Rust using `ffmpeg` or `hound` crate
  - **Recommended**: Send raw WAV from browser to avoid backend conversion

**Response Parsing**:
```json
{
  "text": "This is the transcribed text."
}
```

**Error Scenarios**:
- HTTP 400: Invalid audio format → Convert format
- HTTP 401: Invalid API key → Notify user, check config
- HTTP 429: Rate limit → Retry after delay (exponential backoff)
- Timeout: Network issue → Show offline warning

**Implementation Details**:
```rust
// Pseudo-structure
struct WhisperClient {
    api_key: String,
    http_client: reqwest::Client,
}

impl WhisperClient {
    async fn transcribe(&self, audio_data: Vec<u8>) -> Result<String> {
        let form = reqwest::multipart::Form::new()
            .part("file",
                  reqwest::multipart::Part::bytes(audio_data)
                      .file_name("audio.wav")
                      .mime_str("audio/wav")?)
            .text("model", "whisper-1")
            .text("language", "en");

        let response = self.http_client
            .post("https://api.openai.com/v1/audio/transcriptions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .multipart(form)
            .timeout(Duration::from_secs(30))
            .send()
            .await?;

        // Parse JSON response
        let result: WhisperResponse = response.json().await?;
        Ok(result.text)
    }
}
```

---

### 2. OpenWebUI API (AI Reasoning)

**Endpoint**: `{OPENWEBUI_BASE_URL}/api/chat` or `/api/chat/completions`

**Authentication**: Bearer token in Authorization header

**Request Specification**:
```json
POST /api/chat
Headers:
  Authorization: Bearer <OPENWEBUI_API_KEY>
  Content-Type: application/json

Body:
{
  "model": "<OPENWEBUI_MODEL_NAME>",
  "messages": [
    { "role": "system", "content": "You are CMAC assistant..." },
    { "role": "user", "content": "<user_query>" }
  ],
  "knowledge_ids": ["<OPENWEBUI_KNOWLEDGE_COLLECTION_ID>"],  // Optional
  "stream": false
}
```

**Context Management (In-Memory Only)**:
- Maintain conversation history in `Vec<Message>` during session
- Structure:
  ```rust
  struct Message {
      role: String,  // "user" | "assistant" | "system"
      content: String,
      timestamp: DateTime<Utc>,
  }

  struct ConversationContext {
      messages: Vec<Message>,
      max_history: usize,  // e.g., 10 messages
  }
  ```
- When sending to AI:
  - Include system prompt as first message
  - Include last N user/assistant exchanges
  - Trim old messages to prevent context length overflow
- Clear on:
  - Application restart
  - Manual "Clear Context" command
  - Session timeout (optional: 1 hour of inactivity)

**Model Selection**:
- Read from config: `OPENWEBUI_MODEL_NAME`
- Validate model exists via `/api/models` endpoint on startup
- Fallback to default if invalid

**Response Parsing**:
```json
{
  "id": "chatcmpl-...",
  "object": "chat.completion",
  "choices": [
    {
      "index": 0,
      "message": {
        "role": "assistant",
        "content": "Here is the response..."
      },
      "finish_reason": "stop"
    }
  ]
}
```

**Error Scenarios**:
- HTTP 401: Invalid API key
- HTTP 404: Model not found → Check `OPENWEBUI_MODEL_NAME`
- HTTP 500: Backend error → Retry once, then fail
- Timeout (60s): Show "AI unavailable" message

**Implementation Details**:
```rust
struct OpenWebUIClient {
    base_url: String,
    api_key: String,
    model_name: String,
    knowledge_collection_id: Option<String>,
    http_client: reqwest::Client,
}

impl OpenWebUIClient {
    async fn send_message(&self, context: &ConversationContext, user_input: &str)
        -> Result<String> {

        let mut messages = vec![
            json!({
                "role": "system",
                "content": "You are CMAC, a helpful roofing assistant..."
            })
        ];

        // Add conversation history
        messages.extend(context.messages.iter().map(|m| {
            json!({ "role": m.role, "content": m.content })
        }));

        // Add current user message
        messages.push(json!({
            "role": "user",
            "content": user_input
        }));

        let mut body = json!({
            "model": self.model_name,
            "messages": messages,
            "stream": false
        });

        if let Some(kid) = &self.knowledge_collection_id {
            body["knowledge_ids"] = json!([kid]);
        }

        let response = self.http_client
            .post(format!("{}/api/chat", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&body)
            .timeout(Duration::from_secs(60))
            .send()
            .await?;

        let result: ChatResponse = response.json().await?;
        Ok(result.choices[0].message.content.clone())
    }
}
```

---

### 3. ElevenLabs API (Text-to-Speech)

**Endpoint**: `https://api.elevenlabs.io/v1/text-to-speech/{voice_id}`

**Authentication**: `xi-api-key` header

**Request Specification**:
```json
POST /v1/text-to-speech/{voice_id}
Headers:
  xi-api-key: <ELEVENLABS_API_KEY>
  Content-Type: application/json

Body:
{
  "text": "The response text to synthesize",
  "model_id": "eleven_monolingual_v1",
  "voice_settings": {
    "stability": 0.5,
    "similarity_boost": 0.75
  }
}
```

**Voice Selection**:
- Fetch available voices: `GET /v1/voices`
- Store voice options in config
- Default voice IDs (examples):
  - `21m00Tcm4TlvDq8ikWAM` - Rachel (neutral)
  - `AZnzlk1XvdvUeBnXmlld` - Domi (confident)
  - `EXAVITQu4vr4xnSDxMaL` - Bella (soft)
- Allow user to select via settings UI

**Audio Format Handling**:
- Response: Binary audio stream (MP3 by default)
- Options:
  - `output_format`: `mp3_44100_128` (default), `pcm_16000`, `pcm_22050`
- **Recommended**: Use MP3 for smaller size, decode in browser

**Streaming vs Full Download**:
- **Full download** (recommended for simplicity):
  - Wait for complete audio before playback
  - Buffer in memory
  - Latency: 1-3 seconds for short responses
- **Streaming** (advanced, optional):
  - Use `/v1/text-to-speech/{voice_id}/stream` endpoint
  - Progressively play chunks as they arrive
  - More complex: requires chunked transfer handling
  - Use if latency becomes issue

**Response Handling**:
```rust
// Full response
let audio_bytes: Vec<u8> = response.bytes().await?.to_vec();

// Convert to base64 for frontend transfer
let audio_base64 = base64::encode(&audio_bytes);

// Or save to temp file and return path
let temp_path = temp_dir().join(format!("tts_{}.mp3", Uuid::new_v4()));
fs::write(&temp_path, audio_bytes)?;
```

**Error Scenarios**:
- HTTP 401: Invalid API key
- HTTP 400: Text too long (max 5000 chars) → Split text
- HTTP 402: Insufficient quota → Notify user
- HTTP 429: Rate limit → Retry with backoff
- Timeout: Network issue

**Implementation Details**:
```rust
struct ElevenLabsClient {
    api_key: String,
    voice_id: String,
    http_client: reqwest::Client,
}

impl ElevenLabsClient {
    async fn synthesize(&self, text: &str) -> Result<Vec<u8>> {
        let body = json!({
            "text": text,
            "model_id": "eleven_monolingual_v1",
            "voice_settings": {
                "stability": 0.5,
                "similarity_boost": 0.75
            }
        });

        let response = self.http_client
            .post(format!(
                "https://api.elevenlabs.io/v1/text-to-speech/{}",
                self.voice_id
            ))
            .header("xi-api-key", &self.api_key)
            .json(&body)
            .timeout(Duration::from_secs(45))
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(anyhow!("TTS failed: {}", response.status()));
        }

        let audio_bytes = response.bytes().await?.to_vec();
        Ok(audio_bytes)
    }

    async fn get_voices(&self) -> Result<Vec<Voice>> {
        let response = self.http_client
            .get("https://api.elevenlabs.io/v1/voices")
            .header("xi-api-key", &self.api_key)
            .send()
            .await?;

        let result: VoicesResponse = response.json().await?;
        Ok(result.voices)
    }
}
```

---

## State Management

### Application State Machine

```
┌─────────┐
│  IDLE   │ ◄─────────────────────────────┐
└────┬────┘                                │
     │ user presses PTT                    │
     ▼                                     │
┌──────────┐                               │
│LISTENING │                               │
└────┬─────┘                               │
     │ user releases PTT                   │
     ▼                                     │
┌──────────────┐                           │
│TRANSCRIBING  │                           │
└──────┬───────┘                           │
       │ transcription complete            │
       ▼                                   │
┌──────────┐                               │
│THINKING  │                               │
└────┬─────┘                               │
     │ AI response received                │
     ▼                                     │
┌──────────┐                               │
│SPEAKING  │                               │
└────┬─────┘                               │
     │ audio playback complete or stopped  │
     └──────────────────────────────────────┘
```

**State Enum**:
```rust
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AppState {
    Idle,
    Listening,        // Microphone is recording
    Transcribing,     // Sending audio to Whisper
    Thinking,         // Waiting for AI response
    Speaking,         // Playing TTS audio
    Error(String),    // Error state with message
}
```

### Shared State Structure

```rust
use std::sync::{Arc, Mutex};
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

#[derive(Clone)]
pub struct AppStateManager {
    inner: Arc<Mutex<AppStateInner>>,
}

struct AppStateInner {
    current_state: AppState,
    conversation_context: ConversationContext,
    api_status: ApiConnectionStatus,
    audio_buffer: Option<Vec<u8>>,
    config_cache: ConfigCache,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationContext {
    messages: Vec<Message>,
    session_start: DateTime<Utc>,
    max_history: usize,
}

impl ConversationContext {
    pub fn add_user_message(&mut self, content: String) {
        self.messages.push(Message {
            role: "user".to_string(),
            content,
            timestamp: Utc::now(),
        });
        self.trim_history();
    }

    pub fn add_assistant_message(&mut self, content: String) {
        self.messages.push(Message {
            role: "assistant".to_string(),
            content,
            timestamp: Utc::now(),
        });
        self.trim_history();
    }

    fn trim_history(&mut self) {
        if self.messages.len() > self.max_history {
            let keep_count = self.max_history;
            self.messages = self.messages
                .iter()
                .rev()
                .take(keep_count)
                .rev()
                .cloned()
                .collect();
        }
    }

    pub fn clear(&mut self) {
        self.messages.clear();
        self.session_start = Utc::now();
    }

    pub fn get_messages(&self) -> &[Message] {
        &self.messages
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiConnectionStatus {
    whisper_available: bool,
    openwebui_available: bool,
    elevenlabs_available: bool,
    last_check: DateTime<Utc>,
    network_online: bool,
}

impl ApiConnectionStatus {
    pub fn is_fully_operational(&self) -> bool {
        self.network_online
            && self.whisper_available
            && self.openwebui_available
            && self.elevenlabs_available
    }

    pub fn can_process_voice(&self) -> bool {
        self.whisper_available && self.openwebui_available
    }
}
```

### State Update Pattern

```rust
impl AppStateManager {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(AppStateInner {
                current_state: AppState::Idle,
                conversation_context: ConversationContext::new(),
                api_status: ApiConnectionStatus::default(),
                audio_buffer: None,
                config_cache: ConfigCache::default(),
            })),
        }
    }

    pub fn set_state(&self, new_state: AppState) -> Result<()> {
        let mut inner = self.inner.lock()
            .map_err(|e| anyhow!("Failed to lock state: {}", e))?;

        // Validate state transition
        match (&inner.current_state, &new_state) {
            (AppState::Listening, AppState::Transcribing) => {},
            (AppState::Transcribing, AppState::Thinking) => {},
            (AppState::Thinking, AppState::Speaking) => {},
            (AppState::Speaking, AppState::Idle) => {},
            (_, AppState::Idle) => {},  // Can always return to idle
            (_, AppState::Error(_)) => {},  // Can always error
            _ => {
                warn!("Unexpected state transition: {:?} -> {:?}",
                      inner.current_state, new_state);
            }
        }

        inner.current_state = new_state;
        Ok(())
    }

    pub fn get_state(&self) -> Result<AppState> {
        Ok(self.inner.lock()
            .map_err(|e| anyhow!("Failed to lock state: {}", e))?
            .current_state
            .clone())
    }

    pub fn add_user_message(&self, content: String) -> Result<()> {
        self.inner.lock()
            .map_err(|e| anyhow!("Failed to lock state: {}", e))?
            .conversation_context
            .add_user_message(content);
        Ok(())
    }

    pub fn get_conversation(&self) -> Result<Vec<Message>> {
        Ok(self.inner.lock()
            .map_err(|e| anyhow!("Failed to lock state: {}", e))?
            .conversation_context
            .get_messages()
            .to_vec())
    }

    pub fn clear_conversation(&self) -> Result<()> {
        self.inner.lock()
            .map_err(|e| anyhow!("Failed to lock state: {}", e))?
            .conversation_context
            .clear();
        Ok(())
    }
}
```

---

## Audio Pipeline Architecture

### Audio Capture Flow

```
┌──────────────────────────────────────────────────────────────────┐
│                      BROWSER (Frontend)                          │
│                                                                  │
│  [User presses PTT button or hotkey]                            │
│            ↓                                                     │
│  navigator.mediaDevices.getUserMedia({ audio: true })           │
│            ↓                                                     │
│  MediaRecorder starts recording                                 │
│    - MIME type: audio/wav or audio/webm                         │
│    - Sample rate: 16000 Hz                                      │
│    - Channels: 1 (mono)                                         │
│            ↓                                                     │
│  [User releases PTT]                                            │
│            ↓                                                     │
│  MediaRecorder stops, fires 'dataavailable' event               │
│            ↓                                                     │
│  Blob received → Convert to ArrayBuffer → Base64 or Binary      │
│            ↓                                                     │
│  invoke('process_audio', { audioData: base64 })                 │
└───────────────────────────┬──────────────────────────────────────┘
                            │
                            │ Tauri IPC
                            ▼
┌──────────────────────────────────────────────────────────────────┐
│                      RUST BACKEND                                │
│                                                                  │
│  #[tauri::command]                                               │
│  async fn process_audio(audio_data: String) -> Result<...>      │
│            ↓                                                     │
│  Decode base64 to Vec<u8>                                       │
│            ↓                                                     │
│  [Optional] Validate/Convert format                             │
│            ↓                                                     │
│  Send to Whisper API                                            │
│            ↓                                                     │
│  Return transcription to frontend                               │
└──────────────────────────────────────────────────────────────────┘
```

**Frontend Audio Capture (JavaScript)**:
```javascript
class AudioCapture {
    constructor() {
        this.mediaRecorder = null;
        this.audioChunks = [];
    }

    async startRecording() {
        const stream = await navigator.mediaDevices.getUserMedia({
            audio: {
                channelCount: 1,
                sampleRate: 16000,
                echoCancellation: true,
                noiseSuppression: true,
            }
        });

        this.mediaRecorder = new MediaRecorder(stream, {
            mimeType: 'audio/webm;codecs=opus'
        });

        this.audioChunks = [];

        this.mediaRecorder.ondataavailable = (event) => {
            if (event.data.size > 0) {
                this.audioChunks.push(event.data);
            }
        };

        this.mediaRecorder.start();
    }

    async stopRecording() {
        return new Promise((resolve) => {
            this.mediaRecorder.onstop = async () => {
                const audioBlob = new Blob(this.audioChunks, {
                    type: 'audio/webm'
                });
                const arrayBuffer = await audioBlob.arrayBuffer();
                const base64 = btoa(
                    String.fromCharCode(...new Uint8Array(arrayBuffer))
                );
                resolve(base64);
            };

            this.mediaRecorder.stop();
            this.mediaRecorder.stream.getTracks().forEach(t => t.stop());
        });
    }
}
```

### Audio Format Conversion (Rust)

**Option 1: Accept WebM directly** (send to Whisper as-is)
- Whisper API accepts WebM
- No conversion needed
- Simplest approach

**Option 2: Convert to WAV** (if needed)
```rust
use hound::{WavReader, WavWriter, WavSpec};

fn convert_to_wav(input: &[u8]) -> Result<Vec<u8>> {
    // If input is already WAV, pass through
    if input.starts_with(b"RIFF") {
        return Ok(input.to_vec());
    }

    // For WebM/Opus, would need ffmpeg or similar
    // Simpler: let Whisper handle format conversion
    Ok(input.to_vec())
}
```

**Recommendation**: Send audio directly to Whisper without conversion. Whisper supports multiple formats.

### Audio Playback Queue

**Playback Strategy**:
```rust
pub struct AudioPlaybackManager {
    current_audio: Arc<Mutex<Option<PlaybackHandle>>>,
}

pub struct PlaybackHandle {
    audio_data: Vec<u8>,
    is_playing: Arc<AtomicBool>,
    stop_signal: Arc<AtomicBool>,
}

impl AudioPlaybackManager {
    pub async fn play_audio(&self, audio_data: Vec<u8>) -> Result<()> {
        // Stop any current playback
        self.stop_current();

        let handle = PlaybackHandle {
            audio_data: audio_data.clone(),
            is_playing: Arc::new(AtomicBool::new(true)),
            stop_signal: Arc::new(AtomicBool::new(false)),
        };

        *self.current_audio.lock().unwrap() = Some(handle.clone());

        // Send audio to frontend for playback
        // Frontend handles actual audio output via HTML5 Audio

        Ok(())
    }

    pub fn stop_current(&self) {
        if let Some(handle) = self.current_audio.lock().unwrap().as_ref() {
            handle.stop_signal.store(true, Ordering::SeqCst);
        }
    }

    pub fn is_playing(&self) -> bool {
        self.current_audio.lock()
            .unwrap()
            .as_ref()
            .map(|h| h.is_playing.load(Ordering::SeqCst))
            .unwrap_or(false)
    }
}
```

**Frontend Playback**:
```javascript
async function playTTSAudio(audioBase64) {
    // Convert base64 to blob
    const binaryString = atob(audioBase64);
    const bytes = new Uint8Array(binaryString.length);
    for (let i = 0; i < binaryString.length; i++) {
        bytes[i] = binaryString.charCodeAt(i);
    }
    const audioBlob = new Blob([bytes], { type: 'audio/mpeg' });
    const audioUrl = URL.createObjectURL(audioBlob);

    // Create and play audio element
    const audio = new Audio(audioUrl);
    audio.play();

    // Store reference for interruption
    window.currentAudio = audio;

    return new Promise((resolve) => {
        audio.onended = () => {
            URL.revokeObjectURL(audioUrl);
            resolve();
        };
    });
}

function stopCurrentAudio() {
    if (window.currentAudio) {
        window.currentAudio.pause();
        window.currentAudio = null;
    }
}
```

### Interrupt Handling

```rust
#[tauri::command]
async fn stop_speaking(state: State<'_, AppStateManager>) -> Result<(), String> {
    state.set_state(AppState::Idle).map_err(|e| e.to_string())?;

    // Signal frontend to stop audio
    // Frontend will pause the HTML5 Audio element

    Ok(())
}
```

---

## Rust Backend Command Structure

### Core Tauri Commands

```rust
use tauri::{State, Window};
use serde::{Serialize, Deserialize};

// ============================================================================
// AUDIO COMMANDS
// ============================================================================

/// Process captured audio: transcribe with Whisper
#[tauri::command]
async fn process_audio(
    audio_data: String,  // Base64-encoded audio
    state: State<'_, AppStateManager>,
    api_clients: State<'_, ApiClients>,
) -> Result<String, String> {
    // Update state
    state.set_state(AppState::Transcribing)
        .map_err(|e| e.to_string())?;

    // Decode audio
    let audio_bytes = base64::decode(&audio_data)
        .map_err(|e| format!("Failed to decode audio: {}", e))?;

    // Call Whisper API
    let transcription = api_clients.whisper
        .transcribe(audio_bytes)
        .await
        .map_err(|e| {
            state.set_state(AppState::Error(e.to_string())).ok();
            format!("Transcription failed: {}", e)
        })?;

    Ok(transcription)
}

/// Generate TTS audio from text
#[tauri::command]
async fn synthesize_speech(
    text: String,
    state: State<'_, AppStateManager>,
    api_clients: State<'_, ApiClients>,
) -> Result<String, String> {  // Returns base64 audio
    state.set_state(AppState::Speaking)
        .map_err(|e| e.to_string())?;

    let audio_bytes = api_clients.elevenlabs
        .synthesize(&text)
        .await
        .map_err(|e| format!("TTS failed: {}", e))?;

    let audio_base64 = base64::encode(&audio_bytes);
    Ok(audio_base64)
}

/// Stop current audio playback
#[tauri::command]
async fn stop_audio(
    state: State<'_, AppStateManager>,
    window: Window,
) -> Result<(), String> {
    state.set_state(AppState::Idle).map_err(|e| e.to_string())?;

    // Emit event to frontend to stop audio
    window.emit("stop-audio", ()).map_err(|e| e.to_string())?;

    Ok(())
}

// ============================================================================
// AI CHAT COMMANDS
// ============================================================================

/// Send message to AI and get response
#[tauri::command]
async fn send_message(
    message: String,
    state: State<'_, AppStateManager>,
    api_clients: State<'_, ApiClients>,
) -> Result<String, String> {
    // Add user message to context
    state.add_user_message(message.clone())
        .map_err(|e| e.to_string())?;

    // Update state
    state.set_state(AppState::Thinking)
        .map_err(|e| e.to_string())?;

    // Get conversation context
    let context = state.get_conversation()
        .map_err(|e| e.to_string())?;

    // Call OpenWebUI
    let response = api_clients.openwebui
        .send_message(&context, &message)
        .await
        .map_err(|e| {
            state.set_state(AppState::Error(e.to_string())).ok();
            format!("AI request failed: {}", e)
        })?;

    // Add assistant response to context
    state.add_assistant_message(response.clone())
        .map_err(|e| e.to_string())?;

    state.set_state(AppState::Idle).map_err(|e| e.to_string())?;

    Ok(response)
}

/// Process voice query (full pipeline)
#[tauri::command]
async fn process_voice_query(
    audio_data: String,
    state: State<'_, AppStateManager>,
    api_clients: State<'_, ApiClients>,
) -> Result<VoiceQueryResponse, String> {
    // Step 1: Transcribe
    let transcription = process_audio(audio_data, state.clone(), api_clients.clone())
        .await?;

    // Step 2: Get AI response
    let ai_response = send_message(transcription.clone(), state.clone(), api_clients.clone())
        .await?;

    // Step 3: Synthesize speech
    let audio_base64 = synthesize_speech(ai_response.clone(), state, api_clients)
        .await?;

    Ok(VoiceQueryResponse {
        transcription,
        ai_response,
        audio_base64,
    })
}

#[derive(Serialize)]
struct VoiceQueryResponse {
    transcription: String,
    ai_response: String,
    audio_base64: String,
}

/// Clear conversation context
#[tauri::command]
async fn clear_conversation(
    state: State<'_, AppStateManager>,
) -> Result<(), String> {
    state.clear_conversation().map_err(|e| e.to_string())
}

/// Get current conversation history
#[tauri::command]
async fn get_conversation(
    state: State<'_, AppStateManager>,
) -> Result<Vec<Message>, String> {
    state.get_conversation().map_err(|e| e.to_string())
}

// ============================================================================
// CONFIGURATION COMMANDS
// ============================================================================

/// Load configuration from disk
#[tauri::command]
async fn load_config(
    config_manager: State<'_, ConfigManager>,
) -> Result<AppConfig, String> {
    config_manager.load().await.map_err(|e| e.to_string())
}

/// Save configuration to disk
#[tauri::command]
async fn save_config(
    config: AppConfig,
    config_manager: State<'_, ConfigManager>,
) -> Result<(), String> {
    config_manager.save(config).await.map_err(|e| e.to_string())
}

/// Validate API keys by making test requests
#[tauri::command]
async fn validate_api_keys(
    config: AppConfig,
) -> Result<ApiValidationResult, String> {
    // Test each API
    let whisper_valid = test_whisper_key(&config.openai_api_key).await;
    let openwebui_valid = test_openwebui_connection(&config).await;
    let elevenlabs_valid = test_elevenlabs_key(&config.elevenlabs_api_key).await;

    Ok(ApiValidationResult {
        whisper_valid,
        openwebui_valid,
        elevenlabs_valid,
    })
}

/// Get available ElevenLabs voices
#[tauri::command]
async fn get_available_voices(
    api_clients: State<'_, ApiClients>,
) -> Result<Vec<Voice>, String> {
    api_clients.elevenlabs
        .get_voices()
        .await
        .map_err(|e| e.to_string())
}

// ============================================================================
// SYSTEM INTEGRATION COMMANDS
// ============================================================================

/// Check network connectivity
#[tauri::command]
async fn check_connectivity(
    state: State<'_, AppStateManager>,
) -> Result<ApiConnectionStatus, String> {
    // Perform health checks on all APIs
    let status = perform_health_checks().await;

    // Update state
    state.update_api_status(status.clone())
        .map_err(|e| e.to_string())?;

    Ok(status)
}

/// Get current application state
#[tauri::command]
async fn get_app_state(
    state: State<'_, AppStateManager>,
) -> Result<AppState, String> {
    state.get_state().map_err(|e| e.to_string())
}

/// Register global hotkey
#[tauri::command]
async fn register_hotkey(
    hotkey: String,
    app: tauri::AppHandle,
) -> Result<(), String> {
    use tauri_plugin_global_shortcut::GlobalShortcutExt;

    app.global_shortcut()
        .register(hotkey)
        .map_err(|e| e.to_string())?;

    Ok(())
}

/// Show/hide application window
#[tauri::command]
async fn toggle_window_visibility(
    window: Window,
) -> Result<(), String> {
    if window.is_visible().map_err(|e| e.to_string())? {
        window.hide().map_err(|e| e.to_string())?;
    } else {
        window.show().map_err(|e| e.to_string())?;
        window.set_focus().map_err(|e| e.to_string())?;
    }
    Ok(())
}
```

### Main Application Setup

```rust
use tauri::Manager;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_global_shortcut::init())
        .setup(|app| {
            // Initialize state managers
            let config_manager = ConfigManager::new()?;
            let config = config_manager.load_blocking()?;

            let api_clients = ApiClients::new(&config)?;
            let state_manager = AppStateManager::new();

            // Register state
            app.manage(config_manager);
            app.manage(api_clients);
            app.manage(state_manager);

            // Setup system tray
            setup_system_tray(app)?;

            // Setup global hotkey
            setup_global_hotkey(app)?;

            // Start connectivity monitor
            start_connectivity_monitor(app.handle());

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // Audio commands
            process_audio,
            synthesize_speech,
            stop_audio,
            // Chat commands
            send_message,
            process_voice_query,
            clear_conversation,
            get_conversation,
            // Config commands
            load_config,
            save_config,
            validate_api_keys,
            get_available_voices,
            // System commands
            check_connectivity,
            get_app_state,
            register_hotkey,
            toggle_window_visibility,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

---

## Configuration Management

### Configuration Structure

```rust
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    // API Keys (encrypted at rest)
    pub openai_api_key: String,
    pub elevenlabs_api_key: String,
    pub openwebui_api_key: String,

    // OpenWebUI Settings
    pub openwebui_base_url: String,
    pub openwebui_model_name: String,
    pub openwebui_knowledge_collection_id: Option<String>,

    // Voice Settings
    pub elevenlabs_voice_id: String,
    pub voice_stability: f32,  // 0.0 - 1.0
    pub voice_similarity_boost: f32,  // 0.0 - 1.0

    // Audio Settings
    pub microphone_device_id: Option<String>,
    pub speaker_device_id: Option<String>,
    pub audio_input_volume: f32,
    pub audio_output_volume: f32,

    // Conversation Settings
    pub max_conversation_history: usize,
    pub auto_clear_context_minutes: Option<u32>,

    // Hotkey Settings
    pub push_to_talk_hotkey: String,  // e.g., "Ctrl+Shift+Space"
    pub toggle_window_hotkey: String,

    // UI Settings
    pub theme: String,  // "light" | "dark" | "system"
    pub show_transcription: bool,
    pub show_timestamps: bool,

    // Timeout Settings (seconds)
    pub whisper_timeout: u64,
    pub openwebui_timeout: u64,
    pub elevenlabs_timeout: u64,

    // Privacy Settings
    pub enable_logging: bool,
    pub log_level: String,  // "error" | "warn" | "info" | "debug"
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            openai_api_key: String::new(),
            elevenlabs_api_key: String::new(),
            openwebui_api_key: String::new(),
            openwebui_base_url: "http://localhost:8080".to_string(),
            openwebui_model_name: String::new(),
            openwebui_knowledge_collection_id: None,
            elevenlabs_voice_id: "21m00Tcm4TlvDq8ikWAM".to_string(), // Rachel
            voice_stability: 0.5,
            voice_similarity_boost: 0.75,
            microphone_device_id: None,
            speaker_device_id: None,
            audio_input_volume: 1.0,
            audio_output_volume: 1.0,
            max_conversation_history: 10,
            auto_clear_context_minutes: Some(60),
            push_to_talk_hotkey: "Ctrl+Shift+Space".to_string(),
            toggle_window_hotkey: "Ctrl+Shift+M".to_string(),
            theme: "system".to_string(),
            show_transcription: true,
            show_timestamps: false,
            whisper_timeout: 30,
            openwebui_timeout: 60,
            elevenlabs_timeout: 45,
            enable_logging: true,
            log_level: "info".to_string(),
        }
    }
}
```

### Secure Storage Implementation

```rust
use keyring::Entry;
use aes_gcm::{
    aead::{Aead, KeyInit, OsRng},
    Aes256Gcm, Nonce,
};
use rand::RngCore;

pub struct ConfigManager {
    config_path: PathBuf,
    keyring_service: &'static str,
}

impl ConfigManager {
    pub fn new() -> Result<Self> {
        let config_dir = dirs::config_dir()
            .ok_or_else(|| anyhow!("Could not find config directory"))?
            .join("talk-to-cmac");

        fs::create_dir_all(&config_dir)?;

        Ok(Self {
            config_path: config_dir.join("config.json"),
            keyring_service: "talk-to-cmac",
        })
    }

    /// Load config from disk, decrypt API keys
    pub async fn load(&self) -> Result<AppConfig> {
        if !self.config_path.exists() {
            return Ok(AppConfig::default());
        }

        let config_json = fs::read_to_string(&self.config_path)?;
        let mut config: AppConfig = serde_json::from_str(&config_json)?;

        // Decrypt API keys from system keyring
        config.openai_api_key = self.get_secret("openai_api_key")
            .unwrap_or_default();
        config.elevenlabs_api_key = self.get_secret("elevenlabs_api_key")
            .unwrap_or_default();
        config.openwebui_api_key = self.get_secret("openwebui_api_key")
            .unwrap_or_default();

        Ok(config)
    }

    /// Save config to disk, encrypt API keys
    pub async fn save(&self, config: AppConfig) -> Result<()> {
        // Store API keys in system keyring
        self.set_secret("openai_api_key", &config.openai_api_key)?;
        self.set_secret("elevenlabs_api_key", &config.elevenlabs_api_key)?;
        self.set_secret("openwebui_api_key", &config.openwebui_api_key)?;

        // Save config without API keys
        let mut config_to_save = config.clone();
        config_to_save.openai_api_key = String::new();
        config_to_save.elevenlabs_api_key = String::new();
        config_to_save.openwebui_api_key = String::new();

        let config_json = serde_json::to_string_pretty(&config_to_save)?;
        fs::write(&self.config_path, config_json)?;

        Ok(())
    }

    fn get_secret(&self, key: &str) -> Result<String> {
        let entry = Entry::new(self.keyring_service, key)?;
        let value = entry.get_password()
            .map_err(|e| anyhow!("Failed to get secret {}: {}", key, e))?;
        Ok(value)
    }

    fn set_secret(&self, key: &str, value: &str) -> Result<()> {
        let entry = Entry::new(self.keyring_service, key)?;
        entry.set_password(value)
            .map_err(|e| anyhow!("Failed to set secret {}: {}", key, e))?;
        Ok(())
    }
}
```

### Environment Variable Support

```rust
use dotenv::dotenv;

pub fn load_config_from_env() -> Result<AppConfig> {
    dotenv().ok();

    let mut config = AppConfig::default();

    if let Ok(key) = env::var("OPENAI_API_KEY") {
        config.openai_api_key = key;
    }
    if let Ok(key) = env::var("ELEVENLABS_API_KEY") {
        config.elevenlabs_api_key = key;
    }
    if let Ok(key) = env::var("OPENWEBUI_API_KEY") {
        config.openwebui_api_key = key;
    }
    if let Ok(url) = env::var("OPENWEBUI_BASE_URL") {
        config.openwebui_base_url = url;
    }
    if let Ok(model) = env::var("OPENWEBUI_MODEL_NAME") {
        config.openwebui_model_name = model;
    }
    if let Ok(cid) = env::var("OPENWEBUI_KNOWLEDGE_COLLECTION_ID") {
        config.openwebui_knowledge_collection_id = Some(cid);
    }

    Ok(config)
}
```

---

## Error Handling Strategy

### Error Type Hierarchy

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Audio capture failed: {0}")]
    AudioCapture(String),

    #[error("Audio format conversion failed: {0}")]
    AudioConversion(String),

    #[error("Whisper API error: {0}")]
    WhisperApi(#[from] WhisperError),

    #[error("OpenWebUI API error: {0}")]
    OpenWebUIApi(#[from] OpenWebUIError),

    #[error("ElevenLabs API error: {0}")]
    ElevenLabsApi(#[from] ElevenLabsError),

    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("State management error: {0}")]
    State(String),

    #[error("Timeout: {0}")]
    Timeout(String),

    #[error("Offline: No network connection")]
    Offline,

    #[error("Invalid API key: {0}")]
    InvalidApiKey(String),

    #[error("Internal error: {0}")]
    Internal(String),
}

#[derive(Error, Debug)]
pub enum WhisperError {
    #[error("Invalid audio format")]
    InvalidFormat,

    #[error("Audio file too large (max 25MB)")]
    FileTooLarge,

    #[error("Transcription failed: {0}")]
    TranscriptionFailed(String),

    #[error("Rate limit exceeded")]
    RateLimit,
}

#[derive(Error, Debug)]
pub enum OpenWebUIError {
    #[error("Model not found: {0}")]
    ModelNotFound(String),

    #[error("Authentication failed")]
    AuthFailed,

    #[error("Request failed: {0}")]
    RequestFailed(String),

    #[error("Invalid response format")]
    InvalidResponse,
}

#[derive(Error, Debug)]
pub enum ElevenLabsError {
    #[error("Voice not found: {0}")]
    VoiceNotFound(String),

    #[error("Text too long (max 5000 characters)")]
    TextTooLong,

    #[error("Quota exceeded")]
    QuotaExceeded,

    #[error("Synthesis failed: {0}")]
    SynthesisFailed(String),
}
```

### Error Recovery Flowchart

```
┌────────────────────────────────────────────────────────────┐
│                     ERROR OCCURS                           │
└──────────────────────────┬─────────────────────────────────┘
                           │
                    ┌──────▼──────┐
                    │ Error Type? │
                    └──────┬──────┘
         ┌─────────────────┼─────────────────┐
         │                 │                 │
    ┌────▼────┐      ┌─────▼─────┐     ┌────▼────┐
    │ Network │      │ API Error │     │ Client  │
    │ Error   │      │           │     │ Error   │
    └────┬────┘      └─────┬─────┘     └────┬────┘
         │                 │                 │
    ┌────▼────┐      ┌─────▼─────┐     ┌────▼────┐
    │ Check   │      │ Check     │     │ Show    │
    │ Online  │      │ Status    │     │ Error   │
    │ Status  │      │ Code      │     │ Message │
    └────┬────┘      └─────┬─────┘     └────┬────┘
         │                 │                 │
    ┌────▼────┐            │            ┌────▼────┐
    │ Online? │            │            │ Return  │
    └────┬────┘            │            │ to Idle │
      Yes│  │No            │            └─────────┘
         │  │              │
         │  └──────┐  ┌────▼────┐
         │         │  │ 401/403 │ Auth Error
         │         │  └────┬────┘
         │         │       │
         │         │  ┌────▼────────────┐
         │         │  │ Show "Invalid   │
         │         │  │ API Key" + Link │
         │         │  │ to Settings     │
         │         │  └─────────────────┘
         │         │
         │    ┌────▼────┐
         │    │ Show    │
         │    │"Offline"│
         │    │ Status  │
         │    └────┬────┘
         │         │
         │    ┌────▼───────┐
         │    │ Start Retry│
         │    │ Timer (5s) │
         │    └────┬───────┘
         │         │
         │    ┌────▼──────┐
         │    │ Re-check  │
         │    │ Connection│
         │    └───────────┘
         │
    ┌────▼────┐
    │ 429/503 │ Rate Limit / Service Down
    └────┬────┘
         │
    ┌────▼────────────┐
    │ Retry with      │
    │ Exponential     │
    │ Backoff         │
    │ (1s, 2s, 4s)    │
    └────┬────────────┘
         │
    ┌────▼────┐
    │ Success?│
    └────┬────┘
      Yes│  │No
         │  │
         │  └──────┐
         │         │
    ┌────▼────┐ ┌──▼────────┐
    │Continue │ │Show Error │
    │Pipeline │ │+ Suggest  │
    └─────────┘ │Retry      │
                └───────────┘
```

### Retry Logic Implementation

```rust
use tokio::time::{sleep, Duration};

pub struct RetryConfig {
    max_attempts: u32,
    initial_delay: Duration,
    max_delay: Duration,
    backoff_multiplier: f32,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_attempts: 3,
            initial_delay: Duration::from_secs(1),
            max_delay: Duration::from_secs(10),
            backoff_multiplier: 2.0,
        }
    }
}

pub async fn retry_with_backoff<F, Fut, T, E>(
    config: RetryConfig,
    mut operation: F,
) -> Result<T, E>
where
    F: FnMut() -> Fut,
    Fut: std::future::Future<Output = Result<T, E>>,
    E: std::fmt::Display,
{
    let mut attempts = 0;
    let mut delay = config.initial_delay;

    loop {
        match operation().await {
            Ok(result) => return Ok(result),
            Err(e) => {
                attempts += 1;

                if attempts >= config.max_attempts {
                    return Err(e);
                }

                warn!("Attempt {} failed: {}. Retrying in {:?}...",
                      attempts, e, delay);

                sleep(delay).await;

                // Exponential backoff
                delay = Duration::from_secs_f32(
                    (delay.as_secs_f32() * config.backoff_multiplier)
                        .min(config.max_delay.as_secs_f32())
                );
            }
        }
    }
}

// Usage example
async fn transcribe_with_retry(client: &WhisperClient, audio: Vec<u8>)
    -> Result<String, AppError> {

    retry_with_backoff(
        RetryConfig::default(),
        || async {
            client.transcribe(audio.clone())
                .await
                .map_err(|e| AppError::WhisperApi(e))
        }
    ).await
}
```

### Graceful Degradation

```rust
/// Fallback behavior when services fail
pub async fn process_query_with_fallback(
    audio_data: Vec<u8>,
    state: &AppStateManager,
    clients: &ApiClients,
) -> Result<QueryResult, AppError> {

    // Step 1: Transcription
    let transcription = match clients.whisper.transcribe(audio_data).await {
        Ok(text) => text,
        Err(e) => {
            // Fallback: Show error, allow manual text input
            return Ok(QueryResult::TranscriptionFailed {
                error: format!("Could not transcribe audio: {}", e),
                allow_text_input: true,
            });
        }
    };

    // Step 2: AI Response
    let ai_response = match clients.openwebui.query(&transcription).await {
        Ok(response) => response,
        Err(e) => {
            // Fallback: Show error with transcription
            return Ok(QueryResult::AIFailed {
                transcription,
                error: format!("AI assistant unavailable: {}", e),
            });
        }
    };

    // Step 3: TTS
    let audio = match clients.elevenlabs.synthesize(&ai_response).await {
        Ok(audio) => Some(audio),
        Err(e) => {
            // Fallback: Display text without voice
            warn!("TTS failed: {}. Displaying text only.", e);
            None
        }
    };

    Ok(QueryResult::Success {
        transcription,
        ai_response,
        audio,
    })
}

pub enum QueryResult {
    Success {
        transcription: String,
        ai_response: String,
        audio: Option<Vec<u8>>,
    },
    TranscriptionFailed {
        error: String,
        allow_text_input: bool,
    },
    AIFailed {
        transcription: String,
        error: String,
    },
}
```

### User-Facing Error Messages

```rust
pub fn user_friendly_error(error: &AppError) -> String {
    match error {
        AppError::Offline => {
            "You're offline. Please check your internet connection.".to_string()
        },
        AppError::WhisperApi(WhisperError::InvalidFormat) => {
            "Audio format not supported. Please try again.".to_string()
        },
        AppError::WhisperApi(WhisperError::RateLimit) => {
            "Too many requests. Please wait a moment and try again.".to_string()
        },
        AppError::OpenWebUIApi(OpenWebUIError::ModelNotFound(model)) => {
            format!("AI model '{}' not found. Please check your configuration.", model)
        },
        AppError::OpenWebUIApi(OpenWebUIError::AuthFailed) => {
            "Authentication failed. Please check your API key in Settings.".to_string()
        },
        AppError::ElevenLabsApi(ElevenLabsError::QuotaExceeded) => {
            "Voice synthesis quota exceeded. Response shown as text only.".to_string()
        },
        AppError::InvalidApiKey(service) => {
            format!("Invalid {} API key. Please update in Settings.", service)
        },
        AppError::Timeout(stage) => {
            format!("Request timed out during {}. Please try again.", stage)
        },
        _ => {
            "An unexpected error occurred. Please try again.".to_string()
        }
    }
}
```

---

## Security Considerations

### 1. API Key Management

**Storage Strategy**:
- **Never hardcode API keys** in source code
- Use system keyring for secure storage (Windows Credential Manager)
- Encrypt keys at rest using AES-256-GCM
- Keys only in memory during runtime

**Access Control**:
```rust
// Keys never exposed to frontend
// Only backend commands can access keys
pub struct SecureApiClients {
    whisper: WhisperClient,
    openwebui: OpenWebUIClient,
    elevenlabs: ElevenLabsClient,
}

// Frontend cannot directly access API keys
// All API calls proxied through Rust backend
```

**Key Rotation**:
```rust
#[tauri::command]
async fn update_api_key(
    service: String,
    new_key: String,
    config_manager: State<'_, ConfigManager>,
) -> Result<(), String> {
    // Validate key before saving
    match service.as_str() {
        "whisper" => validate_openai_key(&new_key).await?,
        "elevenlabs" => validate_elevenlabs_key(&new_key).await?,
        "openwebui" => validate_openwebui_key(&new_key).await?,
        _ => return Err("Unknown service".to_string()),
    }

    // Save to keyring
    config_manager.set_secret(&service, &new_key)
        .map_err(|e| e.to_string())?;

    Ok(())
}
```

### 2. Network Security

**HTTPS Only**:
```rust
fn create_http_client() -> Result<reqwest::Client> {
    reqwest::Client::builder()
        .https_only(true)  // Reject HTTP connections
        .timeout(Duration::from_secs(30))
        .build()
        .map_err(|e| anyhow!("Failed to create HTTP client: {}", e))
}
```

**Certificate Validation**:
```rust
// Use system certificate store
// Never disable certificate validation in production
let client = reqwest::Client::builder()
    .use_rustls_tls()  // Use Rust TLS for better security
    .build()?;
```

**Request Headers**:
```rust
// Minimal information disclosure
let response = client
    .post(url)
    .header("User-Agent", "TalkToCMAC/1.0")
    .header("Content-Type", "application/json")
    .header("Authorization", format!("Bearer {}", api_key))
    .send()
    .await?;
```

### 3. Data Privacy

**No Persistent Storage of Conversations**:
```rust
// Conversations only in memory
// Cleared on app restart
pub struct EphemeralConversationContext {
    messages: Vec<Message>,
    // NO file I/O
    // NO database writes
}

impl Drop for EphemeralConversationContext {
    fn drop(&mut self) {
        // Explicitly clear sensitive data
        self.messages.clear();
        info!("Conversation context cleared from memory");
    }
}
```

**Minimal Logging**:
```rust
// Log structure, not content
info!("Transcription completed: {} characters", text.len());
// NOT: info!("Transcription: {}", text);

// Exclude sensitive data from logs
debug!("API request to {}: {} bytes", endpoint, payload.len());
// NOT: debug!("API request payload: {:?}", payload);
```

**Secure Memory Handling**:
```rust
use zeroize::Zeroize;

pub struct SecureString(String);

impl Drop for SecureString {
    fn drop(&mut self) {
        self.0.zeroize();  // Clear memory on drop
    }
}
```

### 4. Input Validation

**Sanitize User Input**:
```rust
pub fn sanitize_text_input(input: &str) -> Result<String, AppError> {
    // Max length check
    if input.len() > 5000 {
        return Err(AppError::InvalidInput("Text too long".to_string()));
    }

    // Remove control characters
    let sanitized: String = input
        .chars()
        .filter(|c| !c.is_control() || c.is_whitespace())
        .collect();

    // Trim whitespace
    Ok(sanitized.trim().to_string())
}
```

**Audio Validation**:
```rust
pub fn validate_audio_data(data: &[u8]) -> Result<(), AppError> {
    // Size check (max 25MB for Whisper)
    const MAX_SIZE: usize = 25 * 1024 * 1024;
    if data.len() > MAX_SIZE {
        return Err(AppError::AudioCapture(
            "Audio file too large".to_string()
        ));
    }

    // Format check (basic magic number validation)
    if data.len() < 4 {
        return Err(AppError::AudioCapture(
            "Invalid audio data".to_string()
        ));
    }

    Ok(())
}
```

### 5. Code Injection Prevention

**No Dynamic Code Execution**:
```rust
// Never use eval() or similar
// All code paths statically defined
// No user input in format strings

// Safe:
let message = format!("User said: {}", user_input);

// Unsafe (never do this):
// eval(user_input)
```

**Parameterized API Requests**:
```rust
// Always use structured JSON, never string interpolation
let body = json!({
    "text": user_input,  // Safely serialized
    "model": model_id,
});

// NOT:
// let body = format!("{{\"text\":\"{}\"}}", user_input);  // Dangerous!
```

### 6. Update Security

**Future Consideration** (out of scope for v1):
```rust
// Use tauri-plugin-updater for secure updates
// Verify signatures before installing
// HTTPS-only update checks
// No auto-install without user consent
```

---

## Recommended Rust Crates

### Core Dependencies

**Tauri Ecosystem**:
```toml
[dependencies]
tauri = { version = "2.0", features = ["protocol-asset"] }
tauri-plugin-global-shortcut = "2.0"
tauri-plugin-notification = "2.0"
tauri-plugin-dialog = "2.0"
```

**Async Runtime**:
```toml
tokio = { version = "1.35", features = ["full"] }
async-trait = "0.1"
futures = "0.3"
```

**HTTP Client**:
```toml
reqwest = { version = "0.11", features = [
    "json",
    "multipart",
    "rustls-tls",  # More secure than native TLS
    "stream",
] }
```

**Serialization**:
```toml
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
base64 = "0.21"
```

**Error Handling**:
```toml
thiserror = "1.0"
anyhow = "1.0"
```

**Cryptography & Security**:
```toml
keyring = "2.2"  # System keyring access
aes-gcm = "0.10"  # AES-256-GCM encryption
rand = "0.8"
zeroize = "1.6"  # Secure memory clearing
```

**Audio Processing** (optional):
```toml
hound = "3.5"  # WAV file I/O
rodio = "0.17"  # Audio playback (if not using browser)
cpal = "0.15"  # Cross-platform audio I/O
```

**Configuration**:
```toml
dirs = "5.0"  # Cross-platform config directories
dotenv = "0.15"  # .env file support
toml = "0.8"  # Alternative to JSON for config
```

**Logging**:
```toml
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
tracing-appender = "0.2"
```

**Date/Time**:
```toml
chrono = { version = "0.4", features = ["serde"] }
```

**UUID Generation**:
```toml
uuid = { version = "1.6", features = ["v4", "serde"] }
```

**Complete Cargo.toml Example**:
```toml
[package]
name = "talk-to-cmac"
version = "0.1.0"
edition = "2021"

[dependencies]
# Tauri
tauri = { version = "2.0", features = ["protocol-asset"] }
tauri-plugin-global-shortcut = "2.0"
tauri-plugin-notification = "2.0"

# Async
tokio = { version = "1.35", features = ["full"] }
async-trait = "0.1"
futures = "0.3"

# HTTP
reqwest = { version = "0.11", features = ["json", "multipart", "rustls-tls", "stream"] }

# Serialization
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
base64 = "0.21"

# Error handling
thiserror = "1.0"
anyhow = "1.0"

# Security
keyring = "2.2"
aes-gcm = "0.10"
rand = "0.8"
zeroize = "1.6"

# Configuration
dirs = "5.0"
dotenv = "0.15"

# Logging
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }

# Utils
chrono = { version = "0.4", features = ["serde"] }
uuid = { version = "1.6", features = ["v4"] }

[build-dependencies]
tauri-build = "2.0"
```

---

## Implementation Roadmap

### Phase 1: Foundation (Week 1-2)

**Goals**: Basic Tauri app with configuration

- [ ] Initialize Tauri project
- [ ] Setup project structure
- [ ] Implement `ConfigManager` with secure storage
- [ ] Create basic UI (system tray + main window)
- [ ] Test configuration loading/saving

**Deliverables**:
- Working Tauri app that starts
- Settings UI for API keys
- Successful config persistence

### Phase 2: API Integration (Week 3-4)

**Goals**: Connect to external services

- [ ] Implement `WhisperClient`
- [ ] Implement `OpenWebUIClient`
- [ ] Implement `ElevenLabsClient`
- [ ] Add connection health checks
- [ ] Test each API independently

**Deliverables**:
- All three API clients functional
- API validation in settings UI
- Error handling for API failures

### Phase 3: Audio Pipeline (Week 5-6)

**Goals**: Voice input/output working

- [ ] Frontend audio capture (MediaRecorder)
- [ ] Tauri command for audio processing
- [ ] Audio format handling
- [ ] TTS playback in browser
- [ ] Interrupt/stop functionality

**Deliverables**:
- Working voice input
- Working voice output
- Ability to stop playback

### Phase 4: State Management (Week 7)

**Goals**: Application state coordination

- [ ] Implement `AppStateManager`
- [ ] State machine transitions
- [ ] Conversation context management
- [ ] State synchronization with UI

**Deliverables**:
- State visible in UI (idle/listening/thinking/speaking)
- Conversation history maintained during session
- Context cleared on restart

### Phase 5: Integration & Polish (Week 8-9)

**Goals**: Complete voice pipeline

- [ ] `process_voice_query` command (full pipeline)
- [ ] Global hotkey registration
- [ ] Push-to-talk implementation
- [ ] Visual feedback for all states
- [ ] Error messages in UI

**Deliverables**:
- End-to-end voice query working
- Hotkey functional
- Professional error handling

### Phase 6: Testing & Optimization (Week 10)

**Goals**: Production-ready application

- [ ] Offline mode handling
- [ ] Timeout configuration
- [ ] Retry logic testing
- [ ] Memory leak checks
- [ ] Performance profiling

**Deliverables**:
- Stable application
- Graceful offline behavior
- No memory leaks

### Phase 7: Distribution (Week 11)

**Goals**: Installable package

- [ ] Configure Tauri bundler
- [ ] Create MSI installer
- [ ] Add application icon
- [ ] Test installation on clean Windows
- [ ] Document installation process

**Deliverables**:
- MSI installer
- Installation guide
- Uninstallation working

### Phase 8: Future Enhancements (Post-launch)

**Optional features**:
- [ ] Streaming TTS for faster response
- [ ] Local Whisper fallback
- [ ] Voice activity detection (VAD)
- [ ] Custom voice training
- [ ] Multi-language support
- [ ] Auto-update mechanism
- [ ] Usage analytics (local only)

---

## Summary

This architecture provides:

1. **Clear separation of concerns**: Frontend handles UI, Rust handles business logic and external integrations
2. **Secure API key management**: System keyring + encryption at rest
3. **Robust error handling**: Retry logic, graceful degradation, user-friendly messages
4. **Ephemeral conversations**: Privacy-focused, no persistent storage
5. **Scalable state management**: Thread-safe, mutex-protected shared state
6. **Full voice pipeline**: Mic → Whisper → OpenWebUI → ElevenLabs → Speaker
7. **Offline resilience**: Network detection, clear status indicators
8. **Production-ready**: Proper logging, timeout handling, input validation

**Key Files to Create**:
- `/src-tauri/src/main.rs` - Main Tauri setup
- `/src-tauri/src/commands.rs` - Tauri command handlers
- `/src-tauri/src/state.rs` - State management
- `/src-tauri/src/api/whisper.rs` - Whisper client
- `/src-tauri/src/api/openwebui.rs` - OpenWebUI client
- `/src-tauri/src/api/elevenlabs.rs` - ElevenLabs client
- `/src-tauri/src/config.rs` - Configuration manager
- `/src-tauri/src/error.rs` - Error types
- `/src-tauri/src/audio.rs` - Audio pipeline
- `/src/App.tsx` - Frontend React app (or Vue/Svelte)

**Next Steps**:
1. Initialize Tauri project: `npm create tauri-app`
2. Set up Rust project structure
3. Install dependencies from Cargo.toml
4. Begin Phase 1 implementation

This architecture is implementation-ready and provides a complete blueprint for the development team.
