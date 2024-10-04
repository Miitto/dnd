use std::path::PathBuf;

use tauri::{AppHandle, Manager};

pub fn resource_dir(app: &AppHandle) -> PathBuf {
    let mut resource_path = app.path().resource_dir().unwrap();
    resource_path.push("resources");
    resource_path
}
