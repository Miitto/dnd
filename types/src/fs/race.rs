use std::path::Path;

use crate::race::Race;

use super::{constants::RACE_PATH, recurse_dirs};

use anyhow::Result;

pub fn get_races<P: AsRef<Path>>(resource_path: P) -> Result<Vec<Race>> {
    let resource_path = resource_path.as_ref();

    let race_path = resource_path.join(RACE_PATH);

    dbg!(&race_path);

    let mut race_dirs = Vec::new();

    recurse_dirs(&race_path, &mut race_dirs)?;

    println!("Found {} files in race dir", race_dirs.len());

    let races = race_dirs
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

            let parsed = serde_json::from_str::<Race>(&json);

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

    Ok(races)
}
