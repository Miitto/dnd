use std::fmt::{self, Display, Formatter};

use crate::meta::Description;

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
    pub description: Description,
}

impl Display for Size {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
