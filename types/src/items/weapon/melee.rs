use super::*;
use crate::items::damage::Damage;
use crate::items::{Item, Property, Rarity};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeleeWeapon {
    pub name: String,
    pub damage: Damage,
    pub rarity: Rarity,
    pub properties: Vec<Property>,
    pub weight: f32,
    pub subtype: Vec<String>,
}

impl MeleeWeapon {
    #[allow(dead_code)]
    pub fn new<S, VP>(
        name: S,
        damage: Damage,
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
    fn damage(&self) -> &Damage {
        &self.damage
    }

    fn weight(&self) -> f32 {
        self.weight
    }

    fn subtype(&self) -> &[String] {
        &self.subtype
    }
}

impl PartialEq<String> for MeleeWeapon {
    fn eq(&self, other: &String) -> bool {
        self.name.to_lowercase() == other.to_lowercase()
    }
}

impl PartialEq<MeleeWeapon> for MeleeWeapon {
    fn eq(&self, other: &MeleeWeapon) -> bool {
        self.name.to_lowercase() == other.name.to_lowercase()
    }
}
