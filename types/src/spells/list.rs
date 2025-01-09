use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crate::meta::Link;

use super::spell::Spell;

use anyhow::Result;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
pub struct SpellList {
    pub name: String,
    pub spells: Vec<Link<Arc<Mutex<Spell>>>>,
}

impl PartialEq<str> for SpellList {
    fn eq(&self, other: &str) -> bool {
        self.name == other
    }
}

impl PartialEq for SpellList {
    fn eq(&self, other: &SpellList) -> bool {
        self.name == other.name && self.spells.len() == other.spells.len()
    }
}

impl SpellList {
    pub fn found(&mut self, spell: Arc<Mutex<Spell>>) {
        let name = spell.lock().unwrap().name.clone();

        self.spells.iter_mut().for_each(|link| {
            if let Link::NotFound(n) = link {
                if n == &name {
                    *link = Link::Found(Arc::clone(&spell));
                }
            }
        });
    }

    pub fn link(&mut self, spells: &HashMap<String, Arc<Mutex<Spell>>>) {
        self.spells.iter_mut().for_each(|spell| {
            if let Link::NotFound(name) = spell {
                if let Some(found) = spells.get(name) {
                    *spell = Link::Found(Arc::clone(found));
                }
            }
        });
    }

    pub fn partitioned(&self) -> (Vec<Arc<Mutex<Spell>>>, Vec<String>) {
        let (found, unfound): (Vec<_>, Vec<_>) =
            self.spells.iter().cloned().partition(|spell| match spell {
                Link::NotFound(_) => false,
                Link::Found(_) => true,
            });

        let found = found.into_iter().map(|spell| match spell {
            Link::Found(found) => found,
            _ => unreachable!(),
        });

        let unfound = unfound.into_iter().map(|spell| match spell {
            Link::NotFound(name) => name,
            _ => unreachable!(),
        });

        (found.collect(), unfound.collect())
    }

    pub fn partitioned_clone(&self) -> (Vec<Spell>, Vec<String>) {
        let (found, unfound) = self.partitioned();

        let found = found.into_iter().map(|spell| spell.lock().unwrap().clone());

        (found.collect(), unfound)
    }

    pub fn serialize(&self) -> Result<String> {
        serde_json::to_string(self).map_err(Into::into)
    }

    pub fn serialize_pretty(&self) -> Result<String> {
        serde_json::to_string_pretty(self).map_err(Into::into)
    }
}

impl crate::Named for SpellList {
    fn name(&self) -> String {
        self.name.clone()
    }
}
