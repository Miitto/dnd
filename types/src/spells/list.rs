use std::sync::Arc;

use super::spell::Spell;

#[derive(Debug, Clone, PartialEq)]
pub struct SpellList {
    pub name: String,
    pub spells: Vec<Arc<Spell>>,
}

impl PartialEq<str> for SpellList {
    fn eq(&self, other: &str) -> bool {
        self.name == other
    }
}
