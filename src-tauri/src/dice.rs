use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dice {
    pub sides: i32,
    pub count: i32,
    pub modifier: i32,
}
