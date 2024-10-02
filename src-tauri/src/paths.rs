use std::path::PathBuf;

use tauri::Manager;

pub fn resource_dir(app: &tauri::AppHandle) -> PathBuf {
    let mut resource_path = app.path().resource_dir().unwrap();
    resource_path.push("resources");
    resource_path
}
