# Talk to CMAC - Frontend Documentation

React frontend for the Talk to CMAC voice assistant application built with Tauri 2.0.

## Architecture

### Tech Stack
- **React 19** - UI framework
- **TypeScript** - Type safety
- **Zustand** - State management
- **Tauri 2.0** - Desktop app framework
- **Vite** - Build tool

### Project Structure

```
src/
├── components/           # React components
│   ├── ChatWindow.tsx   # Main chat interface
│   ├── Header.tsx       # App header with status
│   ├── MessageList.tsx  # Conversation display
│   ├── MessageBubble.tsx # Individual messages
│   ├── InputArea.tsx    # Text input + mic
│   ├── MicrophoneButton.tsx # Voice recording button
│   ├── StatusIndicator.tsx  # App state display
│   ├── ConnectionStatus.tsx # Service connectivity
│   ├── ErrorMessage.tsx # Error handling
│   └── index.ts         # Component exports
│
├── hooks/               # Custom React hooks
│   ├── useAudioRecorder.ts # Recording functionality
│   ├── useAudioPlayer.ts   # Audio playback
│   ├── useTauri.ts         # Tauri integration
│   ├── useKeyboardShortcut.ts # Keyboard shortcuts
│   └── index.ts
│
├── store/               # State management
│   └── useAppStore.ts   # Zustand store
│
├── types/               # TypeScript definitions
│   └── index.ts         # Type exports
│
├── utils/               # Utility functions
│   ├── tauri.ts         # Tauri command wrappers
│   └── audio.ts         # Audio utilities
│
├── App.tsx              # Root component
├── App.css              # Global styles
└── main.tsx             # Entry point
```

## Components

### ChatWindow
Main application container that orchestrates all functionality.

**Features:**
- Integrates all child components
- Manages audio recording and playback
- Handles voice query processing
- Coordinates with Tauri backend

### Header
Application header with status indicators and controls.

**Features:**
- Displays app status (idle, recording, thinking, etc.)
- Shows service connectivity status
- Settings, clear chat, and refresh buttons

### MessageList
Scrollable conversation history.

**Features:**
- Auto-scroll to latest message
- Empty state for new conversations
- Filters system messages from display

### MessageBubble
Individual message display.

**Features:**
- Different styles for user vs assistant
- Timestamp display
- Audio playback button for assistant messages

### InputArea
Text input and microphone controls.

**Features:**
- Multi-line text input
- Send button
- Microphone button for voice input
- Keyboard shortcuts (Enter to send)

### MicrophoneButton
Animated push-to-talk button.

**Features:**
- Recording indicator with pulse animation
- Processing state display
- Visual feedback with expanding rings

### StatusIndicator
Current app state display.

**Features:**
- Animated state indicators
- Color-coded status
- Icon animations (pulse, spin, wave)

### ConnectionStatus
Service connectivity display.

**Features:**
- Shows status for Whisper, OpenWebUI, ElevenLabs
- Last checked timestamp
- Visual error indicators

### ErrorMessage
Error display with auto-dismiss.

**Features:**
- Auto-dismiss after 10 seconds
- Manual dismiss button
- Shows error source and timestamp

## State Management

### Zustand Store (`useAppStore`)

**State:**
```typescript
{
  status: AppStatus                    // Current app status
  isProcessing: boolean                // Processing indicator
  isRecording: boolean                 // Recording state
  messages: Message[]                  // Conversation history
  conversationId: string | null        // Conversation ID
  config: AppConfig | null             // App configuration
  connectivity: ConnectivityStatus     // Service status
  error: ErrorState | null             // Error state
  theme: ThemeMode                     // UI theme
  isSidebarOpen: boolean               // Sidebar visibility
  isSettingsOpen: boolean              // Settings panel
}
```

**Actions:**
- `setStatus(status)` - Update app status
- `addMessage(message)` - Add message to history
- `clearMessages()` - Clear conversation
- `setConfig(config)` - Update configuration
- `setConnectivity(status)` - Update connectivity
- `setError(error)` - Set error state
- `clearError()` - Clear error
- `toggleSettings()` - Toggle settings panel

## Custom Hooks

### useAudioRecorder
Handles microphone recording with browser MediaRecorder API.

**Returns:**
```typescript
{
  isRecording: boolean
  isPaused: boolean
  duration: number
  error: string | null
  startRecording: () => Promise<void>
  stopRecording: () => Promise<Uint8Array | null>
  pauseRecording: () => void
  resumeRecording: () => void
}
```

**Features:**
- Automatic audio format conversion to 16kHz WAV
- Max duration timer
- Pause/resume support
- Error handling

### useAudioPlayer
Handles audio playback.

**Returns:**
```typescript
{
  isPlaying: boolean
  currentTime: number
  duration: number
  error: string | null
  play: (audioData: Uint8Array) => Promise<void>
  stop: () => void
  pause: () => void
  resume: () => void
}
```

### useTauri
Integrates with Tauri backend commands.

**Returns:**
```typescript
{
  checkConnectivity: () => Promise<void>
  sendTextMessage: (message: string) => Promise<string>
  processVoiceQuery: (audioData: Uint8Array) => Promise<VoiceQueryResponse>
  saveConfiguration: (config: AppConfig) => Promise<void>
  updateApiKey: (service: ApiService, key: string) => Promise<void>
  clearConversation: () => Promise<void>
  listVoices: () => Promise<Voice[]>
  updateVoiceSettings: (settings: VoiceSettings) => Promise<void>
}
```

### useKeyboardShortcut
Handles keyboard shortcuts.

**Usage:**
```typescript
useKeyboardShortcut(
  () => console.log('Pressed!'),
  { key: 's', modifiers: ['ctrl'] }
);
```

### usePushToTalk
Specialized hook for push-to-talk (hold space to record).

**Usage:**
```typescript
usePushToTalk(
  () => startRecording(),
  () => stopRecording(),
  { key: ' ', enabled: true }
);
```

## Utilities

### Audio Utils (`utils/audio.ts`)

**Functions:**
- `createAudioRecorder(options)` - Create recorder instance
- `AudioPlayer` class - Audio playback management
- `formatDuration(seconds)` - Format time display

**Features:**
- Browser MediaRecorder API integration
- WAV format conversion
- Audio resampling
- Blob URL management

### Tauri Utils (`utils/tauri.ts`)

Typed wrappers for all 13 Tauri backend commands.

**Commands:**
- `processAudio()` - Transcribe audio
- `synthesizeSpeech()` - Text-to-speech
- `sendMessage()` - Send text to LLM
- `processVoiceQuery()` - Complete voice pipeline
- `loadConfig()` / `saveConfig()` - Configuration
- `updateApiKey()` - Secure key storage
- `getAppState()` - Current state
- `getConversation()` - Message history
- `clearConversation()` - Reset history
- `checkConnectivity()` - Service status
- `listVoices()` - Available voices
- `updateVoiceSettings()` - Voice config

## Styling

### Design System

**Colors:**
- Primary: Purple gradient (#667eea → #764ba2)
- Background: Dark purple gradient
- Error: Red (#ef4444)
- Success: Green (#10b981)
- Warning: Orange (#f59e0b)

**Effects:**
- Glassmorphism with backdrop-filter
- Smooth animations (0.2s - 0.3s ease)
- Pulse effects for recording
- Ring animations for active states

**Typography:**
- System font stack
- Base size: 16px
- Line height: 1.5

**Spacing:**
- Base unit: 0.25rem (4px)
- Common gaps: 0.5rem, 0.75rem, 1rem, 1.5rem

## Accessibility

### Features
- ARIA labels on all interactive elements
- Keyboard navigation support
- Focus visible indicators
- Screen reader friendly status updates
- Semantic HTML structure

### Keyboard Shortcuts
- **Enter** - Send text message
- **Shift+Enter** - New line in text input
- **Space** (hold) - Push-to-talk (optional)
- **Escape** - Dismiss modals

## Performance Optimizations

### React Optimizations
- `React.memo()` on all components
- Zustand selectors for targeted re-renders
- Lazy loading for heavy components
- Debounced scroll handlers

### Audio Optimizations
- Efficient WAV encoding
- Blob URL cleanup
- Audio context reuse
- Timer cleanup on unmount

## Browser Compatibility

### Required APIs
- MediaRecorder API
- Web Audio API
- AudioContext
- Blob API
- URL.createObjectURL

### Supported Browsers
- Chrome/Edge 80+
- Firefox 75+
- Safari 14+

## Development

### Run Development Server
```bash
npm run dev
```

### Build for Production
```bash
npm run build
```

### Type Check
```bash
npx tsc --noEmit
```

## Usage Examples

### Send Text Message
```typescript
const { sendTextMessage } = useTauri();

const handleSend = async (message: string) => {
  const response = await sendTextMessage(message);
  console.log('Response:', response);
};
```

### Voice Recording
```typescript
const recorder = useAudioRecorder();

// Start recording
await recorder.startRecording();

// Stop and get audio data
const audioData = await recorder.stopRecording();
```

### Process Voice Query
```typescript
const { processVoiceQuery } = useTauri();

const result = await processVoiceQuery(audioData);
console.log('Transcription:', result.transcription);
console.log('Response:', result.llm_response);

// Play audio response
await player.play(new Uint8Array(result.audio_response));
```

## Error Handling

All async operations include try-catch blocks:

```typescript
try {
  await processVoiceQuery(audioData);
} catch (error) {
  setError({
    message: error instanceof Error ? error.message : 'Unknown error',
    timestamp: Date.now(),
    source: 'voice-query'
  });
}
```

Errors are displayed via ErrorMessage component with auto-dismiss.

## Future Enhancements

### Planned Features
- [ ] Settings panel for configuration
- [ ] Voice selector dropdown
- [ ] Theme switching (light/dark)
- [ ] Export conversation history
- [ ] Audio waveform visualization
- [ ] Streaming LLM responses
- [ ] Multi-language support
- [ ] Conversation search
- [ ] Message editing
- [ ] Custom keyboard shortcuts

## Troubleshooting

### Microphone Not Working
1. Check browser permissions
2. Verify MediaRecorder support
3. Check console for errors
4. Try different browser

### Audio Playback Issues
1. Check audio format (should be MP3)
2. Verify Blob URL creation
3. Check browser console
4. Test with different audio data

### Tauri Commands Failing
1. Check backend is running
2. Verify API keys are set
3. Check connectivity status
4. Review backend logs

## Contributing

When adding new components:
1. Create component file in `components/`
2. Create corresponding CSS file
3. Add to `components/index.ts`
4. Use TypeScript for type safety
5. Include ARIA labels
6. Use React.memo() for optimization
7. Write clear comments

## License

See LICENSE file in project root.
