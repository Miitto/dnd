use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crate::{
    mechanics::{Attribute, Condition, Damage, Dice},
    stat_block::StatBlock,
    IsFalse, Link, Named,
};

use super::{Components, OnSave};
use anyhow::Result;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
pub struct Spell {
    pub name: String,
    pub level: u8,
    pub school: String,
    pub components: Components,
    pub cast_time: String,
    pub range: String,
    pub duration: String,
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub at_higher_levels: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub save: Option<Attribute>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub damage: Vec<Damage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heal: Option<Dice>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<Condition>,
    #[serde(default, skip_serializing_if = "bool::is_false")]
    pub concentration: bool,
    #[serde(default, skip_serializing_if = "bool::is_false")]
    pub ritual: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_save: Option<OnSave>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub appended_stat_blocks: Vec<Link<Arc<Mutex<StatBlock>>>>,
}

impl PartialEq<str> for Spell {
    fn eq(&self, other: &str) -> bool {
        self.name == other
    }
}

impl PartialEq for Spell {
    fn eq(&self, other: &Spell) -> bool {
        self.name == other.name
    }
}

impl Spell {
    pub fn link_stat_blocks(&mut self, stat_blocks: &HashMap<String, Arc<Mutex<StatBlock>>>) {
        self.appended_stat_blocks.iter_mut().for_each(|stat_block| {
            if let Link::NotFound(name) = stat_block {
                if let Some(found) = stat_blocks.get(name) {
                    *stat_block = Link::Found(Arc::clone(found));
                }
            }
        });
    }

    pub fn serialize(&self) -> Result<String> {
        serde_json::to_string(self).map_err(Into::into)
    }

    pub fn serialize_pretty(&self) -> Result<String> {
        serde_json::to_string_pretty(self).map_err(Into::into)
    }
}

impl Named for Spell {
    fn name(&self) -> String {
        self.name.to_owned()
    }
}
