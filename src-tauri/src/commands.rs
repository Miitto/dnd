use std::sync::Arc;

use tauri::State;

use crate::{items::weapon::WeaponType, stores::Store};

#[tauri::command]
pub fn get_melee_weapon(name: String, weapons: State<Store>) -> Result<Arc<WeaponType>, String> {
    let weapon = weapons.weapons.find_melee(name.as_str());

    match weapon {
        Some(weapon) => Ok(weapon),
        None => Err("Weapon not found".to_string()),
    }
}
