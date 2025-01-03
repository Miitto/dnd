use std::sync::Arc;

use super::spell::Spell;

#[derive(Debug, Clone, PartialEq)]
pub enum SpellEntry {
    Name(String),
    Spell(Arc<Spell>),
}

impl SpellEntry {
    pub fn name(&self) -> &str {
        match self {
            SpellEntry::Name(name) => name,
            SpellEntry::Spell(spell) => &spell.name,
        }
    }

    pub fn found(&mut self, spell: Arc<Spell>) {
        if let SpellEntry::Name(name) = self {
            if *name == spell.name {
                *self = SpellEntry::Spell(spell);
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SpellList {
    pub name: String,
    pub spells: Vec<SpellEntry>,
}

impl PartialEq<str> for SpellList {
    fn eq(&self, other: &str) -> bool {
        self.name == other
    }
}

impl SpellList {
    pub fn found(&mut self, spell: Arc<Spell>) {
        for entry in self.spells.iter_mut() {
            entry.found(spell.clone());
        }
    }
}
