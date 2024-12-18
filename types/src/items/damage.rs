use serde::{Deserialize, Serialize};

use crate::dice::Dice;

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
