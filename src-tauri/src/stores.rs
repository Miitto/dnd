use std::sync::Mutex;

pub struct WeaponStore {
    pub melee: Mutex<Vec<crate::items::weapon::MeleeWeapon>>,
}

impl WeaponStore {
    pub fn new() -> Self {
        WeaponStore {
            melee: Mutex::new(Vec::new()),
        }
    }
}

pub struct Store {
    pub weapons: WeaponStore,
}

impl Store {
    pub fn new() -> Self {
        let weapons = WeaponStore::new();
        Store { weapons }
    }
}
