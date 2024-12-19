use serde::{Deserialize, Serialize};

use crate::attributes::Attribute;

use super::damage::Damage;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Property {
    pub name: String,
    pub description: Option<String>,
    pub effects: Vec<PropertyEffect>,
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
pub enum EffectType {
    #[serde(rename = "damage")]
    Damage(Damage),
    #[serde(rename = "attribute")]
    Attribute(AttributeReplacement),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PropertyEffect {
    pub optional: bool,
    pub when: Option<String>,
    pub effect_type: EffectType,
}
