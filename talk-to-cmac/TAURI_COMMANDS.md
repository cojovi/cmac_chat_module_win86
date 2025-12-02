# Tauri Commands Reference

Quick reference for all Tauri commands available in the Talk to CMAC backend.

## Audio Processing

### process_audio

Transcribe audio to text using Whisper API.

**TypeScript:**
```typescript
import { invoke } from '@tauri-apps/api/core';

const transcription = await invoke<string>('process_audio', {
  audioData: Uint8Array,  // 16kHz WAV audio bytes
  filename: string        // e.g., "recording.wav"
});
```

**Status Updates:** `Idle` → `Transcribing` → `Idle`

### synthesize_speech

Convert text to speech using ElevenLabs API.

**TypeScript:**
```typescript
const audioBytes = await invoke<number[]>('synthesize_speech', {
  text: string  // Max 5000 characters
});

// Convert to Uint8Array for playback
const audio = new Uint8Array(audioBytes);
```

**Status Updates:** `Idle` → `Speaking` → `Idle`

## LLM Interaction

### send_message

Send a text message to the LLM and get a response.

**TypeScript:**
```typescript
const response = await invoke<string>('send_message', {
  message: string
});
```

**Status Updates:** `Idle` → `Thinking` → `Idle`

**Note:** Automatically includes conversation context.

## Complete Pipeline

### process_voice_query

Process complete voice assistant pipeline (audio → text → LLM → speech).

**TypeScript:**
```typescript
interface VoiceQueryResponse {
  transcription: string;
  llm_response: string;
  audio_response: number[];
}

const result = await invoke<VoiceQueryResponse>('process_voice_query', {
  audioData: Uint8Array,
  filename: string
});

console.log('You said:', result.transcription);
console.log('CMAC says:', result.llm_response);
// Play result.audio_response
```

**Status Updates:** `Idle` → `Transcribing` → `Thinking` → `Speaking` → `Idle`

## Configuration Management

### load_config

Load current application configuration.

**TypeScript:**
```typescript
interface AppConfig {
  whisper: WhisperConfig;
  openwebui: OpenWebUiConfig;
  elevenlabs: ElevenLabsConfig;
  audio: AudioConfig;
  ui: UiConfig;
}

const config = await invoke<AppConfig>('load_config');
```

### save_config

Save application configuration.

**TypeScript:**
```typescript
await invoke('save_config', {
  config: AppConfig
});
```

### update_api_key

Store an API key securely in the system keyring.

**TypeScript:**
```typescript
await invoke('update_api_key', {
  service: 'whisper' | 'openwebui' | 'elevenlabs',
  apiKey: string
});
```

## State Management

### get_app_state

Get current application state.

**TypeScript:**
```typescript
interface AppStateResponse {
  status: 'idle' | 'recording' | 'listening' | 'transcribing' | 'thinking' | 'speaking' | { error: { message: string } };
  message_count: number;
  connectivity: {
    whisper: ServiceStatus;
    openwebui: ServiceStatus;
    elevenlabs: ServiceStatus;
    last_checked: number;
  };
}

type ServiceStatus =
  | 'connected'
  | 'checking'
  | { disconnected: { reason: string } }
  | 'unknown';

const state = await invoke<AppStateResponse>('get_app_state');
```

### get_conversation

Get conversation history.

**TypeScript:**
```typescript
interface Message {
  role: 'user' | 'assistant' | 'system';
  content: string;
  timestamp: number;
}

interface ConversationContext {
  id: string;
  messages: Message[];
  max_messages: number;
  started_at: number;
  updated_at: number;
}

const conversation = await invoke<ConversationContext>('get_conversation');
```

### clear_conversation

Clear conversation history and start fresh.

**TypeScript:**
```typescript
await invoke('clear_conversation');
```

## Connectivity

### check_connectivity

Check connectivity to all services.

**TypeScript:**
```typescript
interface ConnectivityResponse {
  whisper: ServiceStatus;
  openwebui: ServiceStatus;
  elevenlabs: ServiceStatus;
}

const status = await invoke<ConnectivityResponse>('check_connectivity');

if (status.whisper === 'connected') {
  console.log('Whisper is ready');
}
```

## Voice Management

### list_voices

List available ElevenLabs voices.

**TypeScript:**
```typescript
interface Voice {
  voice_id: string;
  name: string;
  category?: string;
  labels?: Record<string, string>;
}

const voices = await invoke<Voice[]>('list_voices');

voices.forEach(voice => {
  console.log(`${voice.name}: ${voice.voice_id}`);
});
```

### update_voice_settings

Update ElevenLabs voice synthesis settings.

**TypeScript:**
```typescript
interface VoiceSettings {
  stability: number;          // 0.0 - 1.0
  similarity_boost: number;   // 0.0 - 1.0
  style?: number;             // 0.0 - 1.0
  use_speaker_boost: boolean;
}

await invoke('update_voice_settings', {
  settings: {
    stability: 0.5,
    similarity_boost: 0.75,
    style: 0.0,
    use_speaker_boost: true
  }
});
```

## Error Handling

All commands return `Promise<T>` and can throw errors:

```typescript
try {
  const result = await invoke<string>('process_audio', {
    audioData,
    filename
  });
} catch (error) {
  console.error('Transcription failed:', error);
  // Error is a string with the error message
}
```

## React Hook Example

```typescript
import { invoke } from '@tauri-apps/api/core';
import { useState } from 'react';

function useVoiceAssistant() {
  const [status, setStatus] = useState<string>('idle');
  const [error, setError] = useState<string | null>(null);

  const processVoiceQuery = async (audioData: Uint8Array) => {
    try {
      setStatus('processing');
      setError(null);

      const result = await invoke<VoiceQueryResponse>('process_voice_query', {
        audioData: Array.from(audioData),
        filename: 'recording.wav'
      });

      setStatus('idle');
      return result;
    } catch (err) {
      setError(err as string);
      setStatus('error');
      throw err;
    }
  };

  return { processVoiceQuery, status, error };
}
```

## Vue Composable Example

```typescript
import { invoke } from '@tauri-apps/api/core';
import { ref } from 'vue';

export function useVoiceAssistant() {
  const status = ref<string>('idle');
  const error = ref<string | null>(null);

  const processVoiceQuery = async (audioData: Uint8Array) => {
    try {
      status.value = 'processing';
      error.value = null;

      const result = await invoke<VoiceQueryResponse>('process_voice_query', {
        audioData: Array.from(audioData),
        filename: 'recording.wav'
      });

      status.value = 'idle';
      return result;
    } catch (err) {
      error.value = err as string;
      status.value = 'error';
      throw err;
    }
  };

  return { processVoiceQuery, status, error };
}
```

## Complete Usage Example

```typescript
import { invoke } from '@tauri-apps/api/core';

class VoiceAssistant {
  async initialize() {
    // Load configuration
    const config = await invoke<AppConfig>('load_config');
    console.log('Config loaded:', config);

    // Check connectivity
    const connectivity = await invoke<ConnectivityResponse>('check_connectivity');
    console.log('Services status:', connectivity);

    // List available voices
    const voices = await invoke<Voice[]>('list_voices');
    console.log('Available voices:', voices);
  }

  async processQuery(audioData: Uint8Array) {
    // Process complete voice query
    const result = await invoke<VoiceQueryResponse>('process_voice_query', {
      audioData: Array.from(audioData),
      filename: 'query.wav'
    });

    console.log('Transcription:', result.transcription);
    console.log('Response:', result.llm_response);

    // Play audio response
    this.playAudio(new Uint8Array(result.audio_response));

    return result;
  }

  async sendTextMessage(message: string) {
    const response = await invoke<string>('send_message', {
      message
    });

    console.log('Response:', response);

    // Optionally convert to speech
    const audio = await invoke<number[]>('synthesize_speech', {
      text: response
    });

    this.playAudio(new Uint8Array(audio));

    return response;
  }

  async clearHistory() {
    await invoke('clear_conversation');
    console.log('Conversation cleared');
  }

  playAudio(audioData: Uint8Array) {
    const blob = new Blob([audioData], { type: 'audio/mpeg' });
    const url = URL.createObjectURL(blob);
    const audio = new Audio(url);
    audio.play();
  }
}

// Usage
const assistant = new VoiceAssistant();
await assistant.initialize();
```

## TypeScript Type Definitions

Create a `types/tauri.d.ts` file:

```typescript
declare module '@tauri-apps/api/core' {
  export function invoke<T>(cmd: string, args?: Record<string, any>): Promise<T>;
}

interface AppConfig {
  whisper: {
    endpoint: string;
    model: string;
    language?: string;
    temperature: number;
    timeout_secs: number;
  };
  openwebui: {
    endpoint: string;
    model: string;
    max_context_length: number;
    temperature: number;
    max_tokens?: number;
    stream: boolean;
    timeout_secs: number;
  };
  elevenlabs: {
    endpoint: string;
    voice_id: string;
    model_id: string;
    voice_settings: VoiceSettings;
    timeout_secs: number;
  };
  audio: {
    sample_rate: number;
    bit_depth: number;
    channels: number;
    format: string;
    silence_threshold: number;
    silence_duration: number;
    max_duration: number;
  };
  ui: {
    theme: string;
    show_transcription: boolean;
    show_thinking: boolean;
    auto_minimize: boolean;
    always_on_top: boolean;
    global_hotkey?: string;
  };
}

interface VoiceSettings {
  stability: number;
  similarity_boost: number;
  style?: number;
  use_speaker_boost: boolean;
}

interface VoiceQueryResponse {
  transcription: string;
  llm_response: string;
  audio_response: number[];
}

interface Voice {
  voice_id: string;
  name: string;
  category?: string;
  labels?: Record<string, string>;
}

type ServiceStatus =
  | 'connected'
  | 'checking'
  | { disconnected: { reason: string } }
  | 'unknown';

interface ConnectivityResponse {
  whisper: ServiceStatus;
  openwebui: ServiceStatus;
  elevenlabs: ServiceStatus;
}
```

---

For implementation details, see [BACKEND_IMPLEMENTATION.md](./BACKEND_IMPLEMENTATION.md).
