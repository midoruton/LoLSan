use std::ops::Deref;

use tauri::{AppHandle, Manager};
use tauri_plugin_store::StoreExt;
use crate::app::state::{AppState,LeagueClientState};

pub fn liveclient_data_event(app:&AppHandle){
    log::info!("liveclient_data_event");
    let arc = app.state::<AppState>().league_client_state.clone();
    let client_state
        = match arc.try_lock(){
            None => {
                log::warn!("liveclient_data_event already running");
                return;
            },
            Some(a) => a,
        };
    let payload = match &client_state.deref(){
        LeagueClientState::InGame(payload) => payload,
        _ => {
            log::error!("liveclient_data_event called while not in game: {}", &client_state.deref());
            return;
        }
    };
    log::info!("liveclient_data {}", payload.to_string());
    let config_path = match crate::util::path::get_config_file_path(&app){
        Ok(a) => a,
        Err(e) => {
            log::error!("failed to get config file path: {}", e);
            return;
        }
    };
    let valut_path = match app.store(config_path){
        Ok(store) => {
            match store.get("vault_path") {
                Some(a) => a,
                None => {
                    log::error!("vault_path not found in store");
                    return;
                }
            }
        },
        Err(e) => {
            log::error!("failed to get store: {}", e);
            return;
        }
    };
    log::debug!("valut path: {}", valut_path);
    #[allow(non_snake_case)]
    let riotId  = match payload["activePlayer"]["riotId"].as_str(){
        Some(a) => a,
        None => {
            log::error!("riotId not found in payload");
            return;
        }
    };
    log::debug!("riotId: {}", riotId);

    let champion_name_try = payload["allPlayers"]
        .as_array()
        .and_then(|players| {
            players
                .iter()
                .find(|player| player["riotId"].as_str() == Some(riotId))
                .and_then(|player| player["championName"].as_str())
        });
    
    let champion_name = match champion_name_try {
        Some(a) => a,
        None => {
            log::error!("championName not found in payload: {}", payload);
            return;
        }
    };

    log::debug!("championName: {}", champion_name);
    let open_obsidian_file_uri = format!("obsidian://new?vault=LeagueOfLegends&file={}", champion_name);
    match open::that(&open_obsidian_file_uri){
        Ok(_) => log::info!("opened obsidian {}", &open_obsidian_file_uri),
        Err(e) => log::error!("failed to open obsidian: {}", e),
    }

     
}

