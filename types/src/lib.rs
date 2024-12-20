use std::sync::Mutex;

pub mod asi;
pub mod attributes;
pub mod background;
pub mod classes;
pub mod dice;
pub mod fs;
pub mod items;
pub mod race;
pub mod size;
pub mod skill;
pub mod stores;

pub trait ForceLock<T> {
    fn force_lock(&self) -> std::sync::MutexGuard<'_, T>;
}

impl<T> ForceLock<T> for Mutex<T> {
    fn force_lock(&self) -> std::sync::MutexGuard<'_, T> {
        match self.lock() {
            Ok(i) => i,
            Err(poisoned) => poisoned.into_inner(),
        }
    }
}

pub trait StartsWithVowel {
    fn starts_with_vowel(&self) -> bool;
}

impl StartsWithVowel for str {
    fn starts_with_vowel(&self) -> bool {
        let vowels = ['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];
        vowels.contains(&self.chars().next().unwrap())
    }
}

impl StartsWithVowel for String {
    fn starts_with_vowel(&self) -> bool {
        self.as_str().starts_with_vowel()
    }
}

pub fn proficiency_bonus(level: u8) -> u8 {
    match level {
        1..=4 => 2,
        5..=8 => 3,
        9..=12 => 4,
        13..=16 => 5,
        17..=20 => 6,
        _ => 0,
    }
}

pub fn is_asi_level(level: u8) -> bool {
    matches!(level, 4 | 8 | 12 | 16 | 19)
}

pub fn deserialize_hashmap<'de, D, K, T>(
    d: D,
) -> std::result::Result<std::collections::HashMap<K, T>, D::Error>
where
    D: serde::Deserializer<'de>,
    K: std::str::FromStr + Eq + std::hash::Hash,
    T: serde::Deserialize<'de>,
{
    fn deserialize_string_key<'de, D, S>(d: D) -> std::result::Result<S, D::Error>
    where
        D: serde::Deserializer<'de>,
        S: std::str::FromStr,
    {
        let s: String = serde::Deserialize::deserialize(d).map_err(serde::de::Error::custom)?;
        s.parse::<S>()
            .map_err(|_| serde::de::Error::custom(format!("Invalid key: {}", s)))
    }

    #[derive(serde::Deserialize, Hash, Eq, PartialEq)]
    struct Wrapper<S: std::str::FromStr>(#[serde(deserialize_with = "deserialize_string_key")] S);

    let dict: std::collections::HashMap<Wrapper<K>, T> = serde::Deserialize::deserialize(d)?;
    Ok(dict.into_iter().map(|(Wrapper(k), v)| (k, v)).collect())
}
