/**
 * Type definitions for Talk to CMAC Voice Assistant
 */

// ============================================================================
// App State
// ============================================================================

export type AppStatus =
  | 'idle'
  | 'recording'
  | 'listening'
  | 'transcribing'
  | 'thinking'
  | 'speaking'
  | { error: { message: string } };

export interface AppStateResponse {
  status: AppStatus;
  message_count: number;
  connectivity: ConnectivityStatus;
}

// ============================================================================
// Service Connectivity
// ============================================================================

export type ServiceStatus =
  | 'connected'
  | 'checking'
  | { disconnected: { reason: string } }
  | 'unknown';

export interface ConnectivityStatus {
  whisper: ServiceStatus;
  openwebui: ServiceStatus;
  elevenlabs: ServiceStatus;
  last_checked: number;
}

export interface ConnectivityResponse {
  whisper: ServiceStatus;
  openwebui: ServiceStatus;
  elevenlabs: ServiceStatus;
}

// ============================================================================
// Messages & Conversation
// ============================================================================

export type MessageRole = 'user' | 'assistant' | 'system';

export interface Message {
  role: MessageRole;
  content: string;
  timestamp: number;
  audioData?: Uint8Array; // For assistant messages with audio
}

export interface ConversationContext {
  id: string;
  messages: Message[];
  max_messages: number;
  started_at: number;
  updated_at: number;
}

// ============================================================================
// Voice & Audio
// ============================================================================

export interface VoiceSettings {
  stability: number; // 0.0 - 1.0
  similarity_boost: number; // 0.0 - 1.0
  style?: number; // 0.0 - 1.0
  use_speaker_boost: boolean;
}

export interface Voice {
  voice_id: string;
  name: string;
  category?: string;
  labels?: Record<string, string>;
}

export interface VoiceQueryResponse {
  transcription: string;
  llm_response: string;
  audio_response: number[];
}

// ============================================================================
// Configuration
// ============================================================================

export interface WhisperConfig {
  endpoint: string;
  model: string;
  language?: string;
  temperature: number;
  timeout_secs: number;
}

export interface OpenWebUIConfig {
  endpoint: string;
  model: string;
  max_context_length: number;
  temperature: number;
  max_tokens?: number;
  stream: boolean;
  timeout_secs: number;
}

export interface ElevenLabsConfig {
  endpoint: string;
  voice_id: string;
  model_id: string;
  voice_settings: VoiceSettings;
  timeout_secs: number;
}

export interface AudioConfig {
  sample_rate: number;
  bit_depth: number;
  channels: number;
  format: string;
  silence_threshold: number;
  silence_duration: number;
  max_duration: number;
}

export interface UIConfig {
  theme: string;
  show_transcription: boolean;
  show_thinking: boolean;
  auto_minimize: boolean;
  always_on_top: boolean;
  global_hotkey?: string;
}

export interface AppConfig {
  whisper: WhisperConfig;
  openwebui: OpenWebUIConfig;
  elevenlabs: ElevenLabsConfig;
  audio: AudioConfig;
  ui: UIConfig;
}

// ============================================================================
// UI State
// ============================================================================

export interface ErrorState {
  message: string;
  timestamp: number;
  source?: string;
}

export type ThemeMode = 'light' | 'dark' | 'auto';

// ============================================================================
// Tauri Command Types
// ============================================================================

export type ApiService = 'whisper' | 'openwebui' | 'elevenlabs';

// ============================================================================
// Component Props
// ============================================================================

export interface MessageBubbleProps {
  message: Message;
  isUser: boolean;
  onPlayAudio?: (audioData: Uint8Array) => void;
}

export interface StatusIndicatorProps {
  status: AppStatus;
}

export interface ConnectionStatusProps {
  connectivity: ConnectivityStatus;
}

export interface ErrorMessageProps {
  error: ErrorState | null;
  onDismiss: () => void;
}

export interface MicrophoneButtonProps {
  isRecording: boolean;
  isProcessing: boolean;
  onStartRecording: () => void;
  onStopRecording: () => void;
  disabled?: boolean;
}

export interface SettingsPanelProps {
  isOpen: boolean;
  onClose: () => void;
  config: AppConfig;
  onSave: (config: AppConfig) => Promise<void>;
}

// ============================================================================
// Utility Types
// ============================================================================

export type AsyncResult<T> = {
  data: T | null;
  error: string | null;
  loading: boolean;
};

export type AudioRecorderState = {
  isRecording: boolean;
  isPaused: boolean;
  duration: number;
  error: string | null;
};

export type AudioPlayerState = {
  isPlaying: boolean;
  currentTime: number;
  duration: number;
  error: string | null;
};
