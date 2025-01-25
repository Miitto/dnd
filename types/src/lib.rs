#![allow(incomplete_features)]
#![feature(specialization)]

use std::sync::{Arc, Mutex};

pub mod background;
pub mod classes;
pub mod feat;
pub mod fs;
pub mod items;
pub mod mechanics;
pub mod meta;
pub mod race;
pub mod spells;
pub mod stat_block;
pub mod stores;
pub mod traits;

pub mod extensions;

pub mod aliased;

use extensions::ForceLock;

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

pub trait Category: Named {
    fn category(&self) -> String;
}

pub trait CategoryMut: Category {
    fn category_mut(&mut self) -> &mut String;
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
