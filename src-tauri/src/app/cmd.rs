// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use crate::types::error::LoLSanError;
use serde::Serialize;
use tauri::{AppHandle,Emitter};
use tauri::Runtime;
use tauri_plugin_store::StoreExt;
use crate::app::state::{AppState,LeagueClientState};
use std::sync::Arc;
use super::logic::fetch::{fetch, validate};

#[tauri::command]
pub fn set_obsidian_vault_path<R: Runtime>(
    vault_path: String,
    app: AppHandle<R>,
) -> Result<(), LoLSanError>
where
    Result<(), LoLSanError>: Serialize,
{
    let config_path = crate::util::path::get_config_file_path(&app)?;
    let store = app.store(config_path)?;
    store.set("vault_path", vault_path);
    store.save()?;
    Ok(())
}

const ALL_GAME_DATA_SCHEMA_STR: &str = std::include_str!("../../../src/schema/AllGameData.json");

#[tauri::command]
pub async fn start_get_liveclient_data_loop(my_state: tauri::State<'_, AppState>,app:AppHandle) -> Result<(), LoLSanError>
where
    LoLSanError: Serialize,
{
    log::debug!("Starting get_liveclient_data_loop_command");
    let schema = serde_json::from_str::<serde_json::Value>(ALL_GAME_DATA_SCHEMA_STR)?;
    log::debug!("Schema loaded: {}", schema);
    let access_mutex = Arc::clone(&my_state.liveclinet_data_access_mutex);
    let league_state_mutex = Arc::clone(&my_state.league_client_state);
    //https://stackoverflow.com/questions/77154162/how-to-use-a-managed-tauri-state-variable-inside-a-spawned-tauri-async-runtime-t
    tauri::async_runtime::spawn(async move {
        let loop_lock = access_mutex.try_lock();
        match loop_lock {
            None => {
                log::warn!("get_liveclient_data_loop already running");
            }
            Some(_) => {
                log::debug!("Starting get_liveclient_data_loop");
                loop { 
                    let mut league_state_lock = league_state_mutex.lock().await;
                    let url = "https://127.0.0.1:2999/liveclientdata/allgamedata";
                    log::debug!("Fetching data from: {}", url);
                    let response = match fetch(url, true).await {
                        Ok(a) => a,
                        Err(e) => {
                            //TODO: handle error by status code
                            log::warn!("Error while fetching data: {}", e);
                            *league_state_lock = LeagueClientState::Disconnected;
                            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                            continue;
                        }
                    };
                    log::debug!("Data fetched: {}", response);
                    log::debug!("Validating data with schema: {}", schema);
                    let valid_response = match validate(&schema, &response).await {
                        Ok(_) => response,
                        Err(e) => {
                            log::warn!("Error while validating data: {}", e);
                            *league_state_lock = LeagueClientState::Connected;
                            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                            continue;
                        }
                    };
                    log::info!("Data fetched and validated: {}", valid_response);
                    *league_state_lock = LeagueClientState::InGame(Arc::new(valid_response));
                    drop(league_state_lock);
                    app.emit("liveclient_data_event", ()).expect("failed to emit liveclient_data");
                    tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
                    continue;
                }
            }
        }
    });

    Ok(())
}
