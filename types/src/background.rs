use crate::{
    mechanics::Skill,
    meta::{Description, Source, Table},
    traits::Linkable,
};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
pub struct Background {
    pub name: String,
    pub source: Source,
    pub description: Description,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub skill_proficiencies: Vec<Skill>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tool_proficiencies: Vec<String>,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub languages: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub features: Vec<BackgroundFeature>,
    pub equipment: Vec<Description>,
    #[serde(default, skip_serializing)]
    pub category: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub embedded_tables: Vec<Table>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BackgroundFeature {
    pub name: String,
    pub description: Description,
}

impl Background {
    pub fn skill_string(&self) -> String {
        self.skill_proficiencies.join(", ")
    }

    pub fn tool_string(&self) -> String {
        self.tool_proficiencies.join(", ")
    }
}

impl Linkable for Background {
    fn link_tables(&mut self) -> &mut Self {
        let tables = &self.embedded_tables;
        for feature in &mut self.features {
            feature.description.clone_external_tables(tables);
        }
        self.description.clone_external_tables(tables);

        self
    }
}

impl PartialEq<Background> for Background {
    fn eq(&self, other: &Background) -> bool {
        self.name == other.name
    }
}

impl PartialEq<str> for Background {
    fn eq(&self, other: &str) -> bool {
        self.name == other
    }
}

impl crate::Named for Background {
    fn name(&self) -> String {
        self.name.clone()
    }
}

impl crate::Category for Background {
    fn category(&self) -> String {
        self.category.clone()
    }
}

impl crate::CategoryMut for Background {
    fn category_mut(&mut self) -> &mut String {
        &mut self.category
    }
}
