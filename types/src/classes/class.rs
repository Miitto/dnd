use std::{collections::HashMap, hash::Hash};

use crate::meta::{Description, Source};
use crate::{extensions::StartsWithVowel, mechanics::Attribute};

use super::cantrip::ClassCantrip;
use super::skills::ClassSkills;
use super::subclass::Subclass;
use super::table_entry::TableEntry;
use super::ClassFeature;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub enum CastType {
    Prepared,
    Known,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Default)]
pub enum CastLevel {
    Full,
    Half,
    Third,
    #[default]
    None,
}

impl CastLevel {
    pub fn max_level(&self) -> u8 {
        match self {
            Self::Full => 9,
            Self::Half => 5,
            Self::Third => 3,
            Self::None => 0,
        }
    }

    pub fn slots_at_level(&self, level: u8, slot_level: u8) -> u8 {
        match self {
            Self::Full => match slot_level {
                1 => match level {
                    0 => 0,
                    1 => 2,
                    2 => 3,
                    3..=u8::MAX => 4,
                },
                2 => match level {
                    0..=2 => 0,
                    3 => 2,
                    4..=u8::MAX => 3,
                },
                3 => match level {
                    0..=4 => 0,
                    5 => 2,
                    6..=u8::MAX => 3,
                },
                4 => match level {
                    0..=6 => 0,
                    7 => 1,
                    8 => 2,
                    9..=u8::MAX => 3,
                },
                5 => match level {
                    0..=8 => 0,
                    9 => 1,
                    10..=17 => 2,
                    18..=u8::MAX => 3,
                },
                6 => match level {
                    0..=10 => 0,
                    11..=18 => 1,
                    19..=u8::MAX => 2,
                },
                7 => match level {
                    0..=12 => 0,
                    13..=19 => 1,
                    20..=u8::MAX => 2,
                },
                8 => match level {
                    0..=14 => 0,
                    15..=u8::MAX => 1,
                },
                9 => match level {
                    0..=16 => 0,
                    17..=u8::MAX => 1,
                },
                _ => 0,
            },
            Self::Half => match slot_level {
                1 => match level {
                    0 => 0,
                    1..=2 => 2,
                    3..=4 => 3,
                    5..=u8::MAX => 4,
                },
                2 => match level {
                    0..=4 => 0,
                    5..=6 => 2,
                    7..=u8::MAX => 3,
                },
                3 => match level {
                    0..=8 => 0,
                    9..=10 => 2,
                    11..=u8::MAX => 3,
                },
                4 => match level {
                    0..=12 => 0,
                    13..=14 => 1,
                    15..=16 => 2,
                    17..=u8::MAX => 3,
                },
                5 => match level {
                    0..=16 => 0,
                    17..=18 => 1,
                    19..=u8::MAX => 2,
                },
                _ => 0,
            },
            Self::Third => match slot_level {
                1 => match level {
                    1..=2 => 2,
                    3..=4 => 3,
                    5..=6 => 3,
                    7..=8 => 3,
                    9..=20 => 3,
                    _ => 0,
                },
                2 => match level {
                    1..=2 => 0,
                    3..=4 => 2,
                    5..=6 => 2,
                    7..=8 => 2,
                    9..=20 => 2,
                    _ => 0,
                },
                3 => match level {
                    1..=2 => 0,
                    3..=4 => 0,
                    5..=6 => 2,
                    7..=8 => 2,
                    9..=20 => 2,
                    _ => 0,
                },
                4 => match level {
                    1..=2 => 0,
                    3..=4 => 0,
                    5..=6 => 0,
                    7..=8 => 1,
                    9..=20 => 1,
                    _ => 0,
                },
                5 => match level {
                    1..=2 => 0,
                    3..=4 => 0,
                    5..=6 => 0,
                    7..=8 => 0,
                    9..=20 => 1,
                    _ => 0,
                },
                _ => 0,
            },
            Self::None => 0,
        }
    }
}
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
    pub features: HashMap<u8, Vec<ClassFeature>>,
    pub spellcasting: Option<Attribute>,
    #[serde(default)]
    pub ritual_casting: bool,
    pub spell_lists: Option<Vec<String>>,
    pub cast_type: Option<CastType>,
    #[serde(default)]
    pub cast_level: CastLevel,
    pub cantrips: Option<ClassCantrip>,
    #[serde(flatten)]
    pub subclasses: ClassSubclasses,
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

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct ClassProficiencies {
    #[serde(default)]
    pub armor: Vec<String>,
    #[serde(default)]
    pub weapons: Vec<String>,
    #[serde(default)]
    pub tools: Vec<String>,
    #[serde(default)]
    pub saving_throws: Vec<Attribute>,
    #[serde(default)]
    pub skills: ClassSkills,
}

impl ClassProficiencies {
    pub fn is_empty(&self) -> bool {
        self.armor.is_empty()
            && self.weapons.is_empty()
            && self.tools.is_empty()
            && self.saving_throws.is_empty()
            && self.skills.options.is_empty()
    }
}
