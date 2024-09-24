// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::Serialize; // Add this import
#[derive(Debug, thiserror::Error)]
enum Error {
    #[error(transparent)]
    Io(#[from] reqwest::Error),
}

// we must manually implement serde::Serialize
impl serde::Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

async fn fetch_data() -> Result<String, reqwest::Error> {
    let res = reqwest::get("https://localhost").await?;
    res.text().await
}


#[tauri::command]
async fn greet() -> Result<String, Error>
where
    Result<String, Error>: Serialize, // Add this trait bound
{
    match fetch_data().await {
        Ok(data) => Ok(data),
        Err(e) => Err(Error::Io(e)),
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
