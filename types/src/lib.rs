use std::sync::{Arc, Mutex};

pub mod background;
pub mod classes;
pub mod common;
pub mod feat;
pub mod fs;
pub mod items;
pub mod race;
pub mod spells;
pub mod stat_block;
pub mod stores;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
#[serde(untagged)]
pub enum Link<T>
where
    T: Named,
{
    Found(T),
    NotFound(String),
}

impl<T> Link<T>
where
    T: Named,
{
    pub fn name(&self) -> String {
        match self {
            Link::Found(t) => t.name(),
            Link::NotFound(n) => n.to_owned(),
        }
    }
}

impl<T> Named for Arc<Mutex<T>>
where
    T: Named,
{
    fn name(&self) -> String {
        self.force_lock().name()
    }
}

pub trait Named {
    fn name(&self) -> String;
}

pub trait Category {
    fn category(&self) -> String;
}

pub trait CategoryMut {
    fn category_mut(&mut self) -> &mut String;
}

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

pub trait IsFalse {
    fn is_false(&self) -> bool;
}

impl IsFalse for bool {
    fn is_false(&self) -> bool {
        !self
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
