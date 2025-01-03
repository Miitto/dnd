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

impl TryFrom<&str> for Attribute {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "strength" | "str" => Ok(Attribute::Strength),
            "dexterity" | "dex" => Ok(Attribute::Dexterity),
            "constitution" | "con" => Ok(Attribute::Constitution),
            "intelligence" | "int" => Ok(Attribute::Intelligence),
            "wisdom" | "wis" => Ok(Attribute::Wisdom),
            "charisma" | "cha" => Ok(Attribute::Charisma),
            _ => Err(format!("Invalid attribute: {}", value)),
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

impl Attribute {
    pub fn as_short(&self) -> &str {
        match self {
            Attribute::Strength => "str",
            Attribute::Dexterity => "dex",
            Attribute::Constitution => "con",
            Attribute::Intelligence => "int",
            Attribute::Wisdom => "wis",
            Attribute::Charisma => "cha",
        }
    }
}
