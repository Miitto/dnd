use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
};

use anyhow::Result;

use crate::{
    background::Background,
    classes::Class,
    fs::{
        classes::class::get_classes,
        get_backgrounds, get_feats, get_races,
        spells::{get_spell_lists, get_spells},
        weapons::weapon::get_weapons,
    },
    items::weapon::Weapon,
    race::Race,
    spells::{Spell, SpellList},
    ForceLock,
};

#[derive(Debug, Clone)]
pub struct VecStore<T> {
    pub store: Arc<Mutex<Vec<Arc<T>>>>,
}

impl<T> Default for VecStore<T> {
    fn default() -> Self {
        VecStore {
            store: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

impl<T> VecStore<T> {
    pub fn all(&self) -> Vec<Arc<T>> {
        let store = self.store.force_lock();
        store.clone()
    }
}

impl<T> VecStore<T>
where
    T: PartialEq<str>,
{
    pub fn get(&self, name: &str) -> Option<Arc<T>> {
        let store = self.store.force_lock();
        store.iter().find(|&w| **w == *name).map(Arc::clone)
    }
}

impl VecStore<Weapon> {
    pub fn melee(&self) -> Vec<Arc<Weapon>> {
        self.all()
            .iter()
            .filter(|w| w.is_melee())
            .cloned()
            .collect()
    }
}

#[derive(Debug, Clone)]
pub struct HashStore<T> {
    pub store: Arc<Mutex<HashMap<String, Arc<T>>>>,
}

impl<T> Default for HashStore<T> {
    fn default() -> Self {
        Self {
            store: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

impl<T> HashStore<T> {
    pub fn all(&self) -> HashMap<String, Arc<T>> {
        let store = self.store.force_lock();
        store.clone()
    }

    pub fn set(&self, name: String, item: T) {
        let mut store = self.store.force_lock();
        store.insert(name, Arc::new(item));
    }
}

impl<T> HashStore<T>
where
    T: PartialEq<str>,
{
    pub fn get(&self, name: &str) -> Option<Arc<T>> {
        let store = self.store.force_lock();
        store.get(name).map(Arc::clone)
    }
}

#[derive(Debug, Clone, Default)]
pub struct Store {
    path: Option<PathBuf>,
    pub weapons: Arc<VecStore<Weapon>>,
    pub races: Arc<VecStore<Race>>,
    pub backgrounds: Arc<VecStore<Background>>,
    pub classes: Arc<VecStore<Class>>,
    pub feats: Arc<VecStore<crate::feat::Feat>>,
    pub spells: Arc<HashStore<Spell>>,
    pub spell_lists: Arc<VecStore<SpellList>>,
}

impl Store {
    pub fn get_path(&self) -> Option<&Path> {
        self.path.as_deref()
    }

    pub fn from_path<P>(path: P) -> Self
    where
        P: AsRef<Path>,
    {
        let mut store = Store::default();

        let path = path.as_ref().to_path_buf();

        store.path = Some(path.clone());

        macro_rules! impl_vec_store {
            ($type:ty, $get_fn:ident, $sub:ident) => {{
                impl_vec_store!($type, $get_fn, $sub, Arc::new);
            }};
            ($type:ty, $get_fn:ident, $sub:ident, $map:expr) => {{
                let store = &mut store.$sub.store.lock().expect("Failed to lock $type");

                match $get_fn(&path) {
                    Ok(items) => store.extend(items.into_iter().map($map)),
                    Err(e) => eprintln!("Failed to get $type: {:?}", e),
                }
            }};
        }

        macro_rules! impl_hash_store {
            ($type:ty, $get_fn:ident, $sub:ident) => {{
                impl_vec_store!($type, $get_fn, $sub, |item| (
                    item.name.clone(),
                    Arc::new(item)
                ))
            }};
        }

        impl_vec_store!(Weapon, get_weapons, weapons);
        impl_vec_store!(Race, get_races, races);
        impl_vec_store!(Background, get_backgrounds, backgrounds);
        impl_vec_store!(Class, get_classes, classes);
        impl_vec_store!(crate::feat::Feat, get_feats, feats);
        impl_hash_store!(Spell, get_spells, spells);

        {
            let mut lists = store
                .spell_lists
                .store
                .lock()
                .expect("Failed to lock spell lists");
            let spells = store.spells.store.lock().expect("Failed to lock spells");

            match get_spell_lists(&path, &spells) {
                Ok(items) => lists.extend(items.into_iter().map(Arc::new)),
                Err(e) => eprintln!("Failed to get spell lists: {:?}", e),
            }
        }

        store
    }

    pub fn rebuild(&mut self) -> Result<()> {
        if let Some(path) = self.path.as_ref() {
            *self = Store::from_path(path.clone());
        } else {
            return Err(anyhow::anyhow!("No path set for store"));
        }

        Ok(())
    }
}
