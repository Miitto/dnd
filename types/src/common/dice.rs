use std::fmt::{self, Display, Formatter};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default, Copy)]
pub struct Dice {
    pub sides: i32,
    pub count: i32,
    #[serde(skip_serializing_if = "none_or_zero")]
    pub modifier: Option<i32>,
}

fn none_or_zero(val: &Option<i32>) -> bool {
    val.unwrap_or(0) == 0
}

impl From<Dice> for String {
    fn from(val: Dice) -> Self {
        let modifier = match val.modifier {
            Some(m) => format!("{:+}", m),
            None => "".to_string(),
        };
        format!("{}d{}{}", val.count, val.sides, modifier)
    }
}

impl Display for Dice {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let modifier = match self.modifier {
            Some(m) => format!("{:+}", m),
            None => "".to_string(),
        };
        write!(f, "{}d{}{}", self.count, self.sides, modifier)
    }
}

impl PartialEq for Dice {
    fn eq(&self, other: &Self) -> bool {
        self.sides == other.sides && self.count == other.count && self.modifier == other.modifier
    }
}

impl Dice {
    pub fn new(sides: i32, count: i32, modifier: Option<i32>) -> Self {
        Self {
            sides,
            count,
            modifier,
        }
    }

    pub fn is_effective_zero(&self) -> bool {
        self.count == 0 && self.modifier.unwrap_or(0) == 0
    }
}
