use futures::lock::Mutex;
use std::sync::Arc;
#[derive(Default)]
pub struct AppState {
   pub liveclinet_data_access_mutex : Arc<Mutex<()>>, // 排他制御用の Mutex
}