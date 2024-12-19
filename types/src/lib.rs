use std::sync::Mutex;

pub mod asi;
pub mod attributes;
pub mod background;
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
