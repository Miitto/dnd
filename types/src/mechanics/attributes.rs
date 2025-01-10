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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Attributes {
    pub strength: u8,
    pub dexterity: u8,
    pub constitution: u8,
    pub intelligence: u8,
    pub wisdom: u8,
    pub charisma: u8,
}

impl Attributes {
    pub fn str_mod(&self) -> i8 {
        (self.strength as i8 - 10) / 2
    }

    pub fn str_str(&self) -> String {
        format!("{} ({:+})", self.strength, self.str_mod())
    }

    pub fn dex_mod(&self) -> i8 {
        (self.dexterity as i8 - 10) / 2
    }

    pub fn dex_str(&self) -> String {
        format!("{} ({:+})", self.dexterity, self.dex_mod())
    }

    pub fn con_mod(&self) -> i8 {
        (self.constitution as i8 - 10) / 2
    }

    pub fn con_str(&self) -> String {
        format!("{} ({:+})", self.constitution, self.con_mod())
    }

    pub fn int_mod(&self) -> i8 {
        (self.intelligence as i8 - 10) / 2
    }

    pub fn int_str(&self) -> String {
        format!("{} ({:+})", self.intelligence, self.int_mod())
    }

    pub fn wis_mod(&self) -> i8 {
        (self.wisdom as i8 - 10) / 2
    }

    pub fn wis_str(&self) -> String {
        format!("{} ({:+})", self.wisdom, self.wis_mod())
    }

    pub fn cha_mod(&self) -> i8 {
        (self.charisma as i8 - 10) / 2
    }

    pub fn cha_str(&self) -> String {
        format!("{} ({:+})", self.charisma, self.cha_mod())
    }
}

impl From<[u8; 6]> for Attributes {
    fn from(val: [u8; 6]) -> Self {
        Attributes {
            strength: val[0],
            dexterity: val[1],
            constitution: val[2],
            intelligence: val[3],
            wisdom: val[4],
            charisma: val[5],
        }
    }
}

impl From<Attributes> for [u8; 6] {
    fn from(val: Attributes) -> Self {
        [
            val.strength,
            val.dexterity,
            val.constitution,
            val.intelligence,
            val.wisdom,
            val.charisma,
        ]
    }
}
