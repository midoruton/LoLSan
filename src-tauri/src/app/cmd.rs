// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use crate::types::error::LoLSanError;
use serde::Serialize;
use tauri::AppHandle;
use tauri::Runtime;
use tauri::State;
use tauri_plugin_store::StoreExt;
use crate::app::state::AppState;
use std::ops::Deref;
use std::ops::DerefMut;
use tokio::sync::Mutex;
use std::sync::Arc;
use super::logic::fetch::{fetch, validate};
use super::state;
#[tauri::command]
pub fn set_obsidian_vault_path<R: Runtime>(
    vault_path: String,
    app: AppHandle<R>,
) -> Result<(), LoLSanError>
where
    Result<(), LoLSanError>: Serialize,
{
    let config_path = super::super::util::path::get_config_file_path(&app)?;
    let store = app.store(config_path)?;
    store.set("vault_path", vault_path);
    store.save()?;
    Ok(())
}

const ALL_GAME_DATA_SCHEMA_STR: &str = std::include_str!("../../../src/schema/AllGameData.json");

#[tauri::command]
pub async fn start_get_liveclient_data_loop(my_state_arc: tauri::State<'_, AppState>) -> Result<(), LoLSanError>
where
    LoLSanError: Serialize,
{
    log::debug!("Starting get_liveclient_data_loop_command");
    let schema = serde_json::from_str::<serde_json::Value>(ALL_GAME_DATA_SCHEMA_STR)?;
    let test = Arc::clone(&my_state_arc.getting_liveclient_data_loop_mutex);
    tauri::async_runtime::spawn(async move {
        let lock = test.lock().await;
        log::debug!("Starting get_liveclient_data_loop");
        loop { 
            let url = "https://127.0.0.1:2999/liveclientdata/allgamedata";
            log::debug!("Fetching data from: {}", url);
            let responce = match fetch(url, true).await {
                Ok(a) => a,
                Err(e) => {
                    //TODO: handle error by status code
                    log::error!("Error while fetching data: {}", e);
                    continue;
                }
            };
            log::debug!("Data fetched: {}", responce);
            log::debug!("Validating data with schema: {}", schema);
            let valid_responce = match validate(&schema, &responce).await {
                Ok(_) => responce,
                Err(e) => {
                    log::error!("Error while validating data: {}", e);
                    std::thread::sleep(std::time::Duration::from_secs(2));
                    continue;
                }
            };
            log::info!("Data fetched and validated: {}", valid_responce);
            //ここでlockを解放したい
            drop(lock); 
            break;
        }
    });
    Ok(())
}
