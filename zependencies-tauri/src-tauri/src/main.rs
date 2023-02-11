#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use directories::ProjectDirs;
use error_stack::{Report, ResultExt};
use serde_with::SerializeDisplay;
use thiserror::Error;
use zependencies_common::db;

#[derive(Error, SerializeDisplay, Debug)]
#[error("while initializing application")]
struct InitError;

#[tauri::command]
#[tracing::instrument]
async fn initialize() -> Result<(), Report<InitError>> {
    let Some(project_dirs) = ProjectDirs::from(
        "com.github.zependencies",
        "Zependencies Developers",
        "zependencies",
    ) else {
        return Err(Report::new(InitError).attach_printable("could not retrieve project folders"));
    };
    let _db = db::init(&project_dirs).await.change_context(InitError)?;
    Ok(())
}

fn main() {
    tracing_subscriber::fmt::init();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![initialize])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
