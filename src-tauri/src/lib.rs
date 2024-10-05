use tauri::Manager;

mod commands;
mod paths;
mod types;
use stores::Store;
use types::*;

use commands::*;

// TODO: Re-enable Checking for dead_code

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![get_melee_weapon])
        .setup(|app| {
            let store =
                Store::from_path(paths::resource_dir(app.handle())).expect("Failed to load store");

            dbg!(&store);

            app.manage(store);

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
