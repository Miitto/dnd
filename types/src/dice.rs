use std::fmt::{self, Display, Formatter};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dice {
    pub sides: i32,
    pub count: i32,
    pub modifier: Option<i32>,
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
