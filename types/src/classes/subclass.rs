use std::collections::HashMap;

use crate::meta::Description;

use super::ClassFeature;
use super::ClassProficiencies;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Subclass {
    pub name: String,
    pub description: Description,
    pub proficiencies: ClassProficiencies,
    pub spells: HashMap<u8, Vec<String>>,
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
