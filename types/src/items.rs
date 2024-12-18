use damage::Damage;
use serde::{Deserialize, Serialize};

use crate::attributes::Attribute;
pub mod damage;
pub mod weapon;

pub trait Item {
    fn name(&self) -> &str;
    fn rarity(&self) -> &Rarity;
    fn properties(&self) -> &[Property];
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rarity(String);

impl Rarity {
    #[allow(dead_code)]
    pub fn new<S>(name: S) -> Self
    where
        S: Into<String>,
    {
        Self(name.into())
    }
}

impl std::fmt::Display for Rarity {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Property {
    pub name: String,
    pub description: Option<String>,
    pub effects: Vec<PropertyEffect>,
}

impl Property {
    #[allow(dead_code)]
    pub fn new<S>(name: S, description: Option<S>, effects: Vec<PropertyEffect>) -> Self
    where
        S: Into<String>,
    {
        Self {
            name: name.into(),
            description: description.map(|s| s.into()),
            effects,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttributeReplacement {
    pub replace: Attribute,
    pub with: Attribute,
}

impl std::fmt::Display for AttributeReplacement {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} -> {}", self.replace, self.with)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum EffectType {
    #[serde(rename = "damage")]
    Damage(Damage),
    #[serde(rename = "attribute")]
    Attribute(AttributeReplacement),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PropertyEffect {
    #[serde(default)]
    pub optional: bool,
    pub effect_type: EffectType,
    pub when: Option<String>,
}
