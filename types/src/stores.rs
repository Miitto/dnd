use std::{
    fs::DirEntry,
    path::Path,
    sync::{Arc, Mutex},
};

use anyhow::Result;

use crate::fs;

use super::items::{weapon::Weapon, Item};

#[derive(Debug, Clone)]
pub struct WeaponStore {
    pub weapons: Arc<Mutex<Vec<Arc<crate::items::weapon::Weapon>>>>,
}

impl Default for WeaponStore {
    fn default() -> Self {
        WeaponStore {
            weapons: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

impl WeaponStore {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&self, wpn: Weapon) {
        let mut weapons = match self.weapons.lock() {
            Ok(w) => w,
            Err(poisoned) => poisoned.into_inner(),
        };
        weapons.push(Arc::new(wpn));
    }

    pub fn melee(&self) -> Vec<Arc<Weapon>> {
        let weapons = match self.weapons.lock() {
            Ok(w) => w,
            Err(poisoned) => poisoned.into_inner(),
        };
        weapons.iter().filter(|w| w.is_melee()).cloned().collect()
    }

    pub fn find_weapon(&self, name: &str) -> Option<Arc<Weapon>> {
        let weapons = match self.weapons.lock() {
            Ok(w) => w,
            Err(poisoned) => poisoned.into_inner(),
        };
        weapons.iter().find(|w| w.name() == name).map(Arc::clone)
    }

    pub fn find_melee(&self, name: &str) -> Option<Arc<Weapon>> {
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

#[derive(Debug, Clone)]
pub struct Store {
    pub weapons: WeaponStore,
}

impl Default for Store {
    fn default() -> Self {
        let weapons = WeaponStore::new();
        Store { weapons }
    }
}

impl Store {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_path<P>(path: P) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        let store = Store::new();

        {
            let weapons_store = &mut store
                .weapons
                .weapons
                .lock()
                .expect("Failed to lock weapons on store create");

            let weapons = fs::weapons::weapon::get_weapons(path)?
                .into_iter()
                .map(Arc::new);

            weapons_store.extend(weapons);
        }

        Ok(store)
    }
}

#[allow(dead_code)]
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
