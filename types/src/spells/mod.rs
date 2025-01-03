mod list;
mod spell;

use std::fmt::{Display, Formatter};

pub use list::{SpellEntry, SpellList};
pub use spell::Spell;

use crate::IsFalse;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
pub struct Components {
    #[serde(default, skip_serializing_if = "bool::is_false")]
    pub verbal: bool,
    #[serde(default, skip_serializing_if = "bool::is_false")]
    pub somatic: bool,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub material: Vec<String>,
}

impl Display for Components {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut components = vec![];

        if self.verbal {
            components.push("V".into());
        }

        if self.somatic {
            components.push("S".into());
        }

        if !self.material.is_empty() {
            components.push("M".into());
            let material = self.material.join(", ");
            let bracket = format!("({})", material);
            components.push(bracket);
        }

        write!(f, "{}", components.join(", "))
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default, Copy)]
pub enum OnSave {
    Half,
    #[default]
    None,
}

impl Display for OnSave {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            OnSave::Half => write!(f, "Half"),
            OnSave::None => write!(f, "None"),
        }
    }
}

impl From<&str> for OnSave {
    fn from(val: &str) -> Self {
        match val {
            "Half" => OnSave::Half,
            "None" => OnSave::None,
            _ => OnSave::None,
        }
    }
}

impl From<String> for OnSave {
    fn from(val: String) -> Self {
        val.as_str().into()
    }
}
