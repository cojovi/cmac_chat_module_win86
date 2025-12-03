# Fixes Applied - Environment Variables & System Tray

## Issues Fixed

### 1. ✅ Environment Variables Not Loading
**Problem**: The `.env` file was not being read by the Rust backend, so API keys weren't being loaded.

**Solution**:
- Added `dotenvy` crate to `Cargo.toml` for loading `.env` files
- Modified `src-tauri/src/lib.rs` to load `.env` at startup
- Updated API key loading logic to check environment variables **FIRST** before keyring
- Now checks multiple env var names (OPENAI_API_KEY, WHISPER_API_KEY, etc.)
- Filters out placeholder values (e.g., "your-api-key-here")

**Priority order for API keys**:
1. Environment variables from `.env` file
2. Alternative env var names (OPENAI_API_KEY for whisper, etc.)
3. System keyring (Windows Credential Manager)

### 2. ✅ Wrong OpenWebUI URL
**Problem**: App was using default URL instead of the one specified in `.env`.

**Solution**:
- Modified `AppConfig::default()` to read environment variables:
  - `OPENWEBUI_BASE_URL` → Used for OpenWebUI endpoint
  - `WHISPER_BASE_URL` → Used for Whisper endpoint
  - `ELEVENLABS_BASE_URL` → Used for ElevenLabs endpoint
  - `OPENWEBUI_MODEL_NAME` → Used for model selection
  - `WHISPER_MODEL` → Used for Whisper model
  - `ELEVENLABS_VOICE_ID` → Used for voice selection

### 3. ✅ Window Opening Instead of System Tray
**Problem**: App was opening as a full window instead of starting hidden in system tray.

**Solution**:
- Modified `src-tauri/tauri.conf.json`:
  - Set `visible: false` - window starts hidden
  - Set width/height to 420×650 (designed dimensions)
  - Added `trayIcon` configuration
  - Window only shows when clicking tray icon

---

## Changes Made

### Files Modified:

1. **`src-tauri/Cargo.toml`**
   - Added `dotenvy = "0.15"` dependency

2. **`src-tauri/src/lib.rs`**
   - Added `.env` file loading at startup
   ```rust
   match dotenvy::dotenv() {
       Ok(path) => log::info!("Loaded .env file from: {:?}", path),
       Err(e) => log::warn!("No .env file found: {}", e),
   }
   ```

3. **`src-tauri/src/config.rs`**
   - Enhanced `get_api_key()` to check environment variables first
   - Added smart filtering of placeholder values
   - Added better logging with ✓ and ✗ symbols
   - Updated `AppConfig::default()` to read env vars for endpoints

4. **`src-tauri/tauri.conf.json`**
   - Changed window `visible` to `false`
   - Set proper dimensions (420×650)
   - Added `trayIcon` configuration
   - Set `menuOnLeftClick: false` (right-click for menu)

5. **`.env.example`**
   - Added comprehensive API key configuration
   - Added all endpoint URLs
   - Added helpful comments and voice ID options
   - Included optional settings

6. **`.gitignore`**
   - Added `.env` files to prevent committing API keys

---

## How It Works Now

### On Startup:
1. App loads `.env` file from project root
2. Environment variables are set in memory
3. Config manager checks env vars for all settings
4. API keys loaded in this priority:
   - `OPENAI_API_KEY` → Whisper
   - `OPENWEBUI_API_KEY` → OpenWebUI
   - `ELEVENLABS_API_KEY` → ElevenLabs
5. Window starts hidden
6. System tray icon appears

### When You Click Tray Icon:
1. Window shows centered on screen
2. 420×650 dimensions (perfect for chat UI)
3. Ready for voice or text input

### API Key Loading:
The app will now log clearly what it's using:
```
✓ Using whisper from environment variable: OPENAI_API_KEY
✓ Using openwebui from environment variable: OPENWEBUI_API_KEY
✓ Using elevenlabs from environment variable: ELEVENLABS_API_KEY
```

Or if missing:
```
✗ No API key found for whisper (tried env vars and keyring): ...
```

---

## Testing the Fixes

### 1. Verify .env is being loaded:
```bash
cd /Users/cojovi/dev/windows_gpt/talk-to-cmac
cat .env  # Should show your API keys
npm run tauri dev
```

Look for this in the console:
```
Loaded .env file from: "/Users/cojovi/dev/windows_gpt/talk-to-cmac/.env"
✓ Using whisper from environment variable: OPENAI_API_KEY
```

### 2. Verify endpoints are correct:
The app should now connect to:
- Whisper: Value from `WHISPER_BASE_URL` or default
- OpenWebUI: Value from `OPENWEBUI_BASE_URL` (e.g., http://localhost:3000)
- ElevenLabs: Value from `ELEVENLABS_BASE_URL` or default

### 3. Verify system tray behavior:
- App should start with NO window visible
- System tray icon should appear
- Click tray icon → window appears
- Close window → minimizes back to tray (not exit)
- Right-click tray → menu with "Quit"

---

## Environment Variables Reference

### Required API Keys:
```bash
OPENAI_API_KEY=sk-your-actual-key-here
OPENWEBUI_API_KEY=your-actual-key-here
ELEVENLABS_API_KEY=your-actual-key-here
```

### Optional Endpoints (have sensible defaults):
```bash
WHISPER_BASE_URL=https://api.openai.com/v1
OPENWEBUI_BASE_URL=http://localhost:3000
ELEVENLABS_BASE_URL=https://api.elevenlabs.io/v1
```

### Optional Configuration:
```bash
WHISPER_MODEL=whisper-1
OPENWEBUI_MODEL_NAME=llama3.1:latest
ELEVENLABS_VOICE_ID=21m00Tcm4TlvDq8ikWAM
```

---

## Next Steps

1. **Edit your `.env` file** with real API keys
2. **Run the app**: `npm run tauri dev`
3. **Check the console** for the "✓" messages confirming env vars loaded
4. **Click the system tray icon** to show the window
5. **Test voice input** with your configured APIs

---

## Notes

- **For Development**: Use `.env` file (quick and easy)
- **For Production**: Use the Settings panel in the app (stores in Windows Credential Manager)
- **Environment variables take precedence** over keyring values
- **The .env file is gitignored** - safe to add real keys

---

## Troubleshooting

### "API key not found" errors:
1. Check `.env` file exists in project root
2. Check API keys don't have placeholder text
3. Check no extra quotes around values
4. Try running with `RUST_LOG=debug npm run tauri dev` for verbose logging

### "Wrong endpoint" errors:
1. Check `OPENWEBUI_BASE_URL` is set correctly
2. Don't include `/api/chat` - it's added automatically
3. Example: `http://localhost:3000` not `http://localhost:3000/api/chat`

### Window still showing on startup:
1. Check `tauri.conf.json` has `visible: false`
2. Restart the dev server
3. On Windows, behavior should be correct

---

**All fixes are complete and ready for testing!** 🎉
