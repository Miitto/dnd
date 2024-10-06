use std::{
    fs::DirEntry,
    path::Path,
    sync::{Arc, Mutex},
};

use super::items::{weapon::WeaponType, Item};

#[derive(Debug)]
pub struct WeaponStore {
    weapons: Mutex<Vec<Arc<crate::items::weapon::WeaponType>>>,
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
        weapons.push(Arc::new(wpn));
    }

    pub fn melee(&self) -> Vec<Arc<WeaponType>> {
        let weapons = match self.weapons.lock() {
            Ok(w) => w,
            Err(poisoned) => poisoned.into_inner(),
        };
        weapons.iter().filter(|w| w.is_melee()).cloned().collect()
    }

    pub fn find_melee(&self, name: &str) -> Option<Arc<WeaponType>> {
        let weapons = match self.weapons.lock() {
            Ok(w) => w,
            Err(poisoned) => poisoned.into_inner(),
        };
        weapons
            .iter()
            .filter(|w| w.is_melee())
            .find(|w| w.name() == name)
            .map(Arc::clone)
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

        let mut items_dir: Vec<DirEntry> = Vec::new();
        std::fs::read_dir(items_path)?.for_each(|entry| {
            let _ = entry
                .map(|entry| {
                    if entry.path().is_dir() {
                        visit_dirs(&entry.path(), &mut items_dir)
                    } else {
                        items_dir.push(entry);
                        Ok(())
                    }
                })
                .unwrap_or(Ok(()));
        });

        dbg!(&items_dir);

        let weapons_iter = items_dir.iter().filter_map(|entry| {
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
        });

        let store = Store::new();

        weapons_iter.for_each(|weapon| {
            dbg!(&weapon);

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

fn visit_dirs(dir: &Path, vec: &mut Vec<DirEntry>) -> std::io::Result<()> {
    if dir.is_dir() {
        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, vec)?;
            } else {
                vec.push(entry);
            }
        }
    }
    Ok(())
}
