//! Talk to CMAC - Voice Assistant Application
//!
//! A Tauri-based voice assistant that integrates Whisper (speech-to-text),
//! OpenWebUI (LLM), and ElevenLabs (text-to-speech) for natural voice interactions.

// Module declarations
mod api;
mod commands;
mod config;
mod error;
mod state;

use config::{AppConfig, ConfigManager};
use state::AppState;
use tauri::Manager;

/// Initialize and run the Tauri application
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Initialize logger
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    log::info!("Starting Talk to CMAC application");

    tauri::Builder::default()
        // Register plugins
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        // Setup hook to initialize application state
        .setup(|app| {
            log::info!("Initializing application state");

            // Load configuration
            let config_manager = ConfigManager::new().map_err(|e| {
                log::error!("Failed to create config manager: {}", e);
                e.to_string()
            })?;

            let (config, api_keys) = config_manager.load_with_keys().unwrap_or_else(|e| {
                log::warn!("Failed to load config, using defaults: {}", e);
                (AppConfig::default(), config::ApiKeys {
                    whisper: None,
                    openwebui: None,
                    elevenlabs: None,
                })
            });

            log::info!("Configuration loaded");

            // Create application state
            let app_state = AppState::new(config.clone(), api_keys);

            // Manage state
            app.manage(app_state);

            // Setup system tray if on desktop
            #[cfg(desktop)]
            {
                use tauri::tray::{TrayIconBuilder, MouseButton, MouseButtonState};
                use tauri::menu::{MenuBuilder, MenuItemBuilder};

                log::info!("Setting up system tray");

                // Create tray menu
                let menu = MenuBuilder::new(app)
                    .item(&MenuItemBuilder::new("Show").id("show").build(app)?)
                    .item(&MenuItemBuilder::new("Hide").id("hide").build(app)?)
                    .separator()
                    .item(&MenuItemBuilder::new("Quit").id("quit").build(app)?)
                    .build()?;

                let _tray = TrayIconBuilder::new()
                    .menu(&menu)
                    .on_menu_event(|app, event| {
                        match event.id().as_ref() {
                            "show" => {
                                if let Some(window) = app.get_webview_window("main") {
                                    let _ = window.show();
                                    let _ = window.set_focus();
                                }
                            }
                            "hide" => {
                                if let Some(window) = app.get_webview_window("main") {
                                    let _ = window.hide();
                                }
                            }
                            "quit" => {
                                app.exit(0);
                            }
                            _ => {}
                        }
                    })
                    .on_tray_icon_event(|tray, event| {
                        if let tauri::tray::TrayIconEvent::Click {
                            button: MouseButton::Left,
                            button_state: MouseButtonState::Up,
                            ..
                        } = event
                        {
                            let app = tray.app_handle();
                            if let Some(window) = app.get_webview_window("main") {
                                let _ = window.show();
                                let _ = window.set_focus();
                            }
                        }
                    })
                    .build(app)?;

                log::info!("System tray initialized");
            }

            // Setup global hotkey
            if let Some(hotkey) = &config.ui.global_hotkey {
                log::info!("Registering global hotkey: {}", hotkey);

                // Note: Global hotkey registration needs to be done after the app is running
                // This is a placeholder for the actual implementation
                // In practice, you would use tauri_plugin_global_shortcut
            }

            // Configure window
            if let Some(window) = app.get_webview_window("main") {
                if config.ui.always_on_top {
                    let _ = window.set_always_on_top(true);
                }
                log::info!("Main window configured");
            }

            log::info!("Application setup complete");
            Ok(())
        })
        // Register all Tauri commands
        .invoke_handler(tauri::generate_handler![
            commands::process_audio,
            commands::send_message,
            commands::synthesize_speech,
            commands::process_voice_query,
            commands::load_config,
            commands::save_config,
            commands::update_api_key,
            commands::check_connectivity,
            commands::get_app_state,
            commands::clear_conversation,
            commands::get_conversation,
            commands::list_voices,
            commands::update_voice_settings,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_loading() {
        let config_manager = ConfigManager::new();
        assert!(config_manager.is_ok());
    }

    #[test]
    fn test_state_creation() {
        let config = AppConfig::default();
        let api_keys = config::ApiKeys {
            whisper: None,
            openwebui: None,
            elevenlabs: None,
        };
        let state = AppState::new(config, api_keys);
        assert_eq!(state.get_status(), state::AppStatus::Idle);
    }
}
