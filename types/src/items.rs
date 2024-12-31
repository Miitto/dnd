use properties::Property;
use serde::{Deserialize, Serialize};

pub mod properties;
pub mod weapon;

pub trait Item {
    fn name(&self) -> &str;
    fn rarity(&self) -> &Rarity;
    fn properties(&self) -> &[Property];
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rarity(String);

impl Rarity {
    #[allow(dead_code)]
    pub fn new<S>(name: S) -> Self
    where
        S: Into<String>,
    {
        Self(name.into())
    }
}

impl std::fmt::Display for Rarity {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
