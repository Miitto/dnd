use std::collections::HashMap;

use crate::meta::Description;
use crate::meta::NamedDescription;
use crate::meta::Source;

use super::ClassProficiencies;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Subclass {
    pub name: String,
    pub source: Source,
    pub description: Description,
    pub proficiencies: ClassProficiencies,
    pub spells: HashMap<u8, Vec<String>>,
    pub features: HashMap<u8, Vec<NamedDescription>>,
    #[serde(skip)]
    pub class: String,
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
