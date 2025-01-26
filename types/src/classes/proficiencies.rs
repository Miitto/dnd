use crate::classes::ClassSkills;
use crate::mechanics::Attribute;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct ClassProficiencies {
    #[serde(default)]
    pub armor: Vec<String>,
    #[serde(default)]
    pub weapons: Vec<String>,
    #[serde(default)]
    pub tools: Vec<String>,
    #[serde(default)]
    pub saving_throws: Vec<Attribute>,
    #[serde(default)]
    pub skills: ClassSkills,
}

impl ClassProficiencies {
    pub fn is_empty(&self) -> bool {
        self.armor.is_empty()
            && self.weapons.is_empty()
            && self.tools.is_empty()
            && self.saving_throws.is_empty()
            && self.skills.options.is_empty()
    }
}
