use std::path::Path;

use crate::background::Background;

use super::{constants::BACKGROUND_PATH, recurse_dirs};

use anyhow::Result;

pub fn get_backgrounds<P: AsRef<Path>>(resource_path: P) -> Result<Vec<Background>> {
    let resource_path = resource_path.as_ref();

    let background_path = resource_path.join(BACKGROUND_PATH);

    dbg!(&background_path);

    let mut background_dirs = Vec::new();

    recurse_dirs(&background_path, &mut background_dirs)?;

    println!("Found {} files in background dir", background_dirs.len());

    let backgrounds = background_dirs
        .iter()
        .filter_map(|dir| {
            let path = dir.path();

            if path.is_dir() {
                return None;
            }

            let ext = path.extension()?.to_str()?;

            if ext != "json" {
                return None;
            }

            let category = path
                .parent()
                .unwrap()
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string();

            let read = std::fs::read_to_string(&path);

            let json = if let Ok(json) = read {
                json
            } else {
                eprintln!("Failed to read file: {:?}\n{}", path, read.unwrap_err());
                return None;
            };

            let parsed = serde_json::from_str::<Background>(&json);

            if let Err(e) = parsed {
                eprintln!("Failed to parse file: {:?} - {:?}", path, e);
                return None;
            }

            parsed.ok().map(|mut r| {
                r.category = category.clone();
                r
            })
        })
        .collect();

    Ok(backgrounds)
}
