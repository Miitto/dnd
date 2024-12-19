use std::{fs::DirEntry, path::Path};

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

use crate::{fs::constants::ITEM_WEAPON_PROPERTIES_PATH, items::properties::EffectType};

use super::weapon::{SerializedPropertyEffectType, SerializedWeaponProperty};

/// The property referred to in the weapon file.
/// Hold the information about the property.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SerializedProperty {
    pub name: String,
    pub description: Option<String>,
    pub effects: Vec<SerializedPropertyEffect>,
}

impl PartialEq<SerializedWeaponProperty> for SerializedProperty {
    fn eq(&self, other: &SerializedWeaponProperty) -> bool {
        self.name.to_lowercase() == other.name.to_lowercase()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SerializedPropertyEffect {
    pub optional: bool,
    pub when: Option<String>,
    pub effect_type: SerializedPropertyEffectType,
}

pub fn get_weapon_properties<P: AsRef<Path>>(resource_path: P) -> Result<Vec<SerializedProperty>> {
    let weapons_properties_path = resource_path.as_ref().join(ITEM_WEAPON_PROPERTIES_PATH);
    let weapon_properties_dir: Vec<DirEntry> = std::fs::read_dir(&weapons_properties_path)
        .with_context(|| {
            format!(
                "Failed to read weapon properties dir at: {:?}",
                weapons_properties_path
            )
        })?
        .filter_map(|el| el.ok())
        .collect();

    let weapon_properties_iter = weapon_properties_dir.iter().filter_map(|entry: &DirEntry| {
        Some(entry).filter(|entry| {
            entry.file_type().map(|ft| ft.is_file()).unwrap_or(false)
                && entry
                    .path()
                    .extension()
                    .map(|ext| ext == "json")
                    .unwrap_or(false)
        })
    });

    let properties: Vec<SerializedProperty> = weapon_properties_iter
        .filter_map(|entry| {
            let path = entry.path();
            let read = std::fs::read_to_string(&path);

            let json = if let Ok(read) = read {
                read
            } else {
                eprintln!(
                    "Error reading weapon property file: {:?}\n{}",
                    path,
                    read.unwrap_err()
                );
                return None;
            };

            let parsed = serde_json::from_str::<SerializedProperty>(&json);

            if let Ok(prop) = parsed {
                Some(prop)
            } else {
                eprintln!(
                    "Error parsing weapon property file: {}\n{}",
                    json,
                    parsed.unwrap_err()
                );
                None
            }
        })
        .collect();

    Ok(properties)
}

pub fn combine_property_effect(
    weapon: &SerializedPropertyEffectType,
    template: &SerializedPropertyEffectType,
) -> Result<EffectType> {
    Ok(match (weapon, template) {
        (
            SerializedPropertyEffectType::Damage(weapon),
            SerializedPropertyEffectType::Damage(template),
        ) => {
            if let Some(weapon) = weapon {
                EffectType::Damage(weapon.clone())
            } else if let Some(template) = template {
                EffectType::Damage(template.clone())
            } else {
                return Err(anyhow::anyhow!("No damage effect found"));
            }
        }
        (
            SerializedPropertyEffectType::Attribute(weapon),
            SerializedPropertyEffectType::Attribute(template),
        ) => {
            if let Some(weapon) = weapon {
                EffectType::Attribute(weapon.clone())
            } else if let Some(template) = template {
                EffectType::Attribute(template.clone())
            } else {
                return Err(anyhow::anyhow!("No attribute effect found"));
            }
        }
        _ => return Err(anyhow::anyhow!("Effect type mismatch")),
    })
}
