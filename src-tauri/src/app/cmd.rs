// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use crate::types::error::TauriError;
use crate::types::state::AppState;
use serde::Serialize;
use tauri_plugin_store::StoreExt;
use std::sync::Mutex;
use tauri::State;
use tauri::AppHandle;
use tauri::Runtime;
#[tauri::command]
pub fn set_obsidian_vault_path<R:Runtime>(
    vault_path: String,
    app: AppHandle<R>
) -> Result<(), TauriError>
where
    Result<(), TauriError>: Serialize,
{
    let config_path = super::super::util::path::get_config_file_path(&app)?;
    let   store   = app.store(config_path)?;
    store.set("vault_path", vault_path);
    store.save()?;
    Ok(())
}

async fn fetch_data() -> Result<String, reqwest::Error> {
    let res = reqwest::get("https://localhost").await?;
    res.text().await
}
#[tauri::command]
pub async fn greet() -> Result<String, TauriError>
where
    Result<String, TauriError>: Serialize, // Add this trait bound
{
    let str = fetch_data().await?;
    Ok(str)
}
