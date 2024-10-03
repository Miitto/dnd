use macros::SingleSerialize;
use serde::{Deserialize, Serialize};

use crate::dice::Dice;

use super::{Item, Property, Rarity};

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

impl MeleeWeapon {
    #[allow(dead_code)]
    pub fn new<S, VP>(
        name: S,
        damage: Dice,
        damage_type: DamageType,
        rarity: Rarity,
        properties: VP,
        weight: f32,
        subtype: Vec<S>,
    ) -> Self
    where
        S: Into<String>,
        VP: Into<Vec<Property>>,
    {
        Self {
            name: name.into(),
            damage,
            damage_type,
            rarity,
            properties: properties.into(),
            weight,
            subtype: subtype.into_iter().map(|s| s.into()).collect(),
        }
    }
}

impl Item for MeleeWeapon {
    fn name(&self) -> &str {
        &self.name
    }

    fn rarity(&self) -> &Rarity {
        &self.rarity
    }

    fn properties(&self) -> &[Property] {
        &self.properties
    }
}

impl Weapon for MeleeWeapon {
    fn damage(&self) -> &Dice {
        &self.damage
    }

    fn damage_type(&self) -> &DamageType {
        &self.damage_type
    }

    fn weight(&self) -> f32 {
        self.weight
    }

    fn subtype(&self) -> &[String] {
        &self.subtype
    }
}
