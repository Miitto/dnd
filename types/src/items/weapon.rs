use serde::{Deserialize, Serialize};

use super::{damage::Damage, Item, Property, Rarity};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Weapon {
    pub name: String,
    pub damage: Damage,
    pub rarity: Rarity,
    pub properties: Vec<Property>,
    pub weight: f32,
    pub subtype: Vec<String>,
    pub range: u32,
}

impl Weapon {
    #[allow(dead_code)]
    pub fn new<S, VP>(
        name: S,
        damage: Damage,
        rarity: Rarity,
        properties: VP,
        weight: f32,
        subtype: Vec<S>,
        range: u32,
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
            range,
        }
    }

    pub fn is_melee(&self) -> bool {
        self.range == 0
    }

    pub fn is_ranged(&self) -> bool {
        self.range > 0
    }
}

impl Item for Weapon {
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

impl PartialEq<String> for Weapon {
    fn eq(&self, other: &String) -> bool {
        self.name.to_lowercase() == other.to_lowercase()
    }
}

impl PartialEq<Weapon> for Weapon {
    fn eq(&self, other: &Weapon) -> bool {
        self.name.to_lowercase() == other.name.to_lowercase()
    }
}

impl PartialEq<str> for Weapon {
    fn eq(&self, other: &str) -> bool {
        self.name.to_lowercase() == other.to_lowercase()
    }
}
