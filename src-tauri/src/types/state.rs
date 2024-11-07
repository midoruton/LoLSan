
use tauri::{Wry};
use tauri_plugin_store::{Store};
pub struct AppState {
    pub store: Store<Wry>,
}