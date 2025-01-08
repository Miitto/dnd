use std::fmt::Display;

use crate::mechanics::Skill;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default, PartialEq)]
pub struct ClassSkills {
    pub options: Vec<Skill>,
    pub choose: u8,
}

impl Display for ClassSkills {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let s = self.options.join(", ");

        write!(f, "Choose {} from {}", self.choose, s)
    }
}
