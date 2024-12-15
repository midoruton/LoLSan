use derive_more::derive::Display;
use futures::lock::Mutex;
use std::sync::Arc;
#[derive(Default)]
pub struct AppState {
   pub liveclinet_data_access_mutex : Arc<Mutex<()>>, // 排他制御用の Mutex
   pub league_client_state: Arc<Mutex<LeagueClientState>>,
}
#[derive(Default,Display)]
pub enum LeagueClientState{
    Connected,
    InGame(Arc<serde_json::Value>),
    #[default]Disconnected,
}