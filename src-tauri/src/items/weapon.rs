use macros::SingleSerialize;
use serde::{Deserialize, Serialize};

use crate::dice::Dice;

use super::{Property, Rarity};

#[derive(Debug, Clone, SingleSerialize)]
pub struct DamageType {
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeleeWeapon {
    pub name: String,
    pub damage: Dice,
    pub damage_type: DamageType,
    pub rarity: Rarity,
    pub properties: Vec<Property>,
    pub weight: f32,
    pub subtype: Vec<String>,
}
