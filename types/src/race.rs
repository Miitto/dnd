use crate::{
    mechanics::{Attributes, DescribedSize},
    meta::{Description, NamedDescription, Source, Table},
    traits::Linkable,
    Category, CategoryMut, Named,
};

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct Race {
    pub name: String,
    pub source: Source,
    pub description: Description,
    pub default_asi: Attributes,
    pub age: String,
    pub alignment: String,
    pub size: DescribedSize,
    pub speed: u32,
    pub languages: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tables: Vec<Table>,
    #[serde(default, skip_serializing)]
    pub category: String,
    #[serde(flatten, default, skip_serializing_if = "Vec::is_empty")]
    pub features: Vec<NamedDescription>,
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

impl Linkable for Race {
    fn link(&mut self) -> &mut Self {
        for feature in &mut self.features {
            feature.link();
        }
        self
    }
}
