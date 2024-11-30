use tauri::{Listener,App,Event};

pub fn liveclient_data_event(event:Event){
    if let Ok(payload) = serde_json::from_str::<serde_json::Value>(&event.payload()) {
        log::info!("liveclient_data {}", payload.to_string());
    }
}

