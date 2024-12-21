use std::collections::HashMap;

use crate::deserialize_hashmap;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TableEntry {
    #[serde(default)]
    pub interpolate: bool,
    #[serde(flatten)]
    #[serde(deserialize_with = "deserialize_hashmap")]
    pub entries: HashMap<u8, String>,
}

impl TableEntry {
    pub fn get(&self, level: u8) -> String {
        if !self.interpolate {
            return self
                .entries
                .get(&level)
                .unwrap_or(&String::new())
                .to_string();
        }

        let mut highest = 0;
        for lvl in self.entries.keys() {
            if *lvl <= level && *lvl > highest {
                highest = *lvl;
            }
        }

        self.entries
            .get(&highest)
            .unwrap_or(&String::new())
            .to_string()
    }
}
