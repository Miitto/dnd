use std::fmt::{self, Display, Formatter};

use crate::meta::Description;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Default, Copy)]
pub enum Size {
    Tiny,
    Small,
    #[default]
    Medium,
    Large,
    Huge,
    Gargantuan,
}

impl From<&str> for Size {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "tiny" => Size::Tiny,
            "small" => Size::Small,
            "medium" => Size::Medium,
            "large" => Size::Large,
            "huge" => Size::Huge,
            "gargantuan" => Size::Gargantuan,
            _ => Size::Medium,
        }
    }
}

impl From<String> for Size {
    fn from(s: String) -> Self {
        Size::from(s.as_str())
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
pub struct DescribedSize {
    pub size: Size,
    pub description: Description,
}

impl Display for Size {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
