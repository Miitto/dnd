use std::collections::HashMap;

use crate::fs::deserializers::deserialize_hashmap;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TableEntry {
    #[serde(default)]
    pub interpolate: bool,
    #[serde(flatten, deserialize_with = "deserialize_hashmap")]
    pub entries: HashMap<u8, String>,
}

impl TableEntry {
    pub fn get(&self, level: u8) -> String {
        let level = if self.interpolate {
            *self
                .entries
                .keys()
                .reduce(|acc, e| if *e > *acc && *e <= level { e } else { acc })
                .unwrap_or(&0u8)
        } else {
            level
        };

        self.entries
            .get(&level)
            .unwrap_or(&String::new())
            .to_string()
    }
}
