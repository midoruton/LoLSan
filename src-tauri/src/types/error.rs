use crate::app::logic::fetch::ValidationError;

#[derive(Debug, thiserror::Error)]
pub enum LoLSanError {
    #[error("ReqwestError: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("IoError: {0}")]
    Io(#[from] std::io::Error),
    #[error("PluginStoreError: {0}")]
    PluginStore(#[from] tauri_plugin_store::Error),
    #[error("PoisonError")]
    Poison(String),
    #[error("TauriError: {0}")]
    Tauri(#[from] tauri::Error),
    #[error("ValidationError: {0}")]
    Validation(#[from] ValidationError),
    #[error("SerdeJsonError: {0}")]
    SerdeJson(#[from] serde_json::Error),
}

// we must manually implement serde::Serialize
impl serde::Serialize for LoLSanError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

impl<T> From<std::sync::PoisonError<T>> for LoLSanError {
    fn from(_error: std::sync::PoisonError<T>) -> Self {
        LoLSanError::Poison(_error.to_string())
    }
}
