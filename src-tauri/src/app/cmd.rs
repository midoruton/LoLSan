// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use serde::Serialize;
use std::sync::Mutex;
use tauri::{State};
use crate::types::error::TauriError;
use crate::types::state::AppState;

#[tauri::command]
pub async fn set_obsidian_vault_path(
    vault_path: String,
    state: State<'_, Mutex<AppState>>,
) -> Result<(), TauriError>
where
    Result<(), TauriError>: Serialize, // Add this trait bound
{
    let mut lock = state.lock();
    let store = match lock {
        Ok(ref mut s) => &mut s.store,
        Err(e) => return Err(TauriError::from(e)),
    };
    match store.insert("vault_path".into(), vault_path.into()) {
        Ok(_) => {}
        Err(e) => return Err(TauriError::from(e)),
    }
    match store.save() {
        Ok(_) => {}
        Err(e) => return Err(TauriError::from(e)),
    }
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
    match fetch_data().await {
        Ok(data) => Ok(data),
        Err(e) => Err(TauriError::from(e)),
    }
}

#[tauri::command]
pub async fn create_lol_champions_obsidian_file(champion_name: String) -> Result<(), TauriError>
where
    Result<(), TauriError>: Serialize,
{
    match open::that(format!(
        "obsidian://open?vault=LeagueOfLegends&name={}",
        champion_name
    )) {
        Ok(_) => Ok(()),
        Err(e) => Err(TauriError::from(e)),
    }
    //match open::that(format!("obsidian://new?vault=LeagueOfLegends&name={}",champion_name)){
    //        Ok(_)=>Ok(()),
    //        Err(e)  =>Err(TauriError::Io(e)),
    //}
}