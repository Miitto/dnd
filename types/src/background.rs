use crate::skill::Skill;

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
        if self.skill_proficiencies.is_empty() {
            return String::from("None");
        }

        let mut skill_str = String::new();

        for skill in self.skill_proficiencies.iter() {
            skill_str.push_str(skill.to_string().as_str());
            skill_str.push_str(", ");
        }

        skill_str.pop();
        skill_str.pop();

        skill_str
    }

    pub fn tool_string(&self) -> String {
        if self.skill_proficiencies.is_empty() {
            return String::from("None");
        }

        let mut tool_str = String::new();

        for tool in self.tool_proficiencies.iter() {
            tool_str.push_str(tool.to_string().as_str());
            tool_str.push_str(", ");
        }

        tool_str.pop();
        tool_str.pop();

        tool_str
    }

    pub fn equip_string(&self) -> String {
        if self.skill_proficiencies.is_empty() {
            return String::from("None");
        }

        let mut equip_str = String::new();

        for equip in self.equipment.iter() {
            equip_str.push_str(equip.to_string().as_str());
            equip_str.push_str(", ");
        }

        equip_str.pop();
        equip_str.pop();

        equip_str
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
