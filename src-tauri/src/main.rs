// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod app;
mod types;
mod util;

use app::{cmd::{set_obsidian_vault_path, start_get_liveclient_data_loop}, state::AppState};
use app::event::liveclient_data_event;
use tauri::{Listener, Manager};
use tauri_plugin_log::{self, Target, TargetKind};
fn main() -> anyhow::Result<()> {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(
            tauri_plugin_log::Builder::new()
                .targets([
                    Target::new(TargetKind::Webview),
                    Target::new(TargetKind::LogDir {
                        file_name: Some("logs".to_string()),
                    }),
                ])
                .level(log::LevelFilter::Debug)
                .build(),
        )
        .plugin(tauri_plugin_store::Builder::default().build())
        .setup(|app| {
            // Initialize store
            let config_path = util::path::get_config_file_path(app.handle())?;
            let _ = tauri_plugin_store::StoreBuilder::new(app, config_path).build()?;
            log::info!("store initialized");
            // Initialize state
            let my_state_arc = AppState::default();
            app.manage(my_state_arc);
            log::info!("state initialized");
            // Subscribe to events
            let handle = app.handle().clone();
            app.listen(
                "liveclient_data_event",
                 move|_|{
                    liveclient_data_event(&handle);
                });
            
            log::info!("events subscribed");
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            start_get_liveclient_data_loop,
            set_obsidian_vault_path
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    
    Ok(())
}
