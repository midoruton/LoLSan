use anyhow::{Context, Result};
use std::sync::Arc;
use tauri::{App, Manager, Runtime};
use tauri_plugin_store::{Store, StoreExt};

pub fn get_app_store<R>(app: &App<R>) -> Result<Arc<Store<R>>>
where
    R: Runtime,
{
    let config_path = app
        .handle()
        .path()
        .app_config_dir()
        .with_context(|| format!("Failed to load app config dir."))?
        .join("store.bin");
    Ok(app.store(config_path.as_path())?)
}
