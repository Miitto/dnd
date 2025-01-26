use std::{collections::HashMap, hash::Hash};

use crate::meta::{Description, NamedDescription, Source};
use crate::traits::Linkable;
use crate::{extensions::StartsWithVowel, mechanics::Attribute};

use super::{
    cantrip::ClassCantrip,
    casting::{CastLevel, CastType},
    subclass::Subclass,
    table_entry::TableEntry,
    ClassProficiencies,
};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ClassSubclasses {
    #[serde(skip)]
    pub options: HashMap<String, Subclass>,
    #[serde(default = "default_subclass_unlock", rename = "subclass_unlock")]
    pub unlocked: u8,
}

fn default_subclass_unlock() -> u8 {
    3
}

impl ClassSubclasses {
    pub fn get(&self, subclass: &str) -> Option<&Subclass> {
        self.options.get(subclass)
    }

    pub fn is_empty(&self) -> bool {
        self.options.is_empty()
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Class {
    pub name: String,
    pub source: Source,
    pub description: Description,
    pub requirements: HashMap<Attribute, u8>,
    pub hit_die: u8,
    pub proficiencies: ClassProficiencies,
    pub equipment: Vec<String>,
    pub features: HashMap<u8, Vec<NamedDescription>>,
    pub spellcasting: Option<Attribute>,
    #[serde(default)]
    pub ritual_casting: bool,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub spell_lists: Vec<String>,
    pub cast_type: Option<CastType>,
    #[serde(default)]
    pub cast_level: CastLevel,
    pub cantrips: Option<ClassCantrip>,
    #[serde(flatten)]
    pub subclasses: ClassSubclasses,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub table_entries: HashMap<String, TableEntry>,
}

impl Class {
    pub fn cantrips_known(&self, level: u8) -> u8 {
        self.cantrips.as_ref().map_or(0, |c| c.count(level))
    }

    pub fn spell_slots(&self, level: u8, slot_level: u8) -> u8 {
        self.cast_level.slots_at_level(level, slot_level)
    }

    pub fn requirements_string(&self) -> String {
        let mut requirements = String::new();

        let len = self.requirements.len();

        for (idx, (attr, val)) in self.requirements.iter().enumerate() {
            if idx > 0 {
                if attr.to_string().starts_with_vowel() {
                    requirements.push('n');
                }
                requirements.push(' ');
            }

            requirements.push_str(&format!("{} score of {}", attr, val));

            if len > 1 && idx < len - 2 {
                requirements.push_str(", ");
            }

            if len > 1 && idx == len - 2 {
                requirements.push_str(" and a");
            }
        }

        requirements
    }

    pub fn requirements_string_prepend(&self) -> String {
        let mut requirements = self.requirements_string();

        let starts_with_vowel = requirements.starts_with_vowel();

        requirements.insert(0, ' ');

        if starts_with_vowel {
            requirements.insert(0, 'n');
        }

        requirements
    }
}

impl Hash for Class {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl PartialEq<Class> for Class {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl PartialEq<str> for Class {
    fn eq(&self, other: &str) -> bool {
        self.name == other
    }
}

impl PartialEq<String> for Class {
    fn eq(&self, other: &String) -> bool {
        self.name == *other
    }
}

impl Linkable for Class {
    fn link_tables(&mut self) -> &mut Self {
        for (_lvl, features) in &mut self.features {
            for feature in features {
                feature.link();
            }
        }

        self
    }
}
