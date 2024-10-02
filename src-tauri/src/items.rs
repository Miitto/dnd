use macros::SingleSerialize;
use serde::{Deserialize, Serialize};
pub mod weapon;

#[derive(Debug, Clone, SingleSerialize)]
pub struct Rarity {
    pub name: String,
}

#[derive(Debug, Clone, SingleSerialize)]
pub struct Property {
    pub name: String,
}
