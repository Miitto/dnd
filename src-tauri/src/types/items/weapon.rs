use macros::SingleSerialize;
use serde::{Deserialize, Serialize};

use crate::dice::Dice;

use super::Item;

mod melee;
pub use melee::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WeaponType {
    Melee(MeleeWeapon),
    // Ranged(RangedWeapon),
}

pub trait Weapon: Item {
    fn damage(&self) -> &Dice;
    fn damage_type(&self) -> &DamageType;
    fn weight(&self) -> f32;
    fn subtype(&self) -> &[String];
}
#[derive(Debug, Clone, SingleSerialize)]
pub struct DamageType {
    pub name: String,
}

impl DamageType {
    #[allow(dead_code)]
    pub fn new<S>(name: S) -> Self
    where
        S: Into<String>,
    {
        Self { name: name.into() }
    }
}
