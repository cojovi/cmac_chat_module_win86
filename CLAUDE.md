# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Windows desktop voice assistant application ("Talk to CMAC") that provides a Siri-like interface for interacting with an OpenWebUI chatbot. The application features speech-to-text, AI reasoning, and text-to-speech capabilities in a system tray application.

## Core Architecture

The application follows a multi-stage voice pipeline:

1. **Audio Capture** → User speaks via push-to-talk (hotkey or button)
2. **Speech-to-Text** → Audio sent to OpenAI Whisper API for transcription
3. **AI Reasoning** → Transcribed text sent to OpenWebUI backend
4. **Text-to-Speech** → AI response converted to audio via ElevenLabs API
5. **Audio Playback** → Synthesized speech played back to user

### Key Components

- **UI Layer**: System tray application with popup interface for chat/voice input
- **Audio Capture Module**: Records microphone input on push-to-talk activation
- **STT Module**: Integrates OpenAI Whisper API for speech transcription
- **AI Integration**: Connects to OpenWebUI backend (local or remote)
- **TTS Module**: Uses ElevenLabs API for high-quality voice synthesis
- **Audio Playback**: Handles playing synthesized speech responses

## Technology Stack

### Framework Options
The design document evaluates three frameworks:

- **Tauri** (Recommended): Lightweight (~8.6 MB bundle, ~172 MB RAM), uses native WebView2, Rust backend
- **Electron** (Alternative): Larger footprint (~244 MB bundle, ~409 MB RAM), includes Chromium
- **WPF** (Native): Windows-only, .NET/C#, most efficient but requires XAML expertise

**Decision**: Prefer Tauri for the best balance of modern UI capabilities and resource efficiency. The web-based frontend (React/Vue/Angular) enables elegant UI development while the Rust backend provides performance.

## Configuration

The application requires the following environment variables:

```
OPENWEBUI_BASE_URL=          # URL to OpenWebUI instance (no trailing slash)
OPENWEBUI_API_KEY=           # API key from OpenWebUI
OPENWEBUI_MODEL_NAME=        # Exact model name from Workspace → Models
OPENWEBUI_KNOWLEDGE_COLLECTION_ID=  # Optional knowledge collection ID
```

Additional API credentials needed:
- OpenAI API key (for Whisper STT)
- ElevenLabs API key (for TTS with multiple voice options)

## User Interaction Model

### Push-to-Talk Activation
- **No hotword detection** ("Hey CMAC" is NOT implemented)
- **UI Button**: Press and hold microphone button to record
- **Global Hotkey**: System-wide keyboard shortcut for voice activation
- **System Tray**: Click tray icon to open chat interface

### Interaction Modes
1. **Voice Input**: Push-to-talk → transcription → AI response → spoken reply
2. **Text Input**: Type query in chat window → AI response → optional TTS playback

## Data Handling

### No Persistent Storage
- **No chat history saved to disk** (ephemeral conversations only)
- Context maintained in memory during active session only
- Clears on application restart
- Minimal logging for debugging (excludes conversation content)

### Configuration Persistence
- API keys and preferences stored locally (encrypted)
- User settings (voice selection, hotkey preferences, volume)
- Stored in AppData or similar OS-appropriate location

## Offline Behavior

The application gracefully handles offline scenarios:

- **Detection**: Uses `navigator.onLine` (web) or network APIs (native)
- **Visual Indicators**: Tray icon changes to show offline status
- **User Feedback**: Clear "Offline" message when services unavailable
- **Fallback**: Display text responses if TTS fails, but transcription available

### API Failure Handling
- **Whisper API failure**: Show transcription error, retry option
- **AI Model failure**: Display "Assistant unavailable" message
- **ElevenLabs failure**: Display text response only, notify voice synthesis failed

## Distribution

### Internal Deployment
- Single office use (no public distribution)
- Standard Windows installer (MSI/NSIS) for proper system integration
- Creates Start Menu shortcuts and Add/Remove Programs entry
- Portable exe/zip available for development/testing
- No code signing required for internal use (but recommended to avoid SmartScreen)

### Framework-Specific Packaging
- **Tauri**: Uses `tauri-bundler` with WiX for MSI
- **Electron**: Uses Electron Builder/Forge for Setup.exe
- **WPF**: MSI via WiX or simple self-contained exe

## Development Guidelines

### API Integration Patterns
- All API calls must be **asynchronous** (network requests can take seconds)
- Implement **reasonable timeouts** (prevent indefinite waits on failures)
- Use **error handling** for each pipeline stage (STT, AI, TTS)
- Provide **visual feedback** for each stage ("Listening...", "Thinking...", "Speaking...")

### UI/UX Requirements
- **Visual feedback**: Clear indicators for recording, processing, speaking states
- **Interruptible playback**: Allow user to stop TTS output mid-speech
- **Elegant styling**: Modern, clean interface (Siri-like aesthetic)
- **CMAC branding**: Use CMAC Roofing icon and "Talk to CMAC" text

### Audio Processing
- Capture format: 16 kHz 16-bit PCM WAV (standard for speech recognition)
- Hold-to-talk preferred over toggle (cleaner audio boundaries)
- No continuous audio monitoring (push-to-talk only)

### System Integration
- System tray icon with right-click context menu
- Minimize to tray (don't exit on window close)
- Global hotkey registration (OS-level)
- Optional auto-start on Windows login

## Key Design Decisions

1. **No hotword detection**: Avoids complexity, privacy concerns, and continuous processing overhead
2. **Cloud APIs over local**: Prioritizes quality (Whisper/ElevenLabs) over offline capability
3. **Ephemeral sessions**: No history storage maintains privacy and simplicity
4. **Push-to-talk**: Provides explicit control, simpler implementation, better audio boundaries
5. **Web-based UI framework**: Enables rapid development of polished, modern interface

## Common Pitfalls to Avoid

- Don't implement features beyond the scope (no hotword detection, no history storage)
- Don't use Web Speech API (lower quality than Whisper/ElevenLabs)
- Don't create complex silence detection for voice (use push-and-hold instead)
- Don't store conversation content on disk (privacy requirement)
- Ensure all network operations have timeouts and error handling

## Future Considerations (Out of Current Scope)

- Hotword activation ("Hey CMAC")
- Offline mode with local STT/TTS
- Chat history persistence
- Auto-update mechanism
- Multi-language support
- Voice customization beyond ElevenLabs voice selection
