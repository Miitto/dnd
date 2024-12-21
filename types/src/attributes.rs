use std::borrow::Borrow;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Attribute {
    #[serde(rename = "strength")]
    Strength,
    #[serde(rename = "dexterity")]
    Dexterity,
    #[serde(rename = "constitution")]
    Constitution,
    #[serde(rename = "intelligence")]
    Intelligence,
    #[serde(rename = "wisdom")]
    Wisdom,
    #[serde(rename = "charisma")]
    Charisma,
}

impl From<Attribute> for String {
    fn from(val: Attribute) -> Self {
        match val {
            Attribute::Strength => "Strength".to_string(),
            Attribute::Dexterity => "Dexterity".to_string(),
            Attribute::Constitution => "Constitution".to_string(),
            Attribute::Intelligence => "Intelligence".to_string(),
            Attribute::Wisdom => "Wisdom".to_string(),
            Attribute::Charisma => "Charisma".to_string(),
        }
    }
}

impl std::fmt::Display for Attribute {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", String::from(*self))
    }
}

impl Borrow<str> for Attribute {
    fn borrow(&self) -> &str {
        match self {
            Attribute::Strength => "Strength",
            Attribute::Dexterity => "Dexterity",
            Attribute::Constitution => "Constitution",
            Attribute::Intelligence => "Intelligence",
            Attribute::Wisdom => "Wisdom",
            Attribute::Charisma => "Charisma",
        }
    }
}
