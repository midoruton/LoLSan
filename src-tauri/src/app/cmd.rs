// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use serde::Serialize;
use std::sync::Mutex;
use tauri::State;
use crate::types::error::TauriError;
use crate::types::state::AppState;


#[tauri::command]
pub fn set_obsidian_vault_path(
    vault_path: String,
    state: State<'_, Mutex<AppState>>,
) -> Result<(), TauriError>
where
    Result<(), TauriError>: Serialize,
{
    let store = &mut state.lock()?.store;
    store.insert("vault_path".into(), vault_path.into())?;
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

#[tauri::command]
pub async fn create_lol_champions_obsidian_file(champion_name: String) -> Result<(), TauriError>
where
    Result<(), TauriError>: Serialize,
{
    open::that(format!(
        "obsidian://open?vault=LeagueOfLegends&name={}",
        champion_name
    ))?;
    Ok(())
}