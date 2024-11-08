// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod app;
mod types;
use anyhow::Context;
use app::cmd::{create_lol_champions_obsidian_file, greet, set_obsidian_vault_path};
use types::state::AppState;
use std::sync::Mutex;
use tauri::Manager;
use tauri_plugin_store::StoreBuilder;
use tauri_plugin_log::{Target,TargetKind};


fn main() -> anyhow::Result<()> {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greet,
            create_lol_champions_obsidian_file,
            set_obsidian_vault_path
        ])
        .plugin(tauri_plugin_log::Builder::default().build())
        .setup(|app| {
            let config_path = app
                .handle()
                .path()
                .app_config_dir()
                .with_context(|| format!("Failed to load app config dir."))?
                .join("store.bin");
            StoreBuilder::new(app.handle(), config_path).build()?;
            Ok(())
        })
        .run(tauri::generate_context!())?;
    Ok(())
}
