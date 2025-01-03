use crate::common::{Attribute, Condition, Damage, Dice};

use super::{Components, OnSave};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
pub struct Spell {
    pub name: String,
    pub level: u8,
    pub school: String,
    pub components: Components,
    pub cast_time: String,
    pub range: u32,
    pub duration: String,
    pub description: String,
    pub at_higher_levels: Option<String>,
    pub save: Option<Attribute>,
    pub damage: Option<Damage>,
    pub heal: Option<Dice>,
    pub condition: Option<Condition>,
    #[serde(default)]
    pub concentration: bool,
    #[serde(default)]
    pub ritual: bool,
    pub on_save: Option<OnSave>,
}

impl PartialEq<str> for Spell {
    fn eq(&self, other: &str) -> bool {
        self.name == other
    }
}

impl PartialEq for Spell {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
