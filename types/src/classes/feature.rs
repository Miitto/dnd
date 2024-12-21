use std::collections::HashMap;

use serde::{Deserialize, Deserializer};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ClassFeature {
    pub name: String,
    pub description: String,
}

pub fn deserialize_hashmap_array_to_feature<'de, D>(
    deserializer: D,
) -> Result<HashMap<u8, Vec<ClassFeature>>, D::Error>
where
    D: Deserializer<'de>,
{
    let v: HashMap<u8, Vec<HashMap<String, String>>> = HashMap::deserialize(deserializer)?;

    let mut map = HashMap::new();

    for (level, features) in v {
        let mut class_features = Vec::new();

        for feature in features {
            for (name, description) in feature {
                class_features.push(ClassFeature { name, description });
            }
        }

        map.insert(level, class_features);
    }

    Ok(map)
}
