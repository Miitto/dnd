use crate::{
    mechanics::{Alignment, Attribute, Attributes, CreatureType, Size, Skill},
    meta::NamedDescription,
    traits::Linkable,
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub saving_throws: Vec<Attribute>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub damage_resistances: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub damage_immunities: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub damage_vulnerabilities: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub condition_immunities: Vec<String>,
    pub darkvision: Option<u32>,
    pub passive_perception: Option<u32>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub senses: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub languages: Vec<String>,
    pub challenge_rating: Option<u8>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub proficiencies: Vec<Skill>,
    pub proficiency_bonus: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub traits: Vec<NamedDescription>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actions: Vec<NamedDescription>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reactions: Vec<NamedDescription>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub legendary_actions: Vec<NamedDescription>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub special_abilities: Vec<NamedDescription>,
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

impl Linkable for StatBlock {
    fn link(&mut self) -> &mut Self {
        for t in &mut self.traits {
            t.link();
        }

        for action in &mut self.actions {
            action.link();
        }

        for reaction in &mut self.reactions {
            reaction.link();
        }

        for special_ability in &mut self.special_abilities {
            special_ability.link();
        }

        self
    }
}
