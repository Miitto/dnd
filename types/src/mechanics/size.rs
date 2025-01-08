use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub enum Size {
    Tiny,
    Small,
    Medium,
    Large,
    Huge,
    Gargantuan,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DescribedSize {
    pub size: Size,
    pub description: String,
}

impl Display for Size {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
