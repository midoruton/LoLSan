use tauri::{command, State};
use futures::lock::Mutex;
use std::sync::Arc;
#[derive(Default)]
pub struct AppState {
   pub getting_liveclient_data_loop_mutex : Arc<Mutex<()>>, // 排他制御用の Mutex
}