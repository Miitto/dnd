use std::sync::Mutex;

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
