// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use serde::Serialize;
use std::sync::Mutex;
use tauri::{Manager, State, Wry};
use tauri_plugin_store::{Store, StoreBuilder};
struct AppState {
    store: Store<Wry>,
}

#[derive(Debug, thiserror::Error)]
enum TauriError {
    #[error("ReqwestError: {0}")]
    Reqwest(#[source] reqwest::Error),
    #[error("IoError: {0}")]
    Io(#[source] std::io::Error),
    #[error("PluginStoreError: {0}")]
    PluginStore(#[source] tauri_plugin_store::Error),
    #[error("PoisonError {0}")]
    PoisonError(String),
}

// we must manually implement serde::Serialize
impl serde::Serialize for TauriError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

async fn fetch_data() -> Result<String, reqwest::Error> {
    let res = reqwest::get("https://localhost").await?;
    res.text().await
}

#[tauri::command]
async fn set_obsidian_vault_path(
    vault_path: String,
    state: State<'_, Mutex<AppState>>,
) -> Result<(), TauriError>
where
    Result<(), TauriError>: Serialize, // Add this trait bound
{
    let mut lock = state.lock();
    let store = match lock {
        Ok(ref mut s) => &mut s.store,
        Err(e) => return Err(TauriError::PoisonError(e.to_string())),
    };
    match store.insert("vault_path".into(), vault_path.into()) {
        Ok(_) => {}
        Err(e) => return Err(TauriError::PluginStore(e)),
    }
    match store.save() {
        Ok(_) => {}
        Err(e) => return Err(TauriError::PluginStore(e)),
    }
    Ok(())
}

#[tauri::command]
async fn greet() -> Result<String, TauriError>
where
    Result<String, TauriError>: Serialize, // Add this trait bound
{
    match fetch_data().await {
        Ok(data) => Ok(data),
        Err(e) => Err(TauriError::Reqwest(e)),
    }
}

#[tauri::command]
async fn create_lol_champions_obsidian_file(champion_name: String) -> Result<(), TauriError>
where
    Result<(), TauriError>: Serialize,
{
    match open::that(format!(
        "obsidian://open?vault=LeagueOfLegends&name={}",
        champion_name
    )) {
        Ok(_) => Ok(()),
        Err(e) => Err(TauriError::Io(e)),
    }
    //match open::that(format!("obsidian://new?vault=LeagueOfLegends&name={}",champion_name)){
    //        Ok(_)=>Ok(()),
    //        Err(e)  =>Err(TauriError::Io(e)),
    //}
}
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greet,
            create_lol_champions_obsidian_file,
            set_obsidian_vault_path
        ])
        .setup(|app| {
            let config_path = app
                .handle()
                .path_resolver()
                .app_config_dir()
                .unwrap()
                .join("store.bin");
            app.manage(Mutex::new(AppState {
                store: StoreBuilder::new(app.handle(), config_path).build(),
            }));
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
