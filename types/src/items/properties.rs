use serde::{Deserialize, Serialize};

use crate::common::{Attribute, Damage};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Property {
    pub name: String,
    pub description: Option<String>,
    pub effects: Vec<PropertyEffect>,
}

impl std::fmt::Display for Property {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl PartialEq for Property {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
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

impl PartialEq for AttributeReplacement {
    fn eq(&self, other: &Self) -> bool {
        self.replace == other.replace && self.with == other.with
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum EffectType {
    #[serde(rename = "damage")]
    Damage(Damage),
    #[serde(rename = "attribute")]
    Attribute(AttributeReplacement),
}

impl std::fmt::Display for EffectType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            EffectType::Damage(damage) => write!(f, "{}", damage),
            EffectType::Attribute(replacement) => write!(f, "{}", replacement),
        }
    }
}

impl PartialEq for EffectType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (EffectType::Damage(damage), EffectType::Damage(other_damage)) => {
                damage == other_damage
            }
            (EffectType::Attribute(replacement), EffectType::Attribute(other_replacement)) => {
                replacement == other_replacement
            }
            _ => false,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PropertyEffect {
    pub optional: bool,
    pub when: Option<String>,
    pub effect_type: EffectType,
}
