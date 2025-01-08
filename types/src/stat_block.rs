use std::collections::HashMap;

use crate::{
    mechanics::{Alignment, Attributes, CreatureType, Size, Skill},
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
    pub saving_throws: Option<Attributes>,
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
    pub other_attributes: Option<HashMap<String, String>>,
    pub actions: Option<HashMap<String, String>>,
    pub reactions: Option<HashMap<String, String>>,
    pub legendary_actions: Option<HashMap<String, String>>,
    pub special_abilities: Option<HashMap<String, String>>,
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
