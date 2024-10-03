mod dice;
mod items;
mod paths;

// TODO: Re-enable Checking for dead_code

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet])
        .setup(|app| {
            let resource_path = paths::resource_dir(app.handle());

            let rapier_path = resource_path.join("rapier.json");

            // Read rapier from the file
            let rapier = std::fs::read_to_string(rapier_path).expect("Unable to read rapier.json");

            // Parse the rapier
            let rapier: items::weapon::MeleeWeapon =
                serde_json::from_str(&rapier).expect("Unable to parse rapier.json");

            // Print the rapier
            println!("{:#?}", rapier);

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
