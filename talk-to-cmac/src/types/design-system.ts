/**
 * Talk to CMAC - Design System Types
 * Type definitions for consistent component APIs
 */

/* ============================================
   APPLICATION STATES
   ============================================ */

export type AppState =
  | 'initializing'
  | 'idle'
  | 'listening'
  | 'transcribing'
  | 'thinking'
  | 'speaking'
  | 'error'
  | 'offline';

export type MessageType = 'user' | 'assistant' | 'system';

export type ThemeMode = 'light' | 'dark' | 'auto';

/* ============================================
   MESSAGE STRUCTURE
   ============================================ */

export interface Message {
  id: string;
  type: MessageType;
  content: string;
  timestamp: Date;
  audioUrl?: string;
  audioPlaying?: boolean;
  audioDuration?: number;
  error?: boolean;
}

/* ============================================
   STATUS INDICATOR
   ============================================ */

export interface StatusInfo {
  state: AppState;
  label: string;
  icon: StatusIcon;
  color: string;
  animated?: boolean;
}

export type StatusIcon =
  | 'ready'
  | 'listening'
  | 'transcribing'
  | 'thinking'
  | 'speaking'
  | 'error'
  | 'offline';

/* ============================================
   MICROPHONE BUTTON
   ============================================ */

export interface MicButtonState {
  state: 'idle' | 'listening' | 'disabled';
  recording: boolean;
  disabled: boolean;
}

export interface MicButtonProps {
  state: MicButtonState['state'];
  onClick: () => void;
  onMouseDown?: () => void;
  onMouseUp?: () => void;
  onTouchStart?: () => void;
  onTouchEnd?: () => void;
  disabled?: boolean;
  ariaLabel?: string;
}

/* ============================================
   MESSAGE BUBBLE
   ============================================ */

export interface MessageBubbleProps {
  message: Message;
  showTimestamp?: boolean;
  onAudioPlay?: (messageId: string) => void;
  onAudioPause?: (messageId: string) => void;
  onAudioStop?: (messageId: string) => void;
}

/* ============================================
   AUDIO CONTROLS
   ============================================ */

export interface AudioControlsProps {
  audioUrl: string;
  duration?: number;
  playing?: boolean;
  onPlay?: () => void;
  onPause?: () => void;
  onStop?: () => void;
  onEnded?: () => void;
}

export interface AudioPlaybackState {
  playing: boolean;
  currentTime: number;
  duration: number;
  loading: boolean;
}

/* ============================================
   SETTINGS
   ============================================ */

export interface Settings {
  apiKey: string;
  voiceModel: VoiceModel;
  pushToTalkKey: string;
  autoPlayAudio: boolean;
  darkMode: ThemeMode;
  connectionStatus: ConnectionStatus;
}

export type VoiceModel =
  | 'alloy'
  | 'echo'
  | 'fable'
  | 'onyx'
  | 'nova'
  | 'shimmer';

export interface ConnectionStatus {
  connected: boolean;
  responseTime?: number;
  lastChecked?: Date;
  error?: string;
}

export interface SettingsProps {
  settings: Settings;
  isOpen: boolean;
  onClose: () => void;
  onSave: (settings: Settings) => void;
  onTest: () => Promise<ConnectionStatus>;
}

/* ============================================
   INPUT AREA
   ============================================ */

export interface InputAreaProps {
  value: string;
  onChange: (value: string) => void;
  onSubmit: () => void;
  onMicClick: () => void;
  micState: MicButtonState['state'];
  disabled?: boolean;
  placeholder?: string;
}

/* ============================================
   CHAT WINDOW
   ============================================ */

export interface ChatWindowProps {
  messages: Message[];
  currentState: AppState;
  onSendMessage: (message: string) => void;
  onVoiceInput: () => void;
  onSettingsClick: () => void;
  settings: Settings;
}

/* ============================================
   STATUS BAR
   ============================================ */

export interface StatusBarProps {
  state: AppState;
  onSettingsClick: () => void;
}

/* ============================================
   ERROR HANDLING
   ============================================ */

export interface AppError {
  type: ErrorType;
  message: string;
  timestamp: Date;
  recoverable: boolean;
  retryAction?: () => void;
}

export type ErrorType =
  | 'connection'
  | 'api'
  | 'transcription'
  | 'tts'
  | 'microphone'
  | 'unknown';

export interface ErrorBannerProps {
  error: AppError;
  onDismiss: () => void;
  onRetry?: () => void;
}

/* ============================================
   ANIMATION UTILITIES
   ============================================ */

export type AnimationType =
  | 'pulse'
  | 'pulse-ring'
  | 'spin'
  | 'shake'
  | 'slide-in-left'
  | 'slide-in-right'
  | 'slide-up'
  | 'slide-down'
  | 'fade-in'
  | 'fade-out'
  | 'bounce'
  | 'glow';

export interface AnimationConfig {
  type: AnimationType;
  duration?: number;
  delay?: number;
  iterationCount?: number | 'infinite';
  timingFunction?: string;
}

/* ============================================
   KEYBOARD SHORTCUTS
   ============================================ */

export interface KeyboardShortcut {
  key: string;
  ctrlKey?: boolean;
  shiftKey?: boolean;
  altKey?: boolean;
  action: () => void;
  description: string;
}

export interface KeyboardShortcutsHelp {
  shortcuts: KeyboardShortcut[];
  isOpen: boolean;
  onClose: () => void;
}

/* ============================================
   WINDOW POSITIONING
   ============================================ */

export interface WindowPosition {
  x: number;
  y: number;
  monitor: number;
}

export interface WindowDimensions {
  width: number;
  height: number;
}

export type TaskbarPosition = 'bottom' | 'top' | 'left' | 'right';

/* ============================================
   API RESPONSE TYPES
   ============================================ */

export interface TranscriptionResponse {
  text: string;
  confidence?: number;
  language?: string;
  duration?: number;
}

export interface AIResponse {
  content: string;
  model?: string;
  tokens?: number;
  responseTime?: number;
}

export interface TTSResponse {
  audioUrl: string;
  duration?: number;
  format?: string;
}

/* ============================================
   EVENT HANDLERS
   ============================================ */

export interface AppEventHandlers {
  onStateChange: (state: AppState) => void;
  onMessageSent: (message: Message) => void;
  onMessageReceived: (message: Message) => void;
  onError: (error: AppError) => void;
  onSettingsChange: (settings: Settings) => void;
}

/* ============================================
   STORE STATE (ZUSTAND)
   ============================================ */

export interface AppStore {
  // State
  currentState: AppState;
  messages: Message[];
  settings: Settings;
  error: AppError | null;
  inputValue: string;

  // Actions
  setState: (state: AppState) => void;
  addMessage: (message: Omit<Message, 'id' | 'timestamp'>) => void;
  updateMessage: (id: string, updates: Partial<Message>) => void;
  clearMessages: () => void;
  setSettings: (settings: Partial<Settings>) => void;
  setError: (error: AppError | null) => void;
  setInputValue: (value: string) => void;

  // Audio playback
  playAudio: (messageId: string) => void;
  pauseAudio: (messageId: string) => void;
  stopAudio: (messageId: string) => void;

  // Async actions
  sendTextMessage: (text: string) => Promise<void>;
  sendVoiceMessage: (audioBlob: Blob) => Promise<void>;
  testConnection: () => Promise<ConnectionStatus>;
}

/* ============================================
   UTILITY TYPES
   ============================================ */

export type DeepPartial<T> = {
  [P in keyof T]?: T[P] extends object ? DeepPartial<T[P]> : T[P];
};

export type RequireAtLeastOne<T, Keys extends keyof T = keyof T> = Pick<
  T,
  Exclude<keyof T, Keys>
> &
  {
    [K in Keys]-?: Required<Pick<T, K>> & Partial<Pick<T, Exclude<Keys, K>>>;
  }[Keys];

export type Optional<T, K extends keyof T> = Omit<T, K> & Partial<Pick<T, K>>;

/* ============================================
   CONSTANTS
   ============================================ */

export const APP_STATES: Record<AppState, StatusInfo> = {
  initializing: {
    state: 'initializing',
    label: 'Initializing...',
    icon: 'ready',
    color: 'var(--text-secondary)',
    animated: true,
  },
  idle: {
    state: 'idle',
    label: 'Ready',
    icon: 'ready',
    color: 'var(--state-listening)',
    animated: false,
  },
  listening: {
    state: 'listening',
    label: 'Listening...',
    icon: 'listening',
    color: 'var(--state-listening)',
    animated: true,
  },
  transcribing: {
    state: 'transcribing',
    label: 'Transcribing...',
    icon: 'transcribing',
    color: 'var(--state-transcribing)',
    animated: true,
  },
  thinking: {
    state: 'thinking',
    label: 'Thinking...',
    icon: 'thinking',
    color: 'var(--state-thinking)',
    animated: true,
  },
  speaking: {
    state: 'speaking',
    label: 'Speaking...',
    icon: 'speaking',
    color: 'var(--state-speaking)',
    animated: true,
  },
  error: {
    state: 'error',
    label: 'Error',
    icon: 'error',
    color: 'var(--state-error)',
    animated: false,
  },
  offline: {
    state: 'offline',
    label: 'Offline',
    icon: 'offline',
    color: 'var(--state-offline)',
    animated: false,
  },
};

export const VOICE_MODELS: Array<{ value: VoiceModel; label: string }> = [
  { value: 'alloy', label: 'Alloy (Natural)' },
  { value: 'echo', label: 'Echo (Male)' },
  { value: 'fable', label: 'Fable (Expressive)' },
  { value: 'onyx', label: 'Onyx (Deep)' },
  { value: 'nova', label: 'Nova (Female)' },
  { value: 'shimmer', label: 'Shimmer (Warm)' },
];

export const DEFAULT_SETTINGS: Settings = {
  apiKey: '',
  voiceModel: 'alloy',
  pushToTalkKey: 'Ctrl+M',
  autoPlayAudio: true,
  darkMode: 'auto',
  connectionStatus: {
    connected: false,
  },
};

export const WINDOW_DIMENSIONS: WindowDimensions = {
  width: 420,
  height: 650,
};

export const MIN_WINDOW_DIMENSIONS: WindowDimensions = {
  width: 360,
  height: 400,
};

export const MAX_WINDOW_DIMENSIONS: WindowDimensions = {
  width: 500,
  height: 900,
};
