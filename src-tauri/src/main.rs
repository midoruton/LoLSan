// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod app;
mod types;
use app::cmd::{create_lol_champions_obsidian_file, greet, set_obsidian_vault_path};
use types::state::AppState;
use std::sync::Mutex;
use tauri::Manager;
use tauri_plugin_store::StoreBuilder;




fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greet,
            create_lol_champions_obsidian_file,
            set_obsidian_vault_path
        ])
        .setup(|app| {
            let config_path = app
                .handle()
                .path_resolver()
                .app_config_dir()
                .unwrap()
                .join("store.bin");
            app.manage(Mutex::new(AppState {
                store: StoreBuilder::new(app.handle(), config_path).build(),
            }));
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
