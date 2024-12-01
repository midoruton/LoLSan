use tauri::{utils::config, AppHandle, Event, Listener};
use tauri_plugin_store::StoreExt;
use crate::util::obsidian::{self, obsidian_file_exists};
pub fn liveclient_data_event(app:&AppHandle,event:Event){
    log::info!("liveclient_data_event");
    if let Ok(payload) = serde_json::from_str::<serde_json::Value>(&event.payload()) {
        log::info!("liveclient_data {}", payload.to_string());
        let config_path = match crate::util::path::get_config_file_path(&app){
            Ok(a) => a,
            Err(e) => {
                log::error!("failed to get config file path: {}", e);
                return;
            }
        };
        let vault_path_opt = match app.store(config_path){
            Ok(store) => {
                store.get("vault_path").and_then(|a| a.as_str().map(|s| s.to_string()))
            },
            Err(e) => {
                log::error!("failed to get store: {}", e);
                return;
            }
        };
        let vault_path = match vault_path_opt {
            Some(a) => a,
            None => {
                log::error!("vault_path not found in store");
                return;
            }
        };
        log::debug!("valut path: {}", vault_path);
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
        let mut obsidian_file_path = std::path::PathBuf::from(vault_path).join(champion_name);
        obsidian_file_path.set_extension("md");
        if obsidian_file_exists(&obsidian_file_path){
            log::info!("obsidian file exists: {}" , obsidian_file_path.display());
            let open_obsidian_file_uri = obsidian::OpenBuilder::new()
                .file(champion_name.to_string())
                .build()
                .to_uri();
            match open::that(&open_obsidian_file_uri){
                Ok(_) => log::info!("opened obsidian {}", &open_obsidian_file_uri),
                Err(e) => log::error!("failed to open obsidian: {}", e),
            }
        } else {
            log::info!("obsidian file does not exist: {}", obsidian_file_path.display());
            log::info!("creating obsidian file...");
            let create_obsidian_file_uri = obsidian::NewBuilder::new(champion_name)
                .build()
                .to_uri();
            match open::that(&create_obsidian_file_uri){
                Ok(_) => log::info!("created obsidian {}", &create_obsidian_file_uri),
                Err(e) => log::error!("failed to create obsidian: {}", e),
            }
        }

    } else {    
        log::error!("failed to parse liveclient_data event payload: {}", event.payload());
    }
}

