use tauri::Manager;

mod dice;
mod items;
mod paths;

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
            let mut resource_path = app.path().resource_dir().unwrap();
            resource_path.push("resources");
            dbg!(&resource_path);

            let d8 = dice::Dice { sides: 8, count: 1 };
            let piercing = items::weapon::DamageType {
                name: "Piercing".to_string(),
            };
            let standard = items::Rarity {
                name: "Standard".to_string(),
            };
            let finesse = items::Property {
                name: "Finesse".to_string(),
            };

            let rapier = items::weapon::MeleeWeapon {
                name: "Rapier".to_string(),
                damage: d8,
                damage_type: piercing,
                rarity: standard,
                properties: vec![finesse],
                weight: 2.0,
                subtype: vec!["Martial".to_string(), "Finesse".to_string()],
            };

            dbg!(&rapier);

            let rapier_json = serde_json::to_string(&rapier).unwrap();

            let mut rapier_path = resource_path.clone();
            rapier_path.push("rapier.json");

            std::fs::write(rapier_path, rapier_json).unwrap();

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
