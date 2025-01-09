use std::fs::DirEntry;
use std::path::Path;

use crate::classes::{Class, Subclass};

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
            file.path()
                .file_stem()
                .and_then(|stem| stem.to_str())
                .map(|s| s == CLASS_BASE_NAME)
                .unwrap_or(false)
        });

        let class_file = if let Some(class_file) = class_file {
            class_file.path()
        } else {
            continue;
        };

        let mut class: Class = crate::fs::parsers::parse_file(&class_file)?;

        for subclass_file in class_files.iter().filter(|file| {
            file.path()
                .file_stem()
                .and_then(|stem| stem.to_str())
                .map(|s| s != CLASS_BASE_NAME)
                .unwrap_or(false)
        }) {
            let mut subclass: Subclass = match crate::fs::parsers::parse_file(subclass_file.path())
            {
                Ok(subclass) => subclass,
                Err(e) => {
                    eprintln!("Error parsing subclass file: {:?}", e);
                    continue;
                }
            };

            subclass.class = class.name.clone();

            class
                .subclasses
                .options
                .insert(subclass.name.clone(), subclass);
        }

        classes.push(class);
    }

    Ok(classes)
}
