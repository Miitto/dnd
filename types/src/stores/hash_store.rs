use std::{
    collections::HashMap,
    path::PathBuf,
    sync::{Arc, Mutex},
};

use crate::{
    classes::Class,
    extensions::{ForceLock, SnakeCase},
    items::weapon::Weapon,
    spells::Spell,
    Category, Named,
};

use anyhow::Result;

#[derive(Debug, Clone)]
pub struct HashStore<T>
where
    T: PartialEq<str> + Clone,
{
    pub store: Arc<Mutex<HashMap<String, Arc<Mutex<T>>>>>,
    path: PathBuf,
}

impl<T> HashStore<T>
where
    T: PartialEq<str> + Clone,
{
    pub fn new(path: PathBuf) -> Self {
        Self {
            path,
            store: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn all(&self) -> HashMap<String, T> {
        let store = self.store.force_lock();
        store
            .iter()
            .map(|(k, v)| (k.clone(), (v.lock().expect("Failed to lock item")).clone()))
            .collect()
    }

    pub fn all_vec(&self) -> Vec<T> {
        let store = self.store.force_lock();
        store
            .values()
            .map(|v| (v.lock().expect("Failed to lock item")).clone())
            .collect()
    }

    pub fn set(&self, name: String, item: T) {
        let mut store = self.store.force_lock();
        let existing = store.get(&name);
        if let Some(existing) = existing {
            *existing.lock().expect("Failed to lock existing item") = item;
        } else {
            store.insert(name, Arc::new(Mutex::new(item)));
        }
    }

    pub fn get(&self, name: &str) -> Option<Arc<Mutex<T>>> {
        let store = self.store.force_lock();
        store.get(name).map(Arc::clone)
    }

    pub fn get_clone(&self, name: &str) -> Option<T> {
        self.get(name)
            .map(|item| item.lock().expect("Failed to lock item").clone())
    }

    pub fn get_arced(&self, name: &str) -> Option<Arc<T>> {
        self.get_clone(name).map(Arc::new)
    }
}

impl HashStore<Weapon> {
    pub fn melee(&self) -> Vec<Weapon> {
        self.store
            .force_lock()
            .values()
            .filter_map(|item| {
                let item = item.lock().expect("Failed to lock item");
                if item.is_melee() {
                    Some(item.clone())
                } else {
                    None
                }
            })
            .collect()
    }
}

pub trait Saveable {
    fn save(&self, item: &str) -> Result<()>;
}

impl<T> Saveable for HashStore<T>
where
    T: serde::Serialize + Category + PartialEq<str> + Clone,
{
    fn save(&self, item: &str) -> Result<()> {
        let item = if let Some(item) = self.get(item) {
            item
        } else {
            return Err(anyhow::anyhow!("Failed to get category item to save"));
        };

        let item = if let Ok(item) = item.lock() {
            item
        } else {
            return Err(anyhow::anyhow!("Failed to lock category item to save"));
        };

        let path = self
            .path
            .join(item.category().to_snake_case())
            .join(format!("{}.json", item.name().to_snake_case()));

        let serialized = if let Ok(serialized) = serde_json::to_string_pretty(&*item) {
            serialized
        } else {
            return Err(anyhow::anyhow!("Failed to serialize category item to save"));
        };

        println!("Saving Category to {:?}", path);

        match std::fs::write(path, serialized) {
            Ok(_) => Ok(()),
            Err(e) => Err(anyhow::anyhow!("Failed to save {}: {:?}", item.name(), e)),
        }
    }
}

impl<T> Saveable for HashStore<T>
where
    T: serde::Serialize + Named + PartialEq<str> + Clone,
{
    default fn save(&self, item: &str) -> Result<()> {
        let item = if let Some(item) = self.get(item) {
            item
        } else {
            return Err(anyhow::anyhow!("Failed to get named item to save"));
        };

        let item = if let Ok(item) = item.lock() {
            item
        } else {
            return Err(anyhow::anyhow!("Failed to lock named item to save"));
        };

        let path = self
            .path
            .join(format!("{}.json", item.name().to_snake_case()));

        let serialized = if let Ok(serialized) = serde_json::to_string_pretty(&*item) {
            serialized
        } else {
            return Err(anyhow::anyhow!("Failed to serialize named item to save"));
        };

        println!("Saving Named to {:?}", path);

        match std::fs::write(path, serialized) {
            Ok(_) => Ok(()),
            Err(e) => Err(anyhow::anyhow!("Failed to save {}: {:?}", item.name(), e)),
        }
    }
}

impl Saveable for HashStore<Spell> {
    fn save(&self, item: &str) -> Result<()> {
        let item = if let Some(item) = self.get(item) {
            item
        } else {
            return Err(anyhow::anyhow!("Failed to get spell item to save"));
        };

        let item = if let Ok(item) = item.lock() {
            item
        } else {
            return Err(anyhow::anyhow!("Failed to lock spell item to save"));
        };

        let path = if item.level == 0 {
            self.path.join("cantrips")
        } else {
            self.path.join("levelled").join(item.level.to_string())
        };

        let path = path.join(format!("{}.json", &*item.name.to_snake_case()));

        let serialized = if let Ok(serialized) = serde_json::to_string_pretty(&*item) {
            serialized
        } else {
            return Err(anyhow::anyhow!("Failed to serialize spell item to save"));
        };

        println!("Saving Spell to {:?}", path);

        match std::fs::write(path, serialized) {
            Ok(_) => Ok(()),
            Err(e) => Err(anyhow::anyhow!("Failed to save {}: {:?}", item.name(), e)),
        }
    }
}

impl Saveable for HashStore<Class> {
    fn save(&self, item: &str) -> Result<()> {
        let (name, path, serialized) = if let Some(item) = self.get(item) {
            let item = if let Ok(item) = item.lock() {
                item
            } else {
                return Err(anyhow::anyhow!("Failed to lock class item to save"));
            };

            let path = self.path.join(item.name.to_snake_case()).join("class.json");

            let serialized = if let Ok(serialized) = serde_json::to_string_pretty(&*item) {
                serialized
            } else {
                return Err(anyhow::anyhow!("Failed to serialize class item to save"));
            };

            println!("Saving Class to {:?}", path);
            (item.name.clone(), path, serialized)
        } else {
            let (class, subclass) = item.split_once('/').unwrap_or((item, ""));

            let class = if let Some(class) = self.get(class) {
                class
            } else {
                return Err(anyhow::anyhow!("Failed to get class item to save"));
            };

            let class = if let Ok(class) = class.lock() {
                class
            } else {
                return Err(anyhow::anyhow!("Failed to lock class item to save"));
            };

            let path = self.path.join(class.name.to_snake_case());

            let subclass = if let Some(subclass) = class.subclasses.options.get(subclass) {
                subclass
            } else {
                return Err(anyhow::anyhow!("Failed to get subclass item to save"));
            };

            let path = path.join(format!("{}.json", subclass.name.to_snake_case()));

            let serialized = if let Ok(serialized) = serde_json::to_string_pretty(subclass) {
                serialized
            } else {
                return Err(anyhow::anyhow!("Failed to serialize subclass item to save"));
            };

            println!("Saving Subclass to {:?}", path);
            (subclass.name.clone(), path, serialized)
        };

        match std::fs::write(path, serialized) {
            Ok(_) => Ok(()),
            Err(e) => Err(anyhow::anyhow!("Failed to save {}: {:?}", name, e)),
        }
    }
}
