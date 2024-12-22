use serde::{Deserialize, Serialize};

use crate::common::Dice;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DamageType(String);

impl DamageType {
    #[allow(dead_code)]
    pub fn new<S>(name: S) -> Self
    where
        S: Into<String>,
    {
        Self(name.into())
    }
}

impl std::fmt::Display for DamageType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl PartialEq for DamageType {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Damage {
    pub dice: Dice,
    pub damage_type: DamageType,
}

impl From<Damage> for String {
    fn from(val: Damage) -> Self {
        format!("{} {}", val.dice, val.damage_type.0)
    }
}

impl std::fmt::Display for Damage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} {}", self.dice, self.damage_type.0)
    }
}

impl PartialEq for Damage {
    fn eq(&self, other: &Self) -> bool {
        self.dice == other.dice && self.damage_type == other.damage_type
    }
}
