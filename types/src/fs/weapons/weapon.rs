use std::{fs::DirEntry, path::Path};

use serde::{Deserialize, Serialize};

use anyhow::{Context, Result};

use crate::{
    common::Damage,
    items::{
        properties::{AttributeReplacement, EffectType, Property, PropertyEffect},
        weapon::Weapon,
        Rarity,
    },
};

use crate::fs::constants::*;

use super::property::{combine_property_effect, get_weapon_properties};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializedWeapon {
    pub name: String,
    pub damage: Damage,
    pub rarity: Rarity,
    pub properties: Vec<SerializedWeaponProperty>,
    pub weight: f32,
    pub subtype: Vec<String>,
    pub range: u32,
}

/// A serialized weapon property, with optional parameters.
/// Stored in the weapon file and referrers to a property file.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializedWeaponProperty {
    pub name: String,
    pub parameters: Option<Vec<SerializedPropertyEffectType>>,
}

impl PartialEq<String> for SerializedWeaponProperty {
    fn eq(&self, other: &String) -> bool {
        self.name.to_lowercase() == *other.to_lowercase()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SerializedPropertyEffectType {
    #[serde(rename = "damage")]
    Damage(Option<Damage>),
    #[serde(rename = "attribute")]
    Attribute(Option<AttributeReplacement>),
}

impl SerializedPropertyEffectType {
    pub fn same_type(&self, other: &SerializedPropertyEffectType) -> bool {
        matches!(
            (self, other),
            (Self::Damage(_), Self::Damage(_)) | (Self::Attribute(_), Self::Attribute(_))
        )
    }
}

impl TryInto<EffectType> for SerializedPropertyEffectType {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<EffectType> {
        match self {
            Self::Damage(damage) => Ok(EffectType::Damage(
                damage.ok_or(anyhow::anyhow!("No damage"))?,
            )),
            Self::Attribute(attr) => Ok(EffectType::Attribute(
                attr.ok_or(anyhow::anyhow!("No attribute"))?,
            )),
        }
    }
}

fn get_serialized_weapons<P: AsRef<Path>>(resource_path: P) -> Result<Vec<SerializedWeapon>> {
    let resource_path = resource_path.as_ref();

    let weapons_path = resource_path.join(ITEM_WEAPON_PATH);

    let weapons_dir: Vec<DirEntry> = std::fs::read_dir(&weapons_path)
        .with_context(|| format!("Failed to read weapons dir at: {:?}", weapons_path))?
        .filter_map(|el| el.ok())
        .collect();

    let weapons_iter = weapons_dir.iter().filter_map(|entry| {
        Some(entry).filter(|entry| {
            entry.file_type().map(|ft| ft.is_file()).unwrap_or(false)
                && entry
                    .path()
                    .extension()
                    .map(|ext| ext == "json")
                    .unwrap_or(false)
        })
    });

    Ok(weapons_iter
        .filter_map(|weapon| {
            let path = weapon.path();
            let json = std::fs::read_to_string(&path).ok()?;

            let parsed = serde_json::from_str::<SerializedWeapon>(&json);

            if let Ok(weapon) = parsed {
                Some(weapon)
            } else {
                eprintln!(
                    "Error reading weapon file: {:?}\n{}",
                    path,
                    parsed.unwrap_err()
                );
                None
            }
        })
        .collect())
}

pub fn get_weapons<P: AsRef<Path>>(resource_path: P) -> Result<Vec<Weapon>> {
    let serialized_weapons = get_serialized_weapons(&resource_path)?;

    let properties = get_weapon_properties(&resource_path)?;

    let weapons = serialized_weapons
        .into_iter()
        .map(|serialized| {
            let properties: Vec<Property> = serialized
                .properties
                .iter()
                .filter_map(|prop| {
                    let template = properties.iter().find(|&p| *p == *prop)?;

                    let mut effects = Vec::new();

                    for effect in template.effects.iter() {
                        let weapon_effect = prop
                            .parameters
                            .as_ref()
                            .and_then(|p| p.iter().find(|&p| p.same_type(&effect.effect_type)));

                        let effect_type = if let Some(weapon_effect) = weapon_effect {
                            combine_property_effect(weapon_effect, &effect.effect_type).ok()?
                        } else {
                            let effect: EffectType = effect.effect_type.clone().try_into().ok()?;
                            effect
                        };

                        let effect = PropertyEffect {
                            optional: effect.optional,
                            when: effect.when.clone(),
                            effect_type,
                        };

                        effects.push(effect);
                    }

                    Some(Property {
                        name: template.name.clone(),
                        description: template.description.clone(),
                        effects,
                    })
                })
                .collect();

            Weapon::new(
                serialized.name,
                serialized.damage,
                serialized.rarity,
                properties,
                serialized.weight,
                serialized.subtype,
                serialized.range,
            )
        })
        .collect();

    Ok(weapons)
}
