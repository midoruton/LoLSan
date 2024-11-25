
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
    #[error("JSONSchemaError")]
    JSONSchema(String),
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

impl From<jsonschema::ValidationError<'static>> for LoLSanError {
    fn from(error: jsonschema::ValidationError) -> Self {
        LoLSanError::JSONSchema(error.to_string())
    }
}

//エラーに謎のイテレータが返ってくるので、とりあえずstringに変換しておく
impl From<jsonschema::ErrorIterator<'_>> for LoLSanError {
    fn from(iter_errors: jsonschema::ErrorIterator) -> Self {
        let s = iter_errors.map(|e| e.to_string()).fold(String::new(), |acc, e| acc + &e + "\n");
        LoLSanError::JSONSchema(s)
    }
}

