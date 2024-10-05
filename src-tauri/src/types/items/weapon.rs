use macros::SingleSerialize;
use serde::{Deserialize, Serialize};

use crate::dice::Dice;

use super::{Item, Property, Rarity};

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

impl Item for WeaponType {
    fn name(&self) -> &str {
        match self {
            WeaponType::Melee(m) => m.name(),
            // WeaponType::Ranged(r) => r.name(),
        }
    }

    fn rarity(&self) -> &Rarity {
        match self {
            WeaponType::Melee(m) => m.rarity(),
            // WeaponType::Ranged(r) => r.rarity(),
        }
    }

    fn properties(&self) -> &[Property] {
        match self {
            WeaponType::Melee(m) => m.properties(),
            // WeaponType::Ranged(r) => r.properties(),
        }
    }
}

impl Weapon for WeaponType {
    fn damage(&self) -> &Dice {
        match self {
            WeaponType::Melee(m) => m.damage(),
            // WeaponType::Ranged(r) => r.damage(),
        }
    }

    fn damage_type(&self) -> &DamageType {
        match self {
            WeaponType::Melee(m) => m.damage_type(),
            // WeaponType::Ranged(r) => r.damage_type(),
        }
    }

    fn weight(&self) -> f32 {
        match self {
            WeaponType::Melee(m) => m.weight(),
            // WeaponType::Ranged(r) => r.weight(),
        }
    }

    fn subtype(&self) -> &[String] {
        match self {
            WeaponType::Melee(m) => m.subtype(),
            // WeaponType::Ranged(r) => r.subtype(),
        }
    }
}
