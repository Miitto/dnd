use macros::SingleSerialize;
use serde::{Deserialize, Serialize};

pub mod weapon;

pub trait Item {
    fn name(&self) -> &str;
    fn rarity(&self) -> &Rarity;
    fn properties(&self) -> &[Property];
}

#[derive(Debug, Clone, SingleSerialize)]
pub struct Rarity {
    pub name: String,
}

impl Rarity {
    #[allow(dead_code)]
    pub fn new<S>(name: S) -> Self
    where
        S: Into<String>,
    {
        Self { name: name.into() }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Property {
    pub name: String,
    pub description: Option<String>,
}

impl Property {
    #[allow(dead_code)]
    pub fn new<S>(name: S, description: Option<S>) -> Self
    where
        S: Into<String>,
    {
        Self {
            name: name.into(),
            description: description.map(|s| s.into()),
        }
    }
}
