use std::{fs::DirEntry, path::Path, sync::Mutex};

use super::items::weapon::WeaponType;

#[derive(Debug)]
pub struct WeaponStore {
    weapons: Mutex<Vec<crate::items::weapon::WeaponType>>,
}

impl WeaponStore {
    pub fn new() -> Self {
        WeaponStore {
            weapons: Mutex::new(Vec::new()),
        }
    }

    pub fn push(&self, wpn: WeaponType) {
        let mut weapons = match self.weapons.lock() {
            Ok(w) => w,
            Err(poisoned) => poisoned.into_inner(),
        };
        weapons.push(wpn);
    }
}

#[derive(Debug)]
pub struct Store {
    pub weapons: WeaponStore,
}

impl Store {
    pub fn new() -> Self {
        let weapons = WeaponStore::new();
        Store { weapons }
    }

    pub fn from_path<P>(path: P) -> Result<Self, std::io::Error>
    where
        P: AsRef<Path>,
    {
        let items_path = path.as_ref().join("items");

        dbg!(&items_path);

        let items_dir: Vec<std::io::Result<DirEntry>> = std::fs::read_dir(items_path)?.collect();

        let weapons_iter = items_dir.iter().filter_map(|entry| {
            entry
                .as_ref()
                .map(|entry| {
                    Some(entry).filter(|entry| {
                        entry.file_type().map(|ft| ft.is_file()).unwrap_or(false)
                            && entry
                                .path()
                                .extension()
                                .map(|ext| ext == "json")
                                .unwrap_or(false)
                            && entry
                                .path()
                                .parent()
                                .unwrap()
                                .file_name()
                                .map(|name| name == "weapons")
                                .unwrap_or(false)
                    })
                })
                .ok()
                .flatten()
        });

        let store = Store::new();

        weapons_iter.for_each(|weapon| {
            let path = weapon.path();
            let json = std::fs::read_to_string(&path)
                .map(|contents| serde_json::from_str::<WeaponType>(&contents));

            match json {
                Ok(Ok(weapon)) => store.weapons.push(weapon),
                Ok(Err(err)) => eprintln!("Error parsing weapon: {}", err),
                Err(err) => eprintln!("Error reading weapon file: {}", err),
            }
        });

        Ok(store)
    }
}
