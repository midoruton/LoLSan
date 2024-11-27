use std::path::PathBuf;
use tauri::{AppHandle, Manager, Runtime};

use crate::types::error::LoLSanError;

pub fn get_config_file_path<R>(app: &AppHandle<R>) -> Result<PathBuf, LoLSanError>
where
    R: Runtime,
{
    let config_path = app.path().app_config_dir()?.join("store.bin");
    Ok(config_path)
}
