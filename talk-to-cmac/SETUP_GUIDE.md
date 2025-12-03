# Talk to CMAC - Setup Guide

## Quick Start (5 minutes)

### Step 1: Get Your API Keys

You need three API keys to run this application:

#### 1. OpenAI API Key (for Whisper Speech-to-Text)
- Go to: https://platform.openai.com/api-keys
- Click "Create new secret key"
- Copy the key (starts with `sk-...`)
- Cost: ~$0.006 per minute of audio

#### 2. OpenWebUI API Key
- If running locally: http://localhost:3000/settings
- Go to Settings → Account → API Keys
- Generate new API key
- Copy the key

#### 3. ElevenLabs API Key (for Text-to-Speech)
- Go to: https://elevenlabs.io/app/settings/api-keys
- Click "Create API Key"
- Copy the key
- Cost: Free tier = 10,000 characters/month, Paid starts at $5/month

---

### Step 2: Configure the App

You have **two options** for configuring API keys:

#### Option A: Environment Variables (Quick Testing)

1. Copy the example file:
   ```bash
   cp .env.example .env
   ```

2. Edit `.env` and add your keys:
   ```bash
   # OpenAI Whisper
   OPENAI_API_KEY=sk-your-actual-openai-key-here

   # OpenWebUI
   OPENWEBUI_BASE_URL=http://localhost:3000
   OPENWEBUI_API_KEY=your-openwebui-key-here
   OPENWEBUI_MODEL_NAME=llama3.1:latest

   # ElevenLabs
   ELEVENLABS_API_KEY=your-elevenlabs-key-here
   ELEVENLABS_VOICE_ID=21m00Tcm4TlvDq8ikWAM
   ```

3. **Important**: Never commit `.env` to git! (It's already in `.gitignore`)

#### Option B: System Keyring (Secure, Recommended for Production)

1. Run the app without API keys configured
2. Open Settings panel in the UI
3. Enter your API keys in the settings
4. They'll be stored securely in Windows Credential Manager
5. Keys persist across app restarts

**Note**: Environment variables take precedence over keyring values.

---

### Step 3: Install Dependencies

```bash
cd /Users/cojovi/dev/windows_gpt/talk-to-cmac
npm install
```

---

### Step 4: Run the App

#### Development Mode (with hot reload)
```bash
npm run tauri dev
```

This will:
1. Start the Vite dev server (React frontend)
2. Compile the Rust backend
3. Launch the Tauri window
4. Show the system tray icon

#### Production Build
```bash
npm run tauri build
```

This creates:
- Windows EXE: `src-tauri/target/release/talk-to-cmac.exe`
- MSI Installer: `src-tauri/target/release/bundle/msi/`
- NSIS Installer: `src-tauri/target/release/bundle/nsis/`

---

## Testing the Voice Pipeline

Once the app is running:

1. **Click the system tray icon** (or it should open automatically)
2. **Test text input first**:
   - Type "Hello, how are you?" in the text box
   - Press Enter
   - Should call OpenWebUI and get a response
   - Should play TTS audio via ElevenLabs

3. **Test voice input**:
   - Click and hold the microphone button
   - Speak clearly: "What's the weather like?"
   - Release the button
   - Should transcribe via Whisper
   - Should get LLM response via OpenWebUI
   - Should play TTS audio via ElevenLabs

---

## Configuration Tips

### OpenWebUI Setup

If you don't have OpenWebUI running:

```bash
# Using Docker (easiest)
docker run -d -p 3000:8080 \
  -v open-webui:/app/backend/data \
  --name open-webui \
  ghcr.io/open-webui/open-webui:main
```

Then go to http://localhost:3000, create an account, and generate an API key.

### ElevenLabs Voice Selection

Popular voice options (change `ELEVENLABS_VOICE_ID`):

- **Rachel** (default): `21m00Tcm4TlvDq8ikWAM` - Calm, professional female
- **Bella**: `EXAVITQu4vr4xnSDxMaL` - Soft, friendly female
- **Antoni**: `ErXwobaYiN019PkySvjV` - Well-rounded male
- **Arnold**: `VR6AewLTigWG4xSOukaG` - Crisp, authoritative male
- **Josh**: `TxGEqnHWrfWFTfGW9XjX` - Deep, confident male
- **Domi**: `AZnzlk1XvdvUeBnXmlld` - Strong, assertive female

List all available voices:
```bash
curl https://api.elevenlabs.io/v1/voices \
  -H "xi-api-key: YOUR_API_KEY"
```

### Whisper Model Options

The default is `whisper-1` (based on large-v2 model). No other options needed for the API.

For local Whisper (if you want offline):
- `tiny` - Fastest, least accurate
- `base` - Good balance
- `small` - Better accuracy
- `medium` - High accuracy
- `large` - Best accuracy (requires GPU)

---

## Troubleshooting

### App won't start
- Check that Rust is installed: `rustc --version`
- Check that Node is installed: `node --version`
- Run `npm install` again
- Try `cargo clean` in the `src-tauri/` directory

### "API key not found" error
- Check your `.env` file has the correct key format
- Make sure `.env` is in the project root
- Try setting keys via the UI Settings panel instead

### Whisper API fails
- Verify your OpenAI API key at https://platform.openai.com/api-keys
- Check you have credits available in your OpenAI account
- Ensure audio is being recorded (check browser permissions)

### ElevenLabs API fails
- Verify your ElevenLabs API key at https://elevenlabs.io/app/settings/api-keys
- Check you haven't exceeded your character quota
- Try with a different voice ID

### OpenWebUI connection fails
- Verify OpenWebUI is running: `curl http://localhost:3000/api/health`
- Check the base URL in `.env` matches your setup
- Verify the API key is valid in OpenWebUI settings

### Audio not recording
- Check microphone permissions in browser/system settings
- Try a different browser (Chromium-based recommended)
- Check console for error messages

### No audio playback
- Check system volume is not muted
- Verify default audio output device is correct
- Check browser audio permissions

---

## Environment Variables Reference

### Required
- `OPENAI_API_KEY` - Your OpenAI API key
- `OPENWEBUI_BASE_URL` - OpenWebUI instance URL
- `OPENWEBUI_API_KEY` - OpenWebUI API key
- `ELEVENLABS_API_KEY` - ElevenLabs API key

### Optional (have sensible defaults)
- `WHISPER_BASE_URL` - Default: `https://api.openai.com/v1`
- `WHISPER_MODEL` - Default: `whisper-1`
- `OPENWEBUI_MODEL_NAME` - Default: `llama3.1:latest`
- `ELEVENLABS_BASE_URL` - Default: `https://api.elevenlabs.io/v1`
- `ELEVENLABS_VOICE_ID` - Default: `21m00Tcm4TlvDq8ikWAM` (Rachel)
- `RUST_LOG` - Default: `info` (options: `debug`, `info`, `warn`, `error`)
- `VITE_WINDOW_WIDTH` - Default: `420`
- `VITE_WINDOW_HEIGHT` - Default: `650`
- `VITE_GLOBAL_HOTKEY` - Default: `CommandOrControl+Shift+C`
- `VITE_API_TIMEOUT` - Default: `30` seconds
- `VITE_RETRY_ATTEMPTS` - Default: `3`

---

## Production Deployment

### For Internal Office Use:

1. **Build the installer**:
   ```bash
   npm run tauri build
   ```

2. **Locate the installer**:
   - MSI: `src-tauri/target/release/bundle/msi/talk-to-cmac_0.1.0_x64_en-US.msi`
   - NSIS: `src-tauri/target/release/bundle/nsis/talk-to-cmac_0.1.0_x64-setup.exe`

3. **Distribute**:
   - Copy installer to network share
   - OR put on USB drive
   - OR email (if file size allows)

4. **Install on each PC**:
   - Run the installer
   - First launch will prompt for API keys (use Settings panel)
   - Keys stored securely in Windows Credential Manager
   - Won't need to re-enter after updates

5. **Optional: Code Signing** (prevents SmartScreen warnings):
   - Get a code signing certificate ($50-300/year)
   - Sign the installer with `signtool.exe`
   - For internal use, this is optional

---

## Support & Documentation

- **Complete docs**: See `/docs` folder (20+ markdown files)
- **API reference**: `TAURI_COMMANDS.md`
- **Design system**: `DESIGN_SYSTEM.md`
- **Frontend guide**: `FRONTEND_README.md`
- **Backend architecture**: `BACKEND_ARCHITECTURE.md`
- **Project overview**: `PROJECT_SUMMARY.md`

---

## Cost Estimates

### Per Voice Interaction (Average):
- **Whisper (STT)**: $0.006 per minute = ~$0.001 per 10-second query
- **OpenWebUI (LLM)**: Free if self-hosted, varies if using external API
- **ElevenLabs (TTS)**: ~$0.003 per response (100 characters ≈ $0.0003)

**Total per interaction**: ~$0.004 (less than half a cent!)

### Monthly (100 interactions/day):
- Whisper: $3
- OpenWebUI: $0 (self-hosted)
- ElevenLabs: $9 (or free tier if <10k chars/month)

**Total**: ~$12/month for heavy use

---

## Next Steps

1. ✅ Copy `.env.example` to `.env`
2. ✅ Add your three API keys
3. ✅ Run `npm install`
4. ✅ Run `npm run tauri dev`
5. ✅ Test voice and text interactions
6. ✅ Customize CMAC icon in `/public`
7. ✅ Build installer for office deployment

---

**Congratulations! Your CMAC voice assistant is ready to use!** 🎉
