#[derive(Debug, thiserror::Error)]
pub enum TauriError {
    #[error("ReqwestError: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("IoError: {0}")]
    Io(#[from] std::io::Error),
    #[error("PluginStoreError: {0}")]
    PluginStore(#[from] tauri_plugin_store::Error),
    #[error("PoisonError")]
    PoisonError(String),
}

impl<T> From<std::sync::PoisonError<T>> for TauriError {
    fn from(_error: std::sync::PoisonError<T>) -> Self {
        TauriError::PoisonError(_error.to_string())
    }
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
