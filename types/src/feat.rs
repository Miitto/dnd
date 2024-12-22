use std::collections::HashMap;

use crate::common::Attribute;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Feat {
    pub name: String,
    pub description: String,
    pub attributes: HashMap<Attribute, u8>,
    pub benefits: Vec<String>,
}

impl PartialEq<str> for Feat {
    fn eq(&self, other: &str) -> bool {
        self.name == other
    }
}

impl PartialEq<Feat> for Feat {
    fn eq(&self, other: &Feat) -> bool {
        self.name == other.name
    }
}
