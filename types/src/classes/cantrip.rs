use std::collections::HashMap;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ClassCantrip {
    pub list: Vec<String>,
    pub progression: HashMap<u8, u8>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CantripProgression {
    pub level: u8,
    pub count: u8,
}

impl ClassCantrip {
    pub fn count(&self, level: u8) -> u8 {
        let mut count = 0;
        let mut highest = 0;

        for (lvl, cnt) in self.progression.iter() {
            if *lvl <= level && *lvl > highest {
                count = *cnt;
                highest = *lvl;
            }
        }

        count
    }
}
