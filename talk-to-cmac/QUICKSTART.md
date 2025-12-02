# Talk to CMAC - Quick Start Guide

Get up and running with the Talk to CMAC voice assistant in minutes.

## Prerequisites

- Node.js 18+ and npm
- Rust and Cargo (for Tauri)
- macOS, Windows, or Linux

## Installation

```bash
# Clone the repository (if not already done)
cd /Users/cojovi/dev/windows_gpt/talk-to-cmac

# Install dependencies
npm install

# Install Tauri CLI (if not already installed)
npm install -g @tauri-apps/cli
```

## Configuration

### 1. API Keys

You'll need API keys for:
- **Whisper API** (OpenAI or compatible)
- **OpenWebUI** (or LLM provider)
- **ElevenLabs** (text-to-speech)

Set them via the app settings UI (once running) or use environment variables:

```bash
export WHISPER_API_KEY="sk-..."
export OPENWEBUI_API_KEY="..."
export ELEVENLABS_API_KEY="..."
```

### 2. Configuration File

The app will create a config file automatically at:
- **macOS**: `~/Library/Application Support/com.cmac.talk-to-cmac/config.json`
- **Windows**: `%APPDATA%/TalkToCMAC/config.json`
- **Linux**: `~/.config/talk-to-cmac/config.json`

## Running the App

### Development Mode

```bash
# Run Tauri in development mode
npm run tauri dev
```

This will:
1. Start the Vite dev server (frontend)
2. Compile Rust backend
3. Open the application window
4. Enable hot-reload for frontend changes

### Production Build

```bash
# Build for production
npm run tauri build
```

The compiled app will be in:
- **macOS**: `src-tauri/target/release/bundle/macos/`
- **Windows**: `src-tauri/target/release/bundle/msi/`
- **Linux**: `src-tauri/target/release/bundle/appimage/`

## Usage

### Voice Mode

1. Click or hold the microphone button
2. Speak your question/command
3. Release the button
4. Wait for transcription and response
5. Listen to the audio response

### Text Mode

1. Type your message in the input box
2. Press Enter or click Send
3. View the response in the chat

### Keyboard Shortcuts

- **Enter** - Send text message
- **Shift+Enter** - New line in text input
- **Cmd/Ctrl+K** - Clear conversation (if enabled)

## Troubleshooting

### Microphone Not Working

1. Check browser/app permissions
2. Grant microphone access when prompted
3. Check System Preferences/Settings
4. Try restarting the app

### "Failed to check connectivity" Error

1. Verify API keys are set correctly
2. Check internet connection
3. Verify API endpoints are reachable
4. Check backend logs in terminal

### Audio Playback Issues

1. Check system volume
2. Verify audio device is working
3. Check console for errors
4. Try refreshing the app

### Backend Connection Failed

1. Make sure Rust backend compiled successfully
2. Check for port conflicts
3. Review terminal output for errors
4. Try `npm run tauri dev` again

## Development Workflow

### Frontend Changes

1. Edit files in `src/`
2. Changes hot-reload automatically
3. Check browser console for errors

### Backend Changes

1. Edit files in `src-tauri/src/`
2. Restart `npm run tauri dev`
3. Check terminal for Rust errors

### Type Checking

```bash
# Check TypeScript types
npx tsc --noEmit
```

### Linting (if configured)

```bash
# Frontend
npm run lint

# Backend
cd src-tauri && cargo clippy
```

## Project Structure

```
talk-to-cmac/
├── src/                    # React frontend
│   ├── components/         # UI components
│   ├── hooks/             # Custom hooks
│   ├── store/             # State management
│   ├── types/             # TypeScript types
│   ├── utils/             # Utilities
│   └── App.tsx            # Root component
│
├── src-tauri/             # Rust backend
│   ├── src/
│   │   ├── api/          # API clients
│   │   ├── commands.rs   # Tauri commands
│   │   ├── config.rs     # Configuration
│   │   ├── state.rs      # State management
│   │   └── lib.rs        # Entry point
│   └── Cargo.toml        # Rust dependencies
│
├── package.json           # Node dependencies
└── README.md             # This file
```

## Key Files

### Frontend
- `src/components/ChatWindow.tsx` - Main UI
- `src/store/useAppStore.ts` - State management
- `src/hooks/useTauri.ts` - Backend integration
- `src/utils/audio.ts` - Audio utilities

### Backend
- `src-tauri/src/commands.rs` - Tauri commands
- `src-tauri/src/api/whisper.rs` - Whisper integration
- `src-tauri/src/api/openwebui.rs` - LLM integration
- `src-tauri/src/api/elevenlabs.rs` - TTS integration

## Environment Variables

```bash
# API Keys (optional - can use settings UI)
export WHISPER_API_KEY="your-key-here"
export OPENWEBUI_API_KEY="your-key-here"
export ELEVENLABS_API_KEY="your-key-here"

# Logging (optional)
export RUST_LOG="info"  # or "debug", "warn", "error"
```

## Testing

### Manual Testing

1. Start the app
2. Test microphone recording
3. Test text input
4. Test voice query pipeline
5. Test settings changes
6. Test conversation clear
7. Test error handling

### Connectivity Check

Click the refresh button in the header to check all services:
- Green checkmark = Connected
- Red X = Disconnected
- Orange spinner = Checking

## Performance Tips

1. **Audio Quality**: Default 16kHz is optimal for Whisper
2. **Recording Length**: Keep under 60 seconds for best results
3. **LLM Context**: Conversation auto-trims to prevent context overflow
4. **Memory**: App uses ~200MB RAM typically

## Common Tasks

### Clear Conversation

Click the trash icon in the header or restart the app.

### Change Voice

Settings → Voice Settings → Select Voice (coming soon)

### Export Configuration

Configuration auto-saves to the system config directory.

### Update API Keys

Settings → API Keys → Update (coming soon)

Or use the `update_api_key` command programmatically.

## Getting Help

1. Check `FRONTEND_README.md` for detailed frontend docs
2. Check `TAURI_COMMANDS.md` for backend API reference
3. Check `BACKEND_IMPLEMENTATION.md` for backend details
4. Review console logs for errors
5. Check GitHub issues (if applicable)

## Next Steps

Once the app is running:

1. Configure your API keys
2. Test the microphone
3. Try a voice query
4. Send a text message
5. Explore the settings
6. Clear the conversation
7. Check connectivity status

## Contributing

When making changes:

1. Create a feature branch
2. Make your changes
3. Test thoroughly
4. Update documentation
5. Submit a pull request

## Resources

- [Tauri Documentation](https://tauri.app/)
- [React Documentation](https://react.dev/)
- [Zustand Documentation](https://github.com/pmndrs/zustand)
- [Whisper API Docs](https://platform.openai.com/docs/guides/speech-to-text)
- [ElevenLabs API Docs](https://elevenlabs.io/docs/)

## License

See LICENSE file in project root.

---

**Ready to start?** Run `npm run tauri dev` and start talking to CMAC!
