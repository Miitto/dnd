use std::collections::HashMap;

use crate::{
    mechanics::{DescribedSize, ASI},
    meta::{Description, Table},
    Category, CategoryMut, Named,
};

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Race {
    pub name: String,
    pub description: Description,
    pub default_asi: Vec<ASI>,
    pub age: String,
    pub alignment: String,
    pub size: DescribedSize,
    pub speed: u32,
    pub languages: String,
    pub tables: Vec<Table>,
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

impl Named for Race {
    fn name(&self) -> String {
        self.name.clone()
    }
}

impl CategoryMut for Race {
    fn category_mut(&mut self) -> &mut String {
        &mut self.category
    }
}

impl Category for Race {
    fn category(&self) -> String {
        self.category.clone()
    }
}
