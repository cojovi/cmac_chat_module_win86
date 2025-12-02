# Talk to CMAC - UI/UX Design System

## Executive Summary
A Siri-inspired voice assistant for Windows with CMAC Roofing branding. Features glassmorphism, smooth animations, and a professional yet approachable aesthetic for construction industry users.

---

## 1. Color Palette

### Primary Brand Colors (CMAC Roofing Theme)

```css
/* Primary - Deep Professional Blue */
--cmac-primary: #1E3A8A;        /* Deep blue - trust, reliability */
--cmac-primary-light: #3B82F6;  /* Bright blue - active states */
--cmac-primary-dark: #1E293B;   /* Near-black blue - backgrounds */

/* Secondary - Roofing Accent */
--cmac-secondary: #DC2626;      /* Bold red - roofing accent */
--cmac-secondary-light: #F87171; /* Light red - hover states */
--cmac-secondary-dark: #991B1B;  /* Dark red - pressed states */

/* Neutral Palette */
--cmac-gray-50: #F8FAFC;
--cmac-gray-100: #F1F5F9;
--cmac-gray-200: #E2E8F0;
--cmac-gray-300: #CBD5E1;
--cmac-gray-400: #94A3B8;
--cmac-gray-500: #64748B;
--cmac-gray-600: #475569;
--cmac-gray-700: #334155;
--cmac-gray-800: #1E293B;
--cmac-gray-900: #0F172A;

/* Glassmorphism Base */
--glass-white: rgba(255, 255, 255, 0.85);
--glass-dark: rgba(30, 41, 59, 0.85);
--glass-blur: 20px;
```

### State-Specific Colors

```css
/* Listening State - Pulsing Blue */
--state-listening: #3B82F6;
--state-listening-glow: rgba(59, 130, 246, 0.4);

/* Transcribing State - Processing Yellow */
--state-transcribing: #F59E0B;
--state-transcribing-glow: rgba(245, 158, 11, 0.4);

/* Thinking State - Purple AI */
--state-thinking: #8B5CF6;
--state-thinking-glow: rgba(139, 92, 246, 0.4);

/* Speaking State - Active Green */
--state-speaking: #10B981;
--state-speaking-glow: rgba(16, 185, 129, 0.4);

/* Error State - Warning Red */
--state-error: #DC2626;
--state-error-glow: rgba(220, 38, 38, 0.4);

/* Offline State - Muted Gray */
--state-offline: #64748B;
--state-offline-glow: rgba(100, 116, 139, 0.4);
```

### Light/Dark Mode Support

```css
/* Light Mode */
--bg-primary-light: #FFFFFF;
--bg-secondary-light: #F8FAFC;
--text-primary-light: #0F172A;
--text-secondary-light: #64748B;
--border-light: #E2E8F0;
--shadow-light: rgba(0, 0, 0, 0.1);

/* Dark Mode */
--bg-primary-dark: #0F172A;
--bg-secondary-dark: #1E293B;
--text-primary-dark: #F8FAFC;
--text-secondary-dark: #94A3B8;
--border-dark: #334155;
--shadow-dark: rgba(0, 0, 0, 0.5);
```

---

## 2. Typography System

### Font Families

```css
/* Primary Font - Modern Sans-Serif */
--font-primary: 'Inter', 'Segoe UI', system-ui, -apple-system, sans-serif;

/* Secondary Font - Display/Headers */
--font-display: 'Inter', 'Segoe UI', system-ui, sans-serif;

/* Monospace - Technical Info */
--font-mono: 'JetBrains Mono', 'Consolas', 'Courier New', monospace;
```

### Type Scale

```css
/* Font Sizes */
--text-xs: 0.75rem;      /* 12px - timestamps, metadata */
--text-sm: 0.875rem;     /* 14px - secondary text */
--text-base: 1rem;       /* 16px - body text */
--text-lg: 1.125rem;     /* 18px - emphasized text */
--text-xl: 1.25rem;      /* 20px - small headers */
--text-2xl: 1.5rem;      /* 24px - section headers */
--text-3xl: 1.875rem;    /* 30px - page title */

/* Font Weights */
--font-regular: 400;
--font-medium: 500;
--font-semibold: 600;
--font-bold: 700;

/* Line Heights */
--leading-tight: 1.25;
--leading-normal: 1.5;
--leading-relaxed: 1.75;

/* Letter Spacing */
--tracking-tight: -0.01em;
--tracking-normal: 0;
--tracking-wide: 0.025em;
```

---

## 3. Component Specifications

### A. Main Chat Window

#### Dimensions
```
Width: 420px (compact but comfortable)
Height: 650px (fits standard screens)
Min Height: 400px
Max Height: 800px
Border Radius: 16px
```

#### Structure
```
┌─────────────────────────────────────┐
│  [Status Bar]                   [⚙]│  ← 48px height
├─────────────────────────────────────┤
│                                     │
│  [CMAC Logo]                        │  ← 60px header
│  Talk to CMAC                       │
│                                     │
├─────────────────────────────────────┤
│                                     │
│  [Message Bubbles]                  │
│                                     │
│  ┌─────────────────────┐            │  ← User (right)
│  │ User message here   │            │
│  └─────────────────────┘            │
│                                     │
│  ┌─────────────────────┐            │  ← Assistant (left)
│  │ Assistant response  │            │
│  │ [🔊 Playing... ━━●─]│            │
│  └─────────────────────┘            │
│                                     │
│                                     │  ← Scrollable area
│                                     │
├─────────────────────────────────────┤
│  ┌───────────────────────────┐      │
│  │ Type message...           │ [⊙]  │  ← 72px input area
│  └───────────────────────────┘      │
└─────────────────────────────────────┘
```

#### Styling
```css
.chat-window {
  width: 420px;
  height: 650px;
  border-radius: 16px;
  background: var(--glass-white); /* Light mode */
  backdrop-filter: blur(var(--glass-blur));
  border: 1px solid var(--border-light);
  box-shadow:
    0 20px 60px rgba(0, 0, 0, 0.3),
    0 0 0 1px rgba(255, 255, 255, 0.5) inset;
  overflow: hidden;
  display: flex;
  flex-direction: column;
}

/* Dark mode variant */
.chat-window.dark {
  background: var(--glass-dark);
  border: 1px solid var(--border-dark);
  box-shadow:
    0 20px 60px rgba(0, 0, 0, 0.6),
    0 0 0 1px rgba(255, 255, 255, 0.1) inset;
}
```

---

### B. Status Bar Component

#### Visual States
```
Idle:         [●] Ready
Listening:    [◐] Listening...     (pulsing animation)
Transcribing: [◓] Transcribing...  (spinning animation)
Thinking:     [◑] Thinking...      (ellipsis animation)
Speaking:     [◒] Speaking...      (waveform animation)
Error:        [✕] Error            (shake animation)
Offline:      [○] Offline          (static gray)
```

#### Specifications
```css
.status-bar {
  height: 48px;
  padding: 0 16px;
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

.status-indicator {
  display: flex;
  align-items: center;
  gap: 8px;
}

.status-icon {
  width: 12px;
  height: 12px;
  border-radius: 50%;
  transition: all 0.3s ease;
}

/* State-specific styling */
.status-icon.listening {
  background: var(--state-listening);
  box-shadow: 0 0 12px var(--state-listening-glow);
  animation: pulse 2s ease-in-out infinite;
}

.status-icon.thinking {
  background: var(--state-thinking);
  box-shadow: 0 0 12px var(--state-thinking-glow);
  animation: spin 1.5s linear infinite;
}

.status-icon.speaking {
  background: var(--state-speaking);
  box-shadow: 0 0 12px var(--state-speaking-glow);
  animation: pulse 1s ease-in-out infinite;
}

.status-icon.error {
  background: var(--state-error);
  animation: shake 0.5s ease;
}
```

---

### C. Microphone Button

#### Dimensions & States
```
Size: 64px diameter (large, prominent)
Icon Size: 32px
Border: 3px solid
```

#### Visual States
```
┌─────────────────────────────────────────────────┐
│  IDLE        HOVER        ACTIVE      DISABLED  │
│   ┌─┐         ┌─┐         ┌─┐         ┌─┐      │
│   │🎤│        │🎤│        │🎤│        │🎤│     │
│   └─┘         └─┘         └─┘         └─┘      │
│  Gray        Blue       Pulsing      Muted     │
└─────────────────────────────────────────────────┘
```

#### Specifications
```css
.mic-button {
  width: 64px;
  height: 64px;
  border-radius: 50%;
  border: 3px solid var(--cmac-gray-300);
  background: white;
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  transition: all 0.2s ease;
  position: relative;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
}

.mic-button:hover {
  border-color: var(--cmac-primary-light);
  transform: scale(1.05);
  box-shadow: 0 6px 20px rgba(59, 130, 246, 0.3);
}

.mic-button.active {
  border-color: var(--state-listening);
  background: var(--state-listening);
  animation: pulse-ring 1.5s ease-out infinite;
}

.mic-button.active::before {
  content: '';
  position: absolute;
  width: 100%;
  height: 100%;
  border-radius: 50%;
  border: 3px solid var(--state-listening);
  animation: pulse-ring 1.5s ease-out infinite;
}

.mic-button:disabled {
  opacity: 0.4;
  cursor: not-allowed;
  border-color: var(--cmac-gray-400);
}

.mic-button:disabled:hover {
  transform: none;
}

/* Icon inside button */
.mic-icon {
  width: 32px;
  height: 32px;
  color: var(--cmac-gray-700);
  transition: color 0.2s ease;
}

.mic-button.active .mic-icon {
  color: white;
}
```

---

### D. Message Bubbles

#### User Message (Right-aligned)
```css
.message-bubble.user {
  max-width: 75%;
  margin-left: auto;
  margin-bottom: 12px;
  padding: 12px 16px;
  border-radius: 18px 18px 4px 18px;
  background: linear-gradient(135deg,
    var(--cmac-primary) 0%,
    var(--cmac-primary-light) 100%);
  color: white;
  font-size: var(--text-base);
  line-height: var(--leading-normal);
  box-shadow: 0 2px 8px rgba(59, 130, 246, 0.2);
  animation: slideInRight 0.3s ease-out;
}
```

#### Assistant Message (Left-aligned)
```css
.message-bubble.assistant {
  max-width: 80%;
  margin-right: auto;
  margin-bottom: 12px;
  padding: 12px 16px;
  border-radius: 18px 18px 18px 4px;
  background: var(--cmac-gray-100);
  color: var(--text-primary-light);
  font-size: var(--text-base);
  line-height: var(--leading-normal);
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.05);
  animation: slideInLeft 0.3s ease-out;
}

/* Dark mode assistant message */
.dark .message-bubble.assistant {
  background: var(--cmac-gray-800);
  color: var(--text-primary-dark);
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.3);
}
```

#### Audio Playback Controls
```css
.audio-controls {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-top: 8px;
  padding: 8px;
  background: rgba(0, 0, 0, 0.05);
  border-radius: 8px;
}

.play-button {
  width: 32px;
  height: 32px;
  border-radius: 50%;
  background: var(--cmac-primary);
  border: none;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.2s ease;
}

.play-button:hover {
  background: var(--cmac-primary-light);
  transform: scale(1.1);
}

.audio-waveform {
  flex: 1;
  height: 24px;
  background: repeating-linear-gradient(
    to right,
    var(--cmac-primary) 0px,
    var(--cmac-primary) 2px,
    transparent 2px,
    transparent 4px
  );
  border-radius: 4px;
  position: relative;
  overflow: hidden;
}

.audio-progress {
  position: absolute;
  left: 0;
  top: 0;
  height: 100%;
  background: var(--state-speaking);
  transition: width 0.1s linear;
}
```

#### Timestamp
```css
.message-timestamp {
  font-size: var(--text-xs);
  color: var(--text-secondary-light);
  margin-top: 4px;
  text-align: right;
}

.dark .message-timestamp {
  color: var(--text-secondary-dark);
}
```

---

### E. Input Area

#### Specifications
```css
.input-area {
  height: 72px;
  padding: 12px 16px;
  border-top: 1px solid var(--border-light);
  background: var(--bg-secondary-light);
  display: flex;
  align-items: center;
  gap: 12px;
}

.dark .input-area {
  border-top: 1px solid var(--border-dark);
  background: var(--bg-secondary-dark);
}

.text-input {
  flex: 1;
  height: 48px;
  padding: 0 16px;
  border: 2px solid var(--border-light);
  border-radius: 24px;
  font-size: var(--text-base);
  font-family: var(--font-primary);
  background: white;
  color: var(--text-primary-light);
  transition: all 0.2s ease;
}

.text-input:focus {
  outline: none;
  border-color: var(--cmac-primary-light);
  box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
}

.dark .text-input {
  background: var(--bg-primary-dark);
  color: var(--text-primary-dark);
  border-color: var(--border-dark);
}

.dark .text-input:focus {
  border-color: var(--cmac-primary-light);
  box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.2);
}
```

---

### F. Header Section

#### Specifications
```css
.header {
  height: 80px;
  padding: 16px;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  border-bottom: 1px solid var(--border-light);
  background: linear-gradient(180deg,
    rgba(30, 58, 138, 0.03) 0%,
    transparent 100%);
}

.dark .header {
  border-bottom: 1px solid var(--border-dark);
  background: linear-gradient(180deg,
    rgba(59, 130, 246, 0.05) 0%,
    transparent 100%);
}

.logo {
  width: 36px;
  height: 36px;
  margin-bottom: 4px;
}

.app-title {
  font-size: var(--text-lg);
  font-weight: var(--font-semibold);
  color: var(--cmac-primary);
  letter-spacing: var(--tracking-tight);
}

.dark .app-title {
  color: var(--cmac-primary-light);
}

.app-subtitle {
  font-size: var(--text-xs);
  color: var(--text-secondary-light);
  text-transform: uppercase;
  letter-spacing: var(--tracking-wide);
}

.dark .app-subtitle {
  color: var(--text-secondary-dark);
}
```

---

### G. Settings Panel

#### Overlay Design
```css
.settings-overlay {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background: rgba(0, 0, 0, 0.4);
  backdrop-filter: blur(8px);
  z-index: 1000;
  display: flex;
  align-items: center;
  justify-content: center;
  animation: fadeIn 0.2s ease;
}

.settings-panel {
  width: 90%;
  max-width: 360px;
  padding: 24px;
  background: white;
  border-radius: 16px;
  box-shadow: 0 20px 60px rgba(0, 0, 0, 0.3);
  animation: slideUp 0.3s ease-out;
}

.dark .settings-panel {
  background: var(--bg-primary-dark);
  border: 1px solid var(--border-dark);
}

.settings-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 20px;
}

.settings-title {
  font-size: var(--text-xl);
  font-weight: var(--font-semibold);
  color: var(--text-primary-light);
}

.dark .settings-title {
  color: var(--text-primary-dark);
}

.close-button {
  width: 32px;
  height: 32px;
  border-radius: 50%;
  border: none;
  background: var(--cmac-gray-200);
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.2s ease;
}

.close-button:hover {
  background: var(--cmac-gray-300);
  transform: scale(1.1);
}

.dark .close-button {
  background: var(--cmac-gray-700);
}

.dark .close-button:hover {
  background: var(--cmac-gray-600);
}
```

#### Form Elements
```css
.settings-field {
  margin-bottom: 20px;
}

.settings-label {
  display: block;
  font-size: var(--text-sm);
  font-weight: var(--font-medium);
  color: var(--text-primary-light);
  margin-bottom: 8px;
}

.dark .settings-label {
  color: var(--text-primary-dark);
}

.settings-input {
  width: 100%;
  height: 40px;
  padding: 0 12px;
  border: 2px solid var(--border-light);
  border-radius: 8px;
  font-size: var(--text-base);
  font-family: var(--font-primary);
  background: white;
  color: var(--text-primary-light);
  transition: all 0.2s ease;
}

.settings-input:focus {
  outline: none;
  border-color: var(--cmac-primary-light);
  box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
}

.dark .settings-input {
  background: var(--bg-secondary-dark);
  color: var(--text-primary-dark);
  border-color: var(--border-dark);
}

.settings-select {
  width: 100%;
  height: 40px;
  padding: 0 12px;
  border: 2px solid var(--border-light);
  border-radius: 8px;
  font-size: var(--text-base);
  font-family: var(--font-primary);
  background: white;
  color: var(--text-primary-light);
  cursor: pointer;
}

.dark .settings-select {
  background: var(--bg-secondary-dark);
  color: var(--text-primary-dark);
  border-color: var(--border-dark);
}

.connection-status {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 12px;
  border-radius: 8px;
  background: var(--cmac-gray-100);
}

.connection-status.connected {
  background: rgba(16, 185, 129, 0.1);
}

.connection-status.error {
  background: rgba(220, 38, 38, 0.1);
}

.dark .connection-status {
  background: var(--cmac-gray-800);
}
```

---

## 4. Animation Specifications

### Pulse Animation (Listening/Speaking)
```css
@keyframes pulse {
  0%, 100% {
    transform: scale(1);
    opacity: 1;
  }
  50% {
    transform: scale(1.05);
    opacity: 0.8;
  }
}

@keyframes pulse-ring {
  0% {
    transform: scale(1);
    opacity: 1;
  }
  100% {
    transform: scale(1.5);
    opacity: 0;
  }
}
```

### Spin Animation (Thinking/Processing)
```css
@keyframes spin {
  from {
    transform: rotate(0deg);
  }
  to {
    transform: rotate(360deg);
  }
}
```

### Shake Animation (Error)
```css
@keyframes shake {
  0%, 100% {
    transform: translateX(0);
  }
  25% {
    transform: translateX(-8px);
  }
  75% {
    transform: translateX(8px);
  }
}
```

### Message Slide In
```css
@keyframes slideInRight {
  from {
    transform: translateX(20px);
    opacity: 0;
  }
  to {
    transform: translateX(0);
    opacity: 1;
  }
}

@keyframes slideInLeft {
  from {
    transform: translateX(-20px);
    opacity: 0;
  }
  to {
    transform: translateX(0);
    opacity: 1;
  }
}
```

### Fade & Slide (Settings Panel)
```css
@keyframes fadeIn {
  from {
    opacity: 0;
  }
  to {
    opacity: 1;
  }
}

@keyframes slideUp {
  from {
    transform: translateY(20px);
    opacity: 0;
  }
  to {
    transform: translateY(0);
    opacity: 1;
  }
}
```

### Ellipsis Animation (Thinking)
```css
.ellipsis {
  display: inline-block;
}

.ellipsis::after {
  content: '...';
  animation: ellipsis 1.5s infinite;
}

@keyframes ellipsis {
  0% {
    content: '.';
  }
  33% {
    content: '..';
  }
  66% {
    content: '...';
  }
}
```

### Audio Waveform Animation
```css
@keyframes waveform {
  0%, 100% {
    height: 4px;
  }
  50% {
    height: 20px;
  }
}

.waveform-bar {
  width: 3px;
  background: var(--state-speaking);
  border-radius: 2px;
  animation: waveform 0.8s ease-in-out infinite;
}

.waveform-bar:nth-child(2) {
  animation-delay: 0.1s;
}

.waveform-bar:nth-child(3) {
  animation-delay: 0.2s;
}

.waveform-bar:nth-child(4) {
  animation-delay: 0.3s;
}

.waveform-bar:nth-child(5) {
  animation-delay: 0.4s;
}
```

---

## 5. State Diagram

```
┌──────────────────────────────────────────────────────────┐
│                      APPLICATION START                    │
└─────────────────────┬────────────────────────────────────┘
                      │
                      ▼
              ┌───────────────┐
              │  INITIALIZING │
              │  (Check API)  │
              └───────┬───────┘
                      │
            ┌─────────┴─────────┐
            │                   │
            ▼                   ▼
     ┌──────────┐        ┌──────────┐
     │  ONLINE  │        │ OFFLINE  │
     │  (IDLE)  │        │ (ERROR)  │
     └────┬─────┘        └──────────┘
          │                     │
          │                     │ (Retry)
          │                     └──────────┐
          │                                │
          │ (User clicks mic/types)        │
          ▼                                ▼
   ┌─────────────┐                  ┌──────────┐
   │  LISTENING  │◄─────────────────┤   IDLE   │
   │  (Record)   │                  │ (Ready)  │
   └──────┬──────┘                  └────▲─────┘
          │                              │
          │ (Release button)             │
          ▼                              │
   ┌──────────────┐                     │
   │ TRANSCRIBING │                     │
   │  (STT API)   │                     │
   └──────┬───────┘                     │
          │                              │
          │ (Text ready)                 │
          ▼                              │
   ┌──────────────┐                     │
   │   THINKING   │                     │
   │  (AI API)    │                     │
   └──────┬───────┘                     │
          │                              │
          │ (Response ready)             │
          ▼                              │
   ┌──────────────┐                     │
   │   SPEAKING   │                     │
   │  (TTS Play)  │─────────────────────┘
   └──────────────┘

Error conditions from any state can return to IDLE with error message
```

---

## 6. Interaction Flows

### Flow 1: Voice Query (Push-to-Talk)

```
Step 1: User opens app from system tray
  → Window animates in (slide up + fade in, 300ms)
  → Status: "Ready"
  → Mic button: Idle state (gray)

Step 2: User clicks mic button
  → Button: Immediate visual feedback (scale 1.05, color change)
  → Status: "Listening..." with pulsing blue dot
  → Mic button: Blue background with pulse rings
  → Audio recording starts
  → Visual: Waveform animation based on mic input level

Step 3: User releases mic button
  → Recording stops
  → Button: Returns to idle state
  → Status: "Transcribing..." with spinning yellow dot
  → User message bubble appears (animated slide-in)
  → Visual: Loading shimmer in message bubble

Step 4: Transcription complete
  → Message bubble updates with text
  → Status: "Thinking..." with purple dot
  → Visual: Ellipsis animation in status bar

Step 5: AI response received
  → Assistant message bubble appears (slide-in left)
  → Status: "Speaking..." with green dot
  → Audio playback controls appear in message
  → Visual: Waveform animation synced with audio
  → Mic button: Disabled during playback

Step 6: Playback complete
  → Status: Returns to "Ready"
  → Mic button: Returns to idle state (enabled)
  → User can interact again

TIMING:
- Button press feedback: < 50ms
- Animation durations: 200-300ms
- State transitions: Smooth crossfade 150ms
```

### Flow 2: Text Query

```
Step 1: User types in text input field
  → Input field focus: Border color change + shadow
  → Keyboard typing: Live character count (optional)
  → Mic button: Remains available

Step 2: User presses Enter
  → Input field clears with fade animation
  → User message appears (slide-in right)
  → Status: "Thinking..." with purple dot
  → Input field: Temporarily disabled

Step 3: AI processes
  → Loading indicator in input area
  → Status: Continues "Thinking..."

Step 4: Response received
  → Assistant message appears (slide-in left)
  → Status: "Speaking..." if audio enabled
  → Input field: Re-enabled
  → Focus returns to input field

TIMING:
- Input feedback: Immediate
- Message animations: 300ms
- Auto-scroll to bottom: 200ms smooth scroll
```

### Flow 3: Settings Access

```
Step 1: User clicks settings icon (gear)
  → Icon: Rotate 90deg animation
  → Overlay: Fade in (200ms)
  → Panel: Slide up from center (300ms)

Step 2: User modifies settings
  → Field focus: Border highlight
  → Changes: Immediate local state update
  → Save button: Becomes enabled (color change)

Step 3: User saves
  → Button: Loading spinner
  → Validation: Check API connectivity
  → Success: Green checkmark animation
  → Error: Red shake animation + message

Step 4: Close panel
  → Panel: Slide down + fade (300ms)
  → Overlay: Fade out (200ms)
  → Return to main view with new settings applied
```

### Flow 4: Error Handling

```
ERROR TYPE 1: API Connection Error
  → Status: "Offline" with gray dot
  → Mic button: Disabled state
  → Error banner appears at top: "Cannot connect to server"
  → Retry button: Pulsing animation every 5s
  → Auto-retry: Every 30s in background

ERROR TYPE 2: Transcription Failed
  → Status: Briefly shows "Error" with shake
  → User message: Shows "[Audio could not be transcribed]"
  → Option to re-record appears
  → Status: Returns to "Ready"

ERROR TYPE 3: AI Response Error
  → Status: "Error" with red dot
  → Assistant message: "I'm having trouble responding. Please try again."
  → Mic button: Remains enabled
  → Status: Returns to "Ready" after 2s

TIMING:
- Error display: 300ms fade in
- Auto-dismiss: 5s (if non-critical)
- Shake animation: 500ms
```

---

## 7. Accessibility Guidelines

### Keyboard Navigation

```
TAB ORDER:
1. Status bar (focusable for screen readers)
2. Settings button
3. Message bubbles (focusable, scrollable with arrow keys)
4. Text input field
5. Mic button

KEYBOARD SHORTCUTS:
- Ctrl+M: Toggle microphone (start/stop recording)
- Ctrl+S: Open settings
- Ctrl+L: Clear conversation
- Escape: Close settings/dialogs
- Space: Hold for push-to-talk (when mic button focused)
- Ctrl+/: Show keyboard shortcuts help
```

### Focus Indicators

```css
/* Visible focus ring */
*:focus-visible {
  outline: 3px solid var(--cmac-primary-light);
  outline-offset: 2px;
  border-radius: 4px;
}

/* High contrast mode support */
@media (prefers-contrast: high) {
  *:focus-visible {
    outline: 4px solid currentColor;
    outline-offset: 3px;
  }
}
```

### Screen Reader Support

```html
<!-- Status bar with ARIA live region -->
<div
  className="status-bar"
  role="status"
  aria-live="polite"
  aria-atomic="true"
>
  <span className="status-text">Listening...</span>
</div>

<!-- Mic button with proper labels -->
<button
  className="mic-button"
  aria-label="Start recording"
  aria-pressed="false"
  aria-disabled="false"
>
  <span className="sr-only">Record voice message</span>
</button>

<!-- Message with role -->
<div
  className="message-bubble assistant"
  role="article"
  aria-label="Assistant response"
>
  <p>Response text here</p>
</div>

<!-- Settings with proper semantics -->
<dialog
  className="settings-panel"
  aria-labelledby="settings-title"
  aria-modal="true"
>
  <h2 id="settings-title">Settings</h2>
</dialog>
```

### Motion & Animation Preferences

```css
/* Respect prefers-reduced-motion */
@media (prefers-reduced-motion: reduce) {
  * {
    animation-duration: 0.01ms !important;
    animation-iteration-count: 1 !important;
    transition-duration: 0.01ms !important;
  }

  .pulse,
  .pulse-ring,
  .spin {
    animation: none !important;
  }
}
```

### Color Contrast

```css
/* WCAG AAA compliance - 7:1 contrast ratio minimum */

/* Light mode text */
--text-primary-light: #0F172A;    /* On white: 14.4:1 */
--text-secondary-light: #475569;  /* On white: 7.1:1 */

/* Dark mode text */
--text-primary-dark: #F8FAFC;     /* On #0F172A: 14.4:1 */
--text-secondary-dark: #94A3B8;   /* On #0F172A: 7.1:1 */

/* Button states must maintain 4.5:1 minimum */
.button:disabled {
  opacity: 0.5; /* Ensures contrast requirements still met */
}
```

---

## 8. Branding Integration

### Logo Placement

```
PRIMARY LOCATION: Header center
  - Size: 36x36px
  - Format: SVG (scalable)
  - Fallback: PNG with transparency
  - Position: Above "Talk to CMAC" title
  - Animation on load: Subtle fade-in + scale

SECONDARY LOCATION: System tray icon
  - Size: 16x16px, 32x32px (multi-resolution)
  - Simplified monochrome version
  - Windows notification icon requirements

TERTIARY LOCATION: About panel
  - Size: 64x64px
  - Full color version
  - Includes company name below
```

### Tagline & Messaging

```
PRIMARY: "Talk to CMAC"
  - Font: Inter Semibold
  - Size: 18px (var(--text-lg))
  - Color: Brand primary
  - Location: Header below logo

SECONDARY: "Your Roofing Assistant"
  - Font: Inter Regular
  - Size: 12px (var(--text-xs))
  - Color: Text secondary
  - Uppercase with letter-spacing
  - Location: Header below primary tagline

STATUS MESSAGES:
  - "CMAC is listening..."
  - "CMAC is thinking..."
  - "CMAC is responding..."
```

### Brand Voice in UI Copy

```
✓ Professional yet approachable
✓ Construction industry terminology when relevant
✓ Confidence without arrogance
✓ Helpful and solution-oriented

EXAMPLES:
Error message: "We're having trouble connecting. Let's try that again."
Empty state: "Hi! I'm CMAC, your roofing assistant. How can I help?"
Settings: "Configure your preferences to get the most out of CMAC."
```

---

## 9. Responsive Considerations

### Window Resizing

```css
/* Minimum dimensions */
.chat-window {
  min-width: 360px;
  min-height: 400px;
  max-width: 500px;
  max-height: 900px;
}

/* Adapt layout at narrow widths */
@media (max-width: 380px) {
  .message-bubble {
    max-width: 85%;
    font-size: 14px;
  }

  .mic-button {
    width: 56px;
    height: 56px;
  }

  .header {
    height: 60px;
    padding: 12px;
  }
}

/* Adapt layout at short heights */
@media (max-height: 500px) {
  .header {
    height: 50px;
  }

  .input-area {
    height: 60px;
  }
}
```

### System Tray Popup Positioning

```
POSITION CALCULATION:
- Default: Bottom-right corner
- Offset from screen edge: 16px
- Above taskbar
- Slide-up animation from taskbar

MULTI-MONITOR:
- Opens on monitor with active cursor
- Respects screen bounds
- Never clips off-screen

TASKBAR POSITIONS:
- Bottom (default): Slide up from bottom-right
- Top: Slide down from top-right
- Left: Slide right from bottom-left
- Right: Slide left from bottom-right
```

---

## 10. CSS Architecture Recommendation

### Approach: CSS Modules + CSS Variables

**Why:**
- Component-scoped styles (avoid conflicts)
- Global design tokens via CSS variables
- Easy theme switching (light/dark)
- TypeScript integration
- No runtime overhead vs CSS-in-JS

### File Structure

```
src/
├── styles/
│   ├── variables.css       (Global CSS variables)
│   ├── animations.css      (Keyframe animations)
│   ├── mixins.css          (Reusable styles)
│   └── reset.css           (Browser normalization)
├── components/
│   ├── ChatWindow/
│   │   ├── ChatWindow.tsx
│   │   └── ChatWindow.module.css
│   ├── MessageBubble/
│   │   ├── MessageBubble.tsx
│   │   └── MessageBubble.module.css
│   ├── MicButton/
│   │   ├── MicButton.tsx
│   │   └── MicButton.module.css
│   ├── StatusBar/
│   │   ├── StatusBar.tsx
│   │   └── StatusBar.module.css
│   └── Settings/
│       ├── Settings.tsx
│       └── Settings.module.css
└── App.tsx
```

### Implementation Example

```css
/* variables.css - Imported in App.tsx */
:root {
  /* All CSS variables defined here */
  --cmac-primary: #1E3A8A;
  /* ... etc */
}

[data-theme="dark"] {
  /* Dark mode overrides */
  --bg-primary: var(--bg-primary-dark);
  /* ... etc */
}
```

```typescript
// MicButton.tsx
import styles from './MicButton.module.css';

interface MicButtonProps {
  state: 'idle' | 'listening' | 'disabled';
  onClick: () => void;
}

export const MicButton: React.FC<MicButtonProps> = ({ state, onClick }) => {
  return (
    <button
      className={`${styles.micButton} ${styles[state]}`}
      onClick={onClick}
      aria-label="Record voice message"
      disabled={state === 'disabled'}
    >
      <MicIcon className={styles.icon} />
    </button>
  );
};
```

```css
/* MicButton.module.css */
.micButton {
  width: 64px;
  height: 64px;
  border-radius: 50%;
  background: white;
  border: 3px solid var(--cmac-gray-300);
  transition: all 0.2s ease;
  /* ... etc */
}

.micButton.listening {
  background: var(--state-listening);
  border-color: var(--state-listening);
  animation: pulse 2s infinite;
}

.icon {
  width: 32px;
  height: 32px;
  color: var(--cmac-gray-700);
}
```

### Alternative: Tailwind CSS

**If you prefer utility-first:**

```bash
npm install -D tailwindcss postcss autoprefixer
npx tailwindcss init -p
```

```javascript
// tailwind.config.js
module.exports = {
  content: ['./src/**/*.{js,jsx,ts,tsx}'],
  theme: {
    extend: {
      colors: {
        'cmac-primary': '#1E3A8A',
        'cmac-primary-light': '#3B82F6',
        // ... etc
      },
      animation: {
        'pulse-ring': 'pulse-ring 1.5s ease-out infinite',
      },
      keyframes: {
        'pulse-ring': {
          '0%': { transform: 'scale(1)', opacity: '1' },
          '100%': { transform: 'scale(1.5)', opacity: '0' },
        },
      },
    },
  },
  plugins: [],
};
```

```typescript
// MicButton with Tailwind
export const MicButton: React.FC<MicButtonProps> = ({ state, onClick }) => {
  const stateClasses = {
    idle: 'border-gray-300 hover:border-blue-500',
    listening: 'bg-blue-500 border-blue-500 animate-pulse-ring',
    disabled: 'opacity-40 cursor-not-allowed',
  };

  return (
    <button
      className={`
        w-16 h-16 rounded-full flex items-center justify-center
        border-3 transition-all duration-200 shadow-lg
        ${stateClasses[state]}
      `}
      onClick={onClick}
    >
      <MicIcon className="w-8 h-8" />
    </button>
  );
};
```

**Recommendation:** CSS Modules for this project
- Better for complex animations
- Clearer separation of concerns
- Easier to maintain custom brand styles
- No build-time overhead of Tailwind

---

## 11. Implementation Priority

### Phase 1: Core UI (Week 1)
```
✓ Set up CSS variables system
✓ Implement ChatWindow component
✓ Create MessageBubble component (user & assistant)
✓ Build MicButton with basic states
✓ Add StatusBar component
✓ Implement light/dark mode toggle
```

### Phase 2: Interactions (Week 2)
```
✓ Add all animations (pulse, spin, slide)
✓ Implement state management (Zustand)
✓ Wire up keyboard navigation
✓ Add focus indicators
✓ Implement text input with send
✓ Audio playback controls in messages
```

### Phase 3: Advanced Features (Week 3)
```
✓ Settings panel with form validation
✓ Error states and handling
✓ System tray integration
✓ Window positioning logic
✓ Accessibility audit & fixes
✓ Performance optimization
```

### Phase 4: Polish (Week 4)
```
✓ Fine-tune animations
✓ Add loading states
✓ Empty state designs
✓ Onboarding flow (first launch)
✓ Keyboard shortcuts help
✓ Testing across Windows versions
```

---

## 12. Design Assets Needed

### Icons (SVG format)
```
Required icons:
- microphone.svg (default state)
- microphone-active.svg (recording)
- microphone-disabled.svg (muted)
- settings-gear.svg
- close-x.svg
- play-circle.svg
- pause-circle.svg
- stop-square.svg
- check-circle.svg (success)
- alert-circle.svg (warning)
- error-x-circle.svg (error)
- info-i-circle.svg (info)
```

### Logo Variations
```
Required:
- logo-full-color.svg (36x36)
- logo-monochrome-dark.svg (system tray)
- logo-monochrome-light.svg (system tray, light mode)
- logo-large.svg (64x64 for about)

Formats:
- SVG (primary)
- PNG @1x, @2x (fallback)
- ICO multi-resolution (Windows)
```

### Brand Guidelines Document
```
Include:
- Official CMAC color palette
- Logo usage rules
- Typography specifications
- Tone of voice guidelines
```

---

## 13. Mockup (ASCII Representation)

### Main Window - Idle State
```
╔═════════════════════════════════════════════════╗
║ ● Offline          Talk to CMAC            ⚙  ║  Status Bar
╠═════════════════════════════════════════════════╣
║                                                 ║
║                  [CMAC Logo]                    ║  Header
║                Talk to CMAC                     ║
║            YOUR ROOFING ASSISTANT               ║
║                                                 ║
╠═════════════════════════════════════════════════╣
║                                                 ║
║                                                 ║
║          [Empty state illustration]             ║  Scrollable
║                                                 ║  Messages
║          Hi! I'm CMAC, your roofing            ║  Area
║          assistant. How can I help?            ║
║                                                 ║
║                                                 ║
║                                                 ║
║                                                 ║
║                                                 ║
║                                                 ║
╠═════════════════════════════════════════════════╣
║  ┌────────────────────────────────────┐   ┌─┐ ║  Input
║  │ Type your message...               │   │🎤│ ║  Area
║  └────────────────────────────────────┘   └─┘ ║
╚═════════════════════════════════════════════════╝
```

### Main Window - Conversation Active
```
╔═════════════════════════════════════════════════╗
║ ◐ Listening...     Talk to CMAC            ⚙  ║
╠═════════════════════════════════════════════════╣
║                  [CMAC Logo]                    ║
║                Talk to CMAC                     ║
╠═════════════════════════════════════════════════╣
║                                                 ║
║  ┌────────────────────────────────────┐        ║
║  │ What's the status on the           │  User  ║
║  │ Johnson Street project?            │  Msg   ║
║  └────────────────────────────────────┘        ║
║                                12:34 PM         ║
║                                                 ║
║  ┌────────────────────────────────────┐        ║
║  │ The Johnson Street project is on   │  AI    ║
║  │ schedule. Roofing materials arrive │  Msg   ║
║  │ tomorrow morning.                  │        ║
║  │                                    │        ║
║  │ [🔊 Playing... ━━━━━●────]         │  Audio ║
║  └────────────────────────────────────┘        ║
║  12:34 PM                                       ║
║                                                 ║
║              ┌────────────────┐                 ║
║              │ Thanks CMAC!   │  User          ║
║              └────────────────┘                 ║
║                           12:35 PM              ║
║                                                 ║
╠═════════════════════════════════════════════════╣
║  ┌────────────────────────────────────┐   ┌─┐ ║
║  │ Type your message...               │   │🎤│ ║
║  └────────────────────────────────────┘   └─┘ ║
╚═════════════════════════════════════════════════╝
```

### Mic Button States
```
┌─────────────────────────────────────────────────┐
│                                                 │
│   IDLE          HOVER        LISTENING          │
│  ┌───┐         ┌───┐         ┌───┐             │
│  │   │         │   │         │ )) │             │
│  │🎤 │         │🎤 │         │🎤  │             │
│  │   │         │   │         │ )) │             │
│  └───┘         └───┘         └───┘             │
│  Gray          Blue          Pulsing           │
│                              Blue              │
│                                                 │
│  DISABLED      THINKING      ERROR              │
│  ┌───┐         ┌───┐         ┌───┐             │
│  │   │         │   │         │   │             │
│  │🎤 │         │⟳  │         │ ⚠ │             │
│  │   │         │   │         │   │             │
│  └───┘         └───┘         └───┘             │
│  Muted         Spinning      Shake             │
│  Gray          Purple        Red               │
│                                                 │
└─────────────────────────────────────────────────┘
```

### Settings Panel
```
╔════════════════════════════════════════════╗
║                                            ║
║   Settings                          ✕     ║
║   ────────                                 ║
║                                            ║
║   API Configuration                        ║
║   ┌──────────────────────────────────────┐ ║
║   │ API Key                              │ ║
║   │ ********************************     │ ║
║   └──────────────────────────────────────┘ ║
║                                            ║
║   Voice Selection                          ║
║   ┌──────────────────────────────────────┐ ║
║   │ Alloy (Natural) ▼                    │ ║
║   └──────────────────────────────────────┘ ║
║                                            ║
║   Push-to-Talk Hotkey                      ║
║   ┌──────────────────────────────────────┐ ║
║   │ Ctrl + M                             │ ║
║   └──────────────────────────────────────┘ ║
║                                            ║
║   Connection Status                        ║
║   ┌──────────────────────────────────────┐ ║
║   │ ✓ Connected to OpenAI               │ ║
║   │   Response time: 234ms              │ ║
║   └──────────────────────────────────────┘ ║
║                                            ║
║           ┌──────────────────┐             ║
║           │  Save Settings   │             ║
║           └──────────────────┘             ║
║                                            ║
╚════════════════════════════════════════════╝
```

---

## 14. Performance Considerations

### Animation Performance
```css
/* Use transform and opacity for animations (GPU-accelerated) */
.message-bubble {
  /* ✓ Good - GPU accelerated */
  transform: translateX(0);
  opacity: 1;

  /* ✗ Avoid - causes layout reflow */
  /* left: 0; */
  /* width: 100%; */
}

/* Use will-change for complex animations */
.mic-button.active {
  will-change: transform, opacity;
}

/* Remove will-change after animation */
.mic-button.idle {
  will-change: auto;
}
```

### Rendering Optimization
```typescript
// Use React.memo for message bubbles
export const MessageBubble = React.memo<MessageBubbleProps>(
  ({ message, type, timestamp }) => {
    // Component implementation
  },
  (prev, next) => {
    // Custom comparison
    return prev.message === next.message &&
           prev.type === next.type;
  }
);

// Virtual scrolling for large conversations (react-window)
import { FixedSizeList } from 'react-window';

<FixedSizeList
  height={500}
  itemCount={messages.length}
  itemSize={100}
  width="100%"
>
  {({ index, style }) => (
    <div style={style}>
      <MessageBubble message={messages[index]} />
    </div>
  )}
</FixedSizeList>
```

### Bundle Size
```
Target bundle size: < 500KB (gzipped)

Strategies:
- Code splitting by route
- Lazy load settings panel
- Optimize icon imports (use sprite sheet)
- Tree-shake unused CSS
- Use SVG icons vs icon font
```

---

## 15. Testing Checklist

### Visual Regression
```
□ Light mode renders correctly
□ Dark mode renders correctly
□ All button states visible
□ Message bubbles aligned properly
□ Animations smooth at 60fps
□ Glassmorphism effect visible
□ Logo displays correctly
□ Icons render at all sizes
```

### Interaction Testing
```
□ Click mic button → starts recording
□ Release mic → stops recording
□ Type + Enter → sends message
□ Settings icon → opens panel
□ Close settings → returns to main
□ Keyboard shortcuts work
□ Tab navigation follows logical order
□ Focus indicators visible
```

### Accessibility Audit
```
□ WCAG AAA color contrast met
□ Screen reader announces states
□ All interactive elements keyboard accessible
□ Focus trap in modal dialogs
□ Reduced motion respected
□ High contrast mode supported
□ Aria labels present and correct
□ Semantic HTML used throughout
```

### Cross-Platform
```
□ Windows 10 compatibility
□ Windows 11 compatibility
□ Different screen resolutions
□ Different DPI scaling (125%, 150%)
□ Multiple monitor setups
□ Taskbar positions (top, bottom, sides)
```

---

## Summary

This design system provides a complete blueprint for implementing the Talk to CMAC voice assistant with:

1. **Professional CMAC branding** - Blue/red color scheme for roofing industry
2. **Siri-inspired aesthetics** - Glassmorphism, smooth animations, modern UI
3. **Complete component specifications** - Exact dimensions, colors, styles
4. **Accessibility-first** - WCAG AAA compliant, keyboard navigation, screen reader support
5. **State management clarity** - Clear state diagram and interaction flows
6. **Implementation-ready** - CSS code examples, animation specs, file structure

**Next steps:**
1. Review color palette with CMAC branding team
2. Obtain official logo assets in required formats
3. Set up CSS variables and base styles
4. Begin component implementation following phase priorities
5. Regular testing against accessibility checklist

All specifications are developer-ready and can be implemented directly into the React + TypeScript + Tauri application.

---

**Design System Version:** 1.0
**Last Updated:** 2025-12-01
**Designer:** Claude (UI/UX Specialist)
**Project:** Talk to CMAC Voice Assistant
