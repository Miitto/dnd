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
        classes::class::get_classes, get_backgrounds, get_feats, get_races, get_spell_lists,
        get_stat_blocks, spells::get_spells, weapons::weapon::get_weapons,
    },
    items::weapon::Weapon,
    race::Race,
    spells::{Spell, SpellList},
    ForceLock,
};

#[derive(Debug, Clone)]
pub struct HashStore<T>
where
    T: PartialEq<str> + Clone,
{
    pub store: Arc<Mutex<HashMap<String, Arc<Mutex<T>>>>>,
}

impl<T> Default for HashStore<T>
where
    T: PartialEq<str> + Clone,
{
    fn default() -> Self {
        Self {
            store: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

impl<T> HashStore<T>
where
    T: PartialEq<str> + Clone,
{
    pub fn all(&self) -> HashMap<String, T> {
        let store = self.store.force_lock();
        store
            .iter()
            .map(|(k, v)| (k.clone(), (v.lock().expect("Failed to lock item")).clone()))
            .collect()
    }

    pub fn all_vec(&self) -> Vec<T> {
        let store = self.store.force_lock();
        store
            .values()
            .map(|v| (v.lock().expect("Failed to lock item")).clone())
            .collect()
    }

    pub fn set(&self, name: String, item: T) {
        let mut store = self.store.force_lock();
        let existing = store.get(&name);
        if let Some(existing) = existing {
            *existing.lock().expect("Failed to lock existing item") = item;
        } else {
            store.insert(name, Arc::new(Mutex::new(item)));
        }
    }

    pub fn get(&self, name: &str) -> Option<Arc<Mutex<T>>> {
        let store = self.store.force_lock();
        store.get(name).map(Arc::clone)
    }

    pub fn get_clone(&self, name: &str) -> Option<T> {
        self.get(name)
            .map(|item| item.lock().expect("Failed to lock item").clone())
    }

    pub fn get_arced(&self, name: &str) -> Option<Arc<T>> {
        self.get_clone(name).map(Arc::new)
    }
}

impl HashStore<Weapon> {
    pub fn melee(&self) -> Vec<Weapon> {
        self.store
            .force_lock()
            .values()
            .filter_map(|item| {
                let item = item.lock().expect("Failed to lock item");
                if item.is_melee() {
                    Some(item.clone())
                } else {
                    None
                }
            })
            .collect()
    }
}

#[derive(Debug, Clone, Default)]
pub struct Store {
    path: Option<PathBuf>,
    pub weapons: Arc<HashStore<Weapon>>,
    pub races: Arc<HashStore<Race>>,
    pub backgrounds: Arc<HashStore<Background>>,
    pub classes: Arc<HashStore<Class>>,
    pub feats: Arc<HashStore<crate::feat::Feat>>,
    pub spells: Arc<HashStore<Spell>>,
    pub spell_lists: Arc<HashStore<SpellList>>,
    pub stat_blocks: Arc<HashStore<crate::stat_block::StatBlock>>,
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

        macro_rules! impl_store {
            ($type:ty, $get_fn:ident, $sub:ident) => {{
                let store = &mut store.$sub.store.lock().expect("Failed to lock $type");

                match $get_fn(&path) {
                    Ok(items) => store.extend(
                        items
                            .into_iter()
                            .map(|item| (item.name.clone(), Arc::new(Mutex::new(item)))),
                    ),
                    Err(e) => eprintln!("Failed to get $type: {:?}", e),
                }
            }};
        }

        impl_store!(Weapon, get_weapons, weapons);
        impl_store!(Race, get_races, races);
        impl_store!(Background, get_backgrounds, backgrounds);
        impl_store!(Class, get_classes, classes);
        impl_store!(crate::feat::Feat, get_feats, feats);
        impl_store!(Spell, get_spells, spells);
        impl_store!(SpellList, get_spell_lists, spell_lists);
        impl_store!(StatBlock, get_stat_blocks, stat_blocks);

        {
            let stats = store.stat_blocks.store.force_lock();
            dbg!(stats.keys().collect::<Vec<_>>());
        }

        {
            let spells = store.spells.store.force_lock();
            let lock = store.spell_lists.store.force_lock();

            for list in lock.values() {
                list.lock()
                    .expect("Failed to lock spell list")
                    .link(&spells);
            }
        }

        {
            let stats = store.stat_blocks.store.force_lock();
            let spells = store.spells.store.force_lock();

            for spell in spells.values() {
                spell
                    .lock()
                    .expect("Failed to lock spell")
                    .link_stat_blocks(&stats);
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
