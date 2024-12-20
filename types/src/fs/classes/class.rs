use std::fs::DirEntry;
use std::path::Path;

use crate::classes::class::Class;

use crate::fs::constants::{CLASS_BASE_NAME, CLASS_PATH};

use anyhow::Result;

pub fn get_classes<P: AsRef<Path>>(resource_path: P) -> Result<Vec<Class>> {
    let resource_path = resource_path.as_ref();

    let class_path = resource_path.join(CLASS_PATH);

    let classes_dir = std::fs::read_dir(&class_path)?.filter(|dir| {
        dir.as_ref()
            .map(|d| d.path().is_dir())
            .unwrap_or_else(|_| false)
    });

    let mut classes = Vec::new();

    for class_dir in classes_dir {
        let class_dir = class_dir?;
        let class_path = class_dir.path();

        let class_files: Vec<DirEntry> = std::fs::read_dir(&class_path)?
            .filter_map(|dir| {
                dir.ok().and_then(|d| {
                    let path = d.path();
                    if path.is_file() {
                        Some(d)
                    } else {
                        None
                    }
                })
            })
            .collect();

        let class_file = class_files.iter().find(|file| {
            file.file_name()
                .to_str()
                .map(|s| s.starts_with(CLASS_BASE_NAME))
                .unwrap_or(false)
        });

        let class_file = if let Some(class_file) = class_file {
            class_file.path()
        } else {
            continue;
        };

        let read = std::fs::read_to_string(&class_file);

        let json = if let Ok(json) = read {
            json
        } else {
            eprintln!("Failed to read file: {:?}", class_file);
            continue;
        };

        let class: Class = match serde_json::from_str(&json) {
            Ok(class) => class,
            Err(e) => {
                eprintln!("Failed to parse class: {:?}", e);
                continue;
            }
        };

        classes.push(class);
    }

    Ok(classes)
}
