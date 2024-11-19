// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use crate::types::error::TauriError;
use crate::types::state::AppState;
use serde::Serialize;
use std::sync::Mutex;
use tauri::State;

#[tauri::command]
pub fn set_obsidian_vault_path(
    vault_path: String,
    state: State<'_, Mutex<AppState>>,
) -> Result<(), TauriError>
where
    Result<(), TauriError>: Serialize,
{
    let store = &mut state.lock()?.store;
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
