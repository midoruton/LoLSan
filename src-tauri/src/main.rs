// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod app;
mod types;
mod util;
use anyhow::Context;
use app::cmd::{get_liveclient_data,set_obsidian_vault_path};
use tauri::Manager;
use tauri_plugin_store::{StoreBuilder, StoreExt};

fn main() -> anyhow::Result<()> {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![get_liveclient_data,set_obsidian_vault_path])
        .plugin(tauri_plugin_log::Builder::default().build())
        .plugin(tauri_plugin_store::Builder::default().build())
        .setup(|app| {
            let config_path = util::path::get_config_file_path(app.handle())?;
            let _ =  tauri_plugin_store::StoreBuilder::new(app,config_path).build()?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    Ok(())
}
