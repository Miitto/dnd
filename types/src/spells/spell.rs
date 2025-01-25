use std::sync::{Arc, Mutex};

use crate::{
    extensions::{ForceLock, IsFalse},
    mechanics::{Attribute, Condition, Damage, Dice},
    meta::{Description, Link, Source},
    stat_block::StatBlock,
    traits::Linkable,
    Named,
};

use super::{Components, OnSave};
use anyhow::Result;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
pub struct Spell {
    pub name: String,
    pub source: Source,
    pub level: u8,
    pub school: String,
    pub components: Components,
    pub cast_time: String,
    pub range: String,
    pub duration: String,
    pub description: Description,
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

impl Linkable for Spell {
    fn link_external_stat_blocks(&mut self, stat_blocks: &[Arc<Mutex<StatBlock>>]) -> &mut Self {
        for stat_block in stat_blocks {
            for appended in &mut self.appended_stat_blocks {
                if let Link::NotFound(name) = appended {
                    if stat_block.force_lock().name == *name {
                        *appended = Link::Found(stat_block.clone());
                    }
                }
            }
        }

        self
    }
}

impl Spell {
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
