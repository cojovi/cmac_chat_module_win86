# Talk to CMAC - Implementation Guide

This guide provides step-by-step instructions for implementing the design system.

## Quick Start

### 1. Install Dependencies

```bash
# Core dependencies (already installed)
npm install react react-dom zustand

# Additional UI dependencies
npm install clsx
npm install @tauri-apps/api@^2

# Development dependencies (optional)
npm install -D @types/node
```

### 2. Import Design System Styles

Update `src/main.tsx` to import the design system:

```typescript
import React from 'react';
import ReactDOM from 'react-dom/client';
import App from './App';

// Import design system styles
import './styles/variables.css';
import './styles/animations.css';

ReactDOM.createRoot(document.getElementById('root')!).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>
);
```

### 3. Set Up Theme Management

Create `src/hooks/useTheme.ts`:

```typescript
import { useEffect, useState } from 'react';

export type ThemeMode = 'light' | 'dark' | 'auto';

export function useTheme() {
  const [theme, setTheme] = useState<ThemeMode>('auto');

  useEffect(() => {
    const root = document.documentElement;

    if (theme === 'auto') {
      // Use system preference
      const isDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
      root.removeAttribute('data-theme');
    } else {
      root.setAttribute('data-theme', theme);
    }
  }, [theme]);

  // Listen for system theme changes
  useEffect(() => {
    if (theme !== 'auto') return;

    const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)');
    const handler = () => {
      // Force re-render to apply new theme
      setTheme('auto');
    };

    mediaQuery.addEventListener('change', handler);
    return () => mediaQuery.removeEventListener('change', handler);
  }, [theme]);

  return { theme, setTheme };
}
```

---

## Component Implementation Order

### Phase 1: Foundation Components (Week 1)

#### 1.1 StatusBar Component

Create `src/components/StatusBar/StatusBar.tsx`:

```typescript
import React from 'react';
import { AppState, APP_STATES } from '../../types/design-system';
import styles from './StatusBar.module.css';

interface StatusBarProps {
  state: AppState;
  onSettingsClick: () => void;
}

export const StatusBar: React.FC<StatusBarProps> = ({ state, onSettingsClick }) => {
  const statusInfo = APP_STATES[state];

  return (
    <div className={styles.statusBar}>
      <div className={styles.statusIndicator}>
        <span
          className={`${styles.statusIcon} ${statusInfo.animated ? styles.animated : ''}`}
          style={{ background: statusInfo.color }}
          aria-hidden="true"
        />
        <span className={styles.statusText}>{statusInfo.label}</span>
      </div>

      <button
        className={styles.settingsButton}
        onClick={onSettingsClick}
        aria-label="Open settings"
      >
        <SettingsIcon />
      </button>
    </div>
  );
};

// Simple SVG icon component
const SettingsIcon: React.FC = () => (
  <svg width="20" height="20" viewBox="0 0 20 20" fill="currentColor">
    <path d="M10 6a2 2 0 110-4 2 2 0 010 4zM10 12a2 2 0 110-4 2 2 0 010 4zM10 18a2 2 0 110-4 2 2 0 010 4z" />
  </svg>
);
```

Create `src/components/StatusBar/StatusBar.module.css`:

```css
.statusBar {
  height: 48px;
  padding: 0 var(--spacing-lg);
  display: flex;
  align-items: center;
  justify-content: space-between;
  background: linear-gradient(135deg,
    var(--cmac-primary-dark) 0%,
    var(--cmac-primary) 100%);
  color: white;
  font-size: var(--text-sm);
  font-weight: var(--font-medium);
}

.statusIndicator {
  display: flex;
  align-items: center;
  gap: var(--spacing-sm);
}

.statusIcon {
  width: 12px;
  height: 12px;
  border-radius: 50%;
  transition: all var(--transition-base);
}

.statusIcon.animated {
  animation: pulse 2s ease-in-out infinite;
}

.statusText {
  font-size: var(--text-sm);
  letter-spacing: var(--tracking-normal);
}

.settingsButton {
  width: 32px;
  height: 32px;
  border: none;
  border-radius: 50%;
  background: rgba(255, 255, 255, 0.1);
  color: white;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all var(--transition-base);
}

.settingsButton:hover {
  background: rgba(255, 255, 255, 0.2);
  transform: rotate(90deg);
}

.settingsButton:focus-visible {
  outline: 2px solid white;
  outline-offset: 2px;
}
```

#### 1.2 MicButton Component

Create `src/components/MicButton/MicButton.tsx`:

```typescript
import React from 'react';
import styles from './MicButton.module.css';

interface MicButtonProps {
  state: 'idle' | 'listening' | 'disabled';
  onClick: () => void;
  onMouseDown?: () => void;
  onMouseUp?: () => void;
  disabled?: boolean;
}

export const MicButton: React.FC<MicButtonProps> = ({
  state,
  onClick,
  onMouseDown,
  onMouseUp,
  disabled = false,
}) => {
  return (
    <button
      className={`${styles.micButton} ${styles[state]}`}
      onClick={onClick}
      onMouseDown={onMouseDown}
      onMouseUp={onMouseUp}
      disabled={disabled || state === 'disabled'}
      aria-label={state === 'listening' ? 'Stop recording' : 'Start recording'}
      aria-pressed={state === 'listening'}
    >
      <MicIcon className={styles.icon} />
    </button>
  );
};

const MicIcon: React.FC<{ className?: string }> = ({ className }) => (
  <svg
    width="32"
    height="32"
    viewBox="0 0 24 24"
    fill="none"
    stroke="currentColor"
    strokeWidth="2"
    strokeLinecap="round"
    strokeLinejoin="round"
    className={className}
  >
    <path d="M12 1a3 3 0 0 0-3 3v8a3 3 0 0 0 6 0V4a3 3 0 0 0-3-3z" />
    <path d="M19 10v2a7 7 0 0 1-14 0v-2" />
    <line x1="12" y1="19" x2="12" y2="23" />
    <line x1="8" y1="23" x2="16" y2="23" />
  </svg>
);
```

Create `src/components/MicButton/MicButton.module.css`:

```css
.micButton {
  width: 64px;
  height: 64px;
  border-radius: 50%;
  border: 3px solid var(--cmac-gray-300);
  background: white;
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  transition: all var(--transition-base);
  position: relative;
  box-shadow: var(--shadow-md);
}

.micButton:hover:not(:disabled) {
  border-color: var(--cmac-primary-light);
  transform: scale(1.05);
  box-shadow: 0 6px 20px rgba(59, 130, 246, 0.3);
}

.micButton:active:not(:disabled) {
  transform: scale(0.98);
}

.micButton.listening {
  border-color: var(--state-listening);
  background: var(--state-listening);
  animation: pulse-rings 2s ease-out infinite;
}

.micButton.listening::before {
  content: '';
  position: absolute;
  width: 100%;
  height: 100%;
  border-radius: 50%;
  border: 3px solid var(--state-listening);
  animation: pulse-ring 1.5s ease-out infinite;
}

.micButton.disabled,
.micButton:disabled {
  opacity: 0.4;
  cursor: not-allowed;
  border-color: var(--cmac-gray-400);
}

.micButton:disabled:hover {
  transform: none;
  box-shadow: var(--shadow-md);
}

.icon {
  width: 32px;
  height: 32px;
  color: var(--cmac-gray-700);
  transition: color var(--transition-base);
}

.micButton.listening .icon {
  color: white;
}

.micButton:disabled .icon {
  color: var(--cmac-gray-400);
}

/* Dark mode styles */
[data-theme="dark"] .micButton {
  background: var(--bg-primary-dark);
  border-color: var(--border-dark);
}

[data-theme="dark"] .micButton:hover:not(:disabled) {
  border-color: var(--cmac-primary-light);
}

[data-theme="dark"] .icon {
  color: var(--text-primary-dark);
}
```

#### 1.3 MessageBubble Component

Create `src/components/MessageBubble/MessageBubble.tsx`:

```typescript
import React from 'react';
import { Message } from '../../types/design-system';
import styles from './MessageBubble.module.css';

interface MessageBubbleProps {
  message: Message;
  showTimestamp?: boolean;
}

export const MessageBubble: React.FC<MessageBubbleProps> = ({
  message,
  showTimestamp = true,
}) => {
  const isUser = message.type === 'user';

  return (
    <div className={`${styles.bubble} ${styles[message.type]}`}>
      <div className={styles.content}>
        {message.content}
      </div>

      {message.audioUrl && (
        <AudioControls audioUrl={message.audioUrl} />
      )}

      {showTimestamp && (
        <div className={styles.timestamp}>
          {formatTime(message.timestamp)}
        </div>
      )}
    </div>
  );
};

// Simple audio controls
const AudioControls: React.FC<{ audioUrl: string }> = ({ audioUrl }) => {
  const [playing, setPlaying] = React.useState(false);
  const audioRef = React.useRef<HTMLAudioElement>(null);

  const togglePlay = () => {
    if (!audioRef.current) return;

    if (playing) {
      audioRef.current.pause();
    } else {
      audioRef.current.play();
    }
    setPlaying(!playing);
  };

  return (
    <div className={styles.audioControls}>
      <button
        className={styles.playButton}
        onClick={togglePlay}
        aria-label={playing ? 'Pause' : 'Play'}
      >
        {playing ? <PauseIcon /> : <PlayIcon />}
      </button>

      <div className={styles.waveform}>
        <div className={styles.waveformBar} />
        <div className={styles.waveformBar} />
        <div className={styles.waveformBar} />
        <div className={styles.waveformBar} />
        <div className={styles.waveformBar} />
      </div>

      <audio
        ref={audioRef}
        src={audioUrl}
        onEnded={() => setPlaying(false)}
      />
    </div>
  );
};

const PlayIcon = () => (
  <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor">
    <path d="M8 5v14l11-7z" />
  </svg>
);

const PauseIcon = () => (
  <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor">
    <path d="M6 4h4v16H6V4zm8 0h4v16h-4V4z" />
  </svg>
);

function formatTime(date: Date): string {
  return date.toLocaleTimeString('en-US', {
    hour: 'numeric',
    minute: '2-digit',
  });
}
```

Create `src/components/MessageBubble/MessageBubble.module.css`:

```css
.bubble {
  max-width: 75%;
  margin-bottom: var(--spacing-md);
  padding: var(--spacing-md) var(--spacing-lg);
  font-size: var(--text-base);
  line-height: var(--leading-normal);
  box-shadow: var(--shadow-sm);
}

.bubble.user {
  margin-left: auto;
  border-radius: 18px 18px 4px 18px;
  background: linear-gradient(135deg,
    var(--cmac-primary) 0%,
    var(--cmac-primary-light) 100%);
  color: white;
  animation: slideInRight 0.3s ease-out;
}

.bubble.assistant {
  margin-right: auto;
  border-radius: 18px 18px 18px 4px;
  background: var(--cmac-gray-100);
  color: var(--text-primary);
  animation: slideInLeft 0.3s ease-out;
}

.content {
  word-wrap: break-word;
}

.timestamp {
  font-size: var(--text-xs);
  color: rgba(255, 255, 255, 0.7);
  margin-top: var(--spacing-xs);
  text-align: right;
}

.bubble.assistant .timestamp {
  color: var(--text-secondary);
  text-align: left;
}

.audioControls {
  display: flex;
  align-items: center;
  gap: var(--spacing-sm);
  margin-top: var(--spacing-sm);
  padding: var(--spacing-sm);
  background: rgba(0, 0, 0, 0.05);
  border-radius: var(--radius-md);
}

.playButton {
  width: 32px;
  height: 32px;
  border-radius: 50%;
  background: var(--cmac-primary);
  border: none;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  color: white;
  transition: all var(--transition-base);
  flex-shrink: 0;
}

.playButton:hover {
  background: var(--cmac-primary-light);
  transform: scale(1.1);
}

.waveform {
  flex: 1;
  height: 24px;
  display: flex;
  align-items: center;
  justify-content: space-around;
  gap: 2px;
}

.waveformBar {
  width: 3px;
  height: 4px;
  background: var(--cmac-primary);
  border-radius: 2px;
  animation: waveform 0.8s ease-in-out infinite;
}

.waveformBar:nth-child(2) { animation-delay: 0.1s; }
.waveformBar:nth-child(3) { animation-delay: 0.2s; }
.waveformBar:nth-child(4) { animation-delay: 0.3s; }
.waveformBar:nth-child(5) { animation-delay: 0.4s; }

/* Dark mode */
[data-theme="dark"] .bubble.assistant {
  background: var(--cmac-gray-800);
  color: var(--text-primary-dark);
}

[data-theme="dark"] .audioControls {
  background: rgba(255, 255, 255, 0.05);
}
```

---

## State Management with Zustand

Create `src/store/appStore.ts`:

```typescript
import { create } from 'zustand';
import { AppState, Message, Settings, AppError, DEFAULT_SETTINGS } from '../types/design-system';

interface AppStore {
  currentState: AppState;
  messages: Message[];
  settings: Settings;
  error: AppError | null;
  inputValue: string;

  setState: (state: AppState) => void;
  addMessage: (message: Omit<Message, 'id' | 'timestamp'>) => void;
  clearMessages: () => void;
  setSettings: (settings: Partial<Settings>) => void;
  setError: (error: AppError | null) => void;
  setInputValue: (value: string) => void;
}

export const useAppStore = create<AppStore>((set) => ({
  currentState: 'initializing',
  messages: [],
  settings: DEFAULT_SETTINGS,
  error: null,
  inputValue: '',

  setState: (state) => set({ currentState: state }),

  addMessage: (message) =>
    set((state) => ({
      messages: [
        ...state.messages,
        {
          ...message,
          id: crypto.randomUUID(),
          timestamp: new Date(),
        },
      ],
    })),

  clearMessages: () => set({ messages: [] }),

  setSettings: (settings) =>
    set((state) => ({
      settings: { ...state.settings, ...settings },
    })),

  setError: (error) => set({ error }),

  setInputValue: (value) => set({ inputValue: value }),
}));
```

---

## Main App Structure

Update `src/App.tsx`:

```typescript
import React, { useEffect } from 'react';
import { StatusBar } from './components/StatusBar/StatusBar';
import { MessageBubble } from './components/MessageBubble/MessageBubble';
import { MicButton } from './components/MicButton/MicButton';
import { useAppStore } from './store/appStore';
import { useTheme } from './hooks/useTheme';
import './App.css';

function App() {
  const { theme, setTheme } = useTheme();
  const {
    currentState,
    messages,
    inputValue,
    setState,
    addMessage,
    setInputValue,
  } = useAppStore();

  const [settingsOpen, setSettingsOpen] = React.useState(false);

  useEffect(() => {
    // Initialize app
    setState('idle');
  }, [setState]);

  const handleMicClick = () => {
    if (currentState === 'listening') {
      setState('transcribing');
      // TODO: Stop recording and process
    } else if (currentState === 'idle') {
      setState('listening');
      // TODO: Start recording
    }
  };

  const handleSendMessage = () => {
    if (!inputValue.trim()) return;

    addMessage({
      type: 'user',
      content: inputValue,
    });

    setInputValue('');
    setState('thinking');

    // TODO: Send to API
  };

  return (
    <div className="app">
      <StatusBar
        state={currentState}
        onSettingsClick={() => setSettingsOpen(true)}
      />

      <div className="header">
        <div className="logo">🏠</div>
        <h1 className="title">Talk to CMAC</h1>
        <p className="subtitle">Your Roofing Assistant</p>
      </div>

      <div className="messages">
        {messages.length === 0 ? (
          <div className="empty-state">
            <p>Hi! I'm CMAC, your roofing assistant.</p>
            <p>How can I help you today?</p>
          </div>
        ) : (
          messages.map((message) => (
            <MessageBubble key={message.id} message={message} />
          ))
        )}
      </div>

      <div className="input-area">
        <input
          type="text"
          className="text-input"
          placeholder="Type your message..."
          value={inputValue}
          onChange={(e) => setInputValue(e.target.value)}
          onKeyDown={(e) => e.key === 'Enter' && handleSendMessage()}
          disabled={currentState !== 'idle'}
        />
        <MicButton
          state={currentState === 'listening' ? 'listening' : 'idle'}
          onClick={handleMicClick}
          disabled={currentState !== 'idle' && currentState !== 'listening'}
        />
      </div>
    </div>
  );
}

export default App;
```

Update `src/App.css`:

```css
.app {
  width: 420px;
  height: 650px;
  border-radius: var(--radius-xl);
  background: var(--glass-bg);
  backdrop-filter: blur(var(--glass-blur));
  border: 1px solid var(--border-color);
  box-shadow: var(--shadow-2xl), 0 0 0 1px rgba(255, 255, 255, 0.5) inset;
  overflow: hidden;
  display: flex;
  flex-direction: column;
}

.header {
  height: 80px;
  padding: var(--spacing-lg);
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  border-bottom: 1px solid var(--border-color);
  background: linear-gradient(180deg,
    rgba(30, 58, 138, 0.03) 0%,
    transparent 100%);
}

.logo {
  font-size: 36px;
  margin-bottom: var(--spacing-xs);
}

.title {
  font-size: var(--text-lg);
  font-weight: var(--font-semibold);
  color: var(--cmac-primary);
  letter-spacing: var(--tracking-tight);
  margin: 0;
}

.subtitle {
  font-size: var(--text-xs);
  color: var(--text-secondary);
  text-transform: uppercase;
  letter-spacing: var(--tracking-wide);
  margin: 0;
}

.messages {
  flex: 1;
  padding: var(--spacing-lg);
  overflow-y: auto;
  display: flex;
  flex-direction: column;
}

.empty-state {
  flex: 1;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  text-align: center;
  color: var(--text-secondary);
  gap: var(--spacing-sm);
}

.input-area {
  height: 72px;
  padding: var(--spacing-md) var(--spacing-lg);
  border-top: 1px solid var(--border-color);
  background: var(--bg-secondary);
  display: flex;
  align-items: center;
  gap: var(--spacing-md);
}

.text-input {
  flex: 1;
  height: 48px;
  padding: 0 var(--spacing-lg);
  border: 2px solid var(--border-color);
  border-radius: var(--radius-2xl);
  font-size: var(--text-base);
  font-family: var(--font-primary);
  background: var(--bg-primary);
  color: var(--text-primary);
  transition: all var(--transition-base);
}

.text-input:focus {
  outline: none;
  border-color: var(--cmac-primary-light);
  box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
}

.text-input:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}
```

---

## Testing Your Implementation

### Visual Testing Checklist

1. **Colors & Theming**
   - [ ] Light mode displays correctly
   - [ ] Dark mode displays correctly
   - [ ] System theme preference auto-switches
   - [ ] All state colors visible (listening, thinking, etc.)

2. **Components**
   - [ ] StatusBar renders with correct state
   - [ ] MicButton shows all states (idle, listening, disabled)
   - [ ] Message bubbles align correctly (user right, assistant left)
   - [ ] Audio controls appear and function

3. **Interactions**
   - [ ] Mic button responds to clicks
   - [ ] Input field accepts text
   - [ ] Enter key sends message
   - [ ] Settings button opens panel (when implemented)

4. **Animations**
   - [ ] Messages slide in smoothly
   - [ ] Mic button pulses when listening
   - [ ] Status icon animates
   - [ ] Transitions feel smooth (60fps)

### Keyboard Navigation Test

```
Tab order:
1. Status bar (screen reader only)
2. Settings button
3. Message bubbles (scrollable)
4. Text input
5. Mic button

Test all keyboard shortcuts:
- Tab: Navigate forward
- Shift+Tab: Navigate backward
- Enter: Send message (when input focused)
- Space: Activate buttons
- Escape: Close modals/settings
```

---

## Next Steps

1. **Week 1**: Implement foundation components (StatusBar, MicButton, MessageBubble)
2. **Week 2**: Add Settings panel, error handling, and complete state management
3. **Week 3**: Integrate with Tauri APIs for system tray and global shortcuts
4. **Week 4**: Polish animations, accessibility, and cross-platform testing

---

## Resources

- **Design System**: `/DESIGN_SYSTEM.md`
- **Type Definitions**: `/src/types/design-system.ts`
- **CSS Variables**: `/src/styles/variables.css`
- **Animations**: `/src/styles/animations.css`

## Support

Refer to the main `DESIGN_SYSTEM.md` document for detailed specifications on:
- Color palette and usage
- Typography system
- Complete animation specifications
- Accessibility requirements
- Component dimensions and spacing

---

**Version**: 1.0
**Last Updated**: 2025-12-01
