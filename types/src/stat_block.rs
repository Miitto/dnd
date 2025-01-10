use std::collections::HashMap;

use crate::{
    mechanics::{Alignment, Attribute, Attributes, CreatureType, Size, Skill},
    Named,
};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct StatBlock {
    pub name: String,
    pub size: Size,
    pub creature_type: CreatureType,
    pub alignment: Option<Alignment>,
    pub armor_class: String,
    pub hit_points: String,
    pub speed: u32,
    pub attributes: Attributes,
    #[serde(default)]
    pub saving_throws: Vec<Attribute>,
    #[serde(default)]
    pub damage_resistances: Vec<String>,
    #[serde(default)]
    pub damage_immunities: Vec<String>,
    #[serde(default)]
    pub damage_vulnerabilities: Vec<String>,
    #[serde(default)]
    pub condition_immunities: Vec<String>,
    pub darkvision: Option<u32>,
    pub passive_perception: Option<u32>,
    #[serde(default)]
    pub senses: Vec<String>,
    #[serde(default)]
    pub languages: Vec<String>,
    pub challenge_rating: Option<u8>,
    #[serde(default)]
    pub proficiencies: Vec<Skill>,
    pub proficiency_bonus: Option<String>,
    #[serde(default)]
    pub traits: HashMap<String, String>,
    #[serde(default)]
    pub actions: HashMap<String, String>,
    #[serde(default)]
    pub reactions: HashMap<String, String>,
    #[serde(default)]
    pub legendary_actions: HashMap<String, String>,
    #[serde(default)]
    pub special_abilities: HashMap<String, String>,
}

impl PartialEq<str> for StatBlock {
    fn eq(&self, other: &str) -> bool {
        self.name == other
    }
}

impl Named for StatBlock {
    fn name(&self) -> String {
        self.name.to_owned()
    }
}
