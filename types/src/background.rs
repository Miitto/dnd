use crate::common::Skill;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Background {
    pub name: String,
    pub description: String,
    pub skill_proficiencies: Vec<Skill>,
    pub tool_proficiencies: Vec<String>,
    pub languages: String,
    pub equipment: Vec<String>,
    pub features: Vec<BackgroundFeature>,
    #[serde(default)]
    pub category: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BackgroundFeature {
    pub name: String,
    pub description: String,
}

impl Background {
    pub fn skill_string(&self) -> String {
        self.skill_proficiencies.join(", ")
    }

    pub fn tool_string(&self) -> String {
        self.tool_proficiencies.join(", ")
    }

    pub fn equip_string(&self) -> String {
        self.equipment.join(", ")
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

impl crate::CategoryMut for Background {
    fn category_mut(&mut self) -> &mut String {
        &mut self.category
    }
}
