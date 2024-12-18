use serde::{Deserialize, Serialize};

use super::{damage::Damage, Item, Property, Rarity};

mod melee;
mod ranged;
pub use melee::*;
pub use ranged::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum WeaponType {
    Melee(MeleeWeapon),
    Ranged(RangedWeapon),
}

pub trait Weapon: Item {
    fn damage(&self) -> &Damage;
    fn weight(&self) -> f32;
    fn subtype(&self) -> &[String];
}

impl WeaponType {
    pub fn is_melee(&self) -> bool {
        matches!(self, WeaponType::Melee(_))
    }

    pub fn is_ranged(&self) -> bool {
        !self.is_melee()
    }

    pub fn as_melee(&self) -> Option<&MeleeWeapon> {
        match self {
            WeaponType::Melee(m) => Some(m),
            _ => None,
        }
    }

    pub fn as_ranged(&self) -> Option<&RangedWeapon> {
        match self {
            WeaponType::Ranged(r) => Some(r),
            _ => None,
        }
    }
}

impl Item for WeaponType {
    fn name(&self) -> &str {
        match self {
            WeaponType::Melee(m) => m.name(),
            WeaponType::Ranged(r) => r.name(),
        }
    }

    fn rarity(&self) -> &Rarity {
        match self {
            WeaponType::Melee(m) => m.rarity(),
            WeaponType::Ranged(r) => r.rarity(),
        }
    }

    fn properties(&self) -> &[Property] {
        match self {
            WeaponType::Melee(m) => m.properties(),
            WeaponType::Ranged(r) => r.properties(),
        }
    }
}

impl Weapon for WeaponType {
    fn damage(&self) -> &Damage {
        match self {
            WeaponType::Melee(m) => m.damage(),
            WeaponType::Ranged(r) => r.damage(),
        }
    }

    fn weight(&self) -> f32 {
        match self {
            WeaponType::Melee(m) => m.weight(),
            WeaponType::Ranged(r) => r.weight(),
        }
    }

    fn subtype(&self) -> &[String] {
        match self {
            WeaponType::Melee(m) => m.subtype(),
            WeaponType::Ranged(r) => r.subtype(),
        }
    }
}
