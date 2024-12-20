use std::collections::HashMap;

use crate::{asi::ASI, size::DescribedSize};

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Race {
    pub name: String,
    pub description: String,
    pub default_asi: Vec<ASI>,
    pub age: String,
    pub alignment: String,
    pub size: DescribedSize,
    pub speed: u32,
    pub languages: String,
    pub tables: Vec<Vec<Vec<String>>>,
    #[serde(default)]
    pub category: String,
    #[serde(flatten)]
    pub unique: HashMap<String, String>,
}

impl PartialEq<Race> for Race {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl PartialEq<str> for Race {
    fn eq(&self, other: &str) -> bool {
        self.name == other
    }
}
