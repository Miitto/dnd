use std::{collections::HashMap, path::Path, sync::Arc};

use crate::{
    fs::{parse_dir, SPELL_LIST_PATH},
    spells::{Spell, SpellEntry, SpellList},
};

use anyhow::Result;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct SerializedSpellList {
    pub name: String,
    pub spells: Vec<String>,
}

pub fn get_spell_lists<P: AsRef<Path>>(
    resource_path: P,
    spells: &HashMap<String, Arc<Spell>>,
) -> Result<Vec<SpellList>> {
    let list_path = resource_path.as_ref().join(SPELL_LIST_PATH);

    let serialized_lists: Vec<SerializedSpellList> = parse_dir(list_path)?;

    let lists = serialized_lists
        .into_iter()
        .map(|list| {
            let spells = list
                .spells
                .into_iter()
                .map(|spell| {
                    if let Some(spell) = spells.get(spell.as_str()).map(Arc::clone) {
                        SpellEntry::Spell(spell)
                    } else {
                        SpellEntry::Name(spell)
                    }
                })
                .collect();
            SpellList {
                name: list.name,
                spells,
            }
        })
        .collect();

    Ok(lists)
}
