use serde::{Deserialize, Serialize};

pub mod weapon;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rarity {
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Property {
    pub name: String,
}
