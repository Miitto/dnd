use std::{
    path::Path,
    sync::{Arc, Mutex},
};

use anyhow::Result;

use crate::{
    background::Background,
    fs::{background::get_backgrounds, race::get_races, weapons::weapon::get_weapons},
    items::weapon::Weapon,
    race::Race,
    ForceLock,
};

#[derive(Debug, Clone)]
pub struct InnerStore<T> {
    pub store: Arc<Mutex<Vec<Arc<T>>>>,
}

impl<T> Default for InnerStore<T> {
    fn default() -> Self {
        InnerStore {
            store: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

impl<T> InnerStore<T> {
    pub fn all(&self) -> Vec<Arc<T>> {
        let store = self.store.force_lock();
        store.clone()
    }
}

impl<T> InnerStore<T>
where
    T: PartialEq<str>,
{
    pub fn get(&self, name: &str) -> Option<Arc<T>> {
        let store = self.store.force_lock();
        store.iter().find(|&w| **w == *name).map(Arc::clone)
    }
}

impl InnerStore<Weapon> {
    pub fn melee(&self) -> Vec<Arc<Weapon>> {
        self.all()
            .iter()
            .filter(|w| w.is_melee())
            .cloned()
            .collect()
    }
}

#[derive(Debug, Clone, Default)]
pub struct Store {
    pub weapons: InnerStore<Weapon>,
    pub races: InnerStore<Race>,
    pub backgrounds: InnerStore<Background>,
}

macro_rules! impl_store {
    ($store:ident, $type:ty, $get_fn:ident, $path:ident, $sub:ident) => {{
        let store = &mut $store.$sub.store.lock().expect(concat!(
            "Failed to lock ",
            stringify!($store),
            " store on create"
        ));

        let items = $get_fn(&$path)?.into_iter().map(Arc::new);

        store.extend(items);
    }};
}

impl Store {
    pub fn from_path<P>(path: P) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        let store = Store::default();

        impl_store!(store, Weapon, get_weapons, path, weapons);
        impl_store!(store, Race, get_races, path, races);
        impl_store!(store, Background, get_backgrounds, path, backgrounds);

        Ok(store)
    }
}
