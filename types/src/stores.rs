use std::{
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
};

use hash_store::HashStore;

use crate::{
    background::Background,
    classes::Class,
    fs::{
        classes::class::get_classes, constants::*, get_backgrounds, get_feats, get_races,
        get_spell_lists, get_stat_blocks, spells::get_spells, weapons::weapon::get_weapons,
    },
    items::weapon::Weapon,
    race::Race,
    spells::{Spell, SpellList},
    stat_block::StatBlock,
    traits::Linkable,
    ForceLock,
};

mod hash_store;

pub use hash_store::Saveable;

#[derive(Debug, Clone)]
pub struct Store {
    path: PathBuf,
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
    pub fn get_path(&self) -> &Path {
        self.path.as_path()
    }

    pub fn new<P>(path: P) -> Self
    where
        P: AsRef<Path>,
    {
        let path = path.as_ref().to_path_buf();

        let weapon_path = path.join(ITEM_WEAPON_PATH);
        let background_path = path.join(BACKGROUND_PATH);
        let races_path = path.join(RACE_PATH);
        let classes_path = path.join(CLASS_PATH);
        let feats_path = path.join(FEAT_PATH);
        let spells_path = path.join(SPELL_PATH);
        let spell_lists_path = path.join(SPELL_LIST_PATH);
        let stat_blocks_path = path.join(STAT_BLOCK_PATH);

        let store = Self {
            path,
            weapons: Arc::new(HashStore::new(weapon_path)),
            backgrounds: Arc::new(HashStore::new(background_path)),
            races: Arc::new(HashStore::new(races_path)),
            classes: Arc::new(HashStore::new(classes_path)),
            feats: Arc::new(HashStore::new(feats_path)),
            spells: Arc::new(HashStore::new(spells_path)),
            spell_lists: Arc::new(HashStore::new(spell_lists_path)),
            stat_blocks: Arc::new(HashStore::new(stat_blocks_path)),
        };

        macro_rules! impl_store {
            ($type:ty, $get_fn:ident, $sub:ident) => {{
                let inner = &mut store.$sub.store.lock().expect("Failed to lock $type");

                match $get_fn(store.get_path()) {
                    Ok(items) => inner.extend(
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
            let stats: Vec<Arc<Mutex<StatBlock>>> = store
                .stat_blocks
                .store
                .force_lock()
                .values()
                .cloned()
                .collect();
            let spells = store.spells.store.force_lock();

            for spell in spells.values() {
                spell
                    .lock()
                    .expect("Failed to lock spell")
                    .link_external_stat_blocks(&stats);
            }
        }

        store
    }
}
