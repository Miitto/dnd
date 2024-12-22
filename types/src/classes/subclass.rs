use std::collections::HashMap;

use super::deserialize_hashmap_array_to_feature;
use super::ClassFeature;
use super::ClassProficiencies;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Subclass {
    pub name: String,
    pub description: String,
    pub proficiencies: ClassProficiencies,
    pub spells: HashMap<u8, Vec<String>>,
    #[serde(deserialize_with = "deserialize_hashmap_array_to_feature")]
    pub features: HashMap<u8, Vec<ClassFeature>>,
}

impl PartialEq<str> for Subclass {
    fn eq(&self, other: &str) -> bool {
        self.name == other
    }
}

impl PartialEq<Subclass> for Subclass {
    fn eq(&self, other: &Subclass) -> bool {
        self.name == other.name
    }
}
