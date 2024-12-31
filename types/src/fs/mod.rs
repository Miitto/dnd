use std::{fs::DirEntry, path::Path};

pub mod classes;
mod constants;
pub mod spells;
pub mod weapons;

use constants::*;

use anyhow::Result;

use crate::{background::Background, race::Race};

#[allow(dead_code)]
fn recurse_dirs<P: AsRef<Path>>(dir: P, vec: &mut Vec<DirEntry>) -> Result<()> {
    let dir = dir.as_ref();
    if dir.is_dir() {
        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                recurse_dirs(&path, vec)?;
            } else {
                vec.push(entry);
            }
        }
    }
    Ok(())
}

pub fn parse_file<T, P: AsRef<Path>>(path: P) -> Result<T>
where
    T: serde::de::DeserializeOwned,
{
    let path = path.as_ref();
    let read = std::fs::read_to_string(path);

    let json = if let Ok(json) = read {
        json
    } else {
        eprintln!("Failed to read file: {:?}\n{}", path, read.unwrap_err());
        return Err(anyhow::anyhow!("Failed to read file: {:?}", path));
    };

    let parsed = serde_json::from_str::<T>(&json);

    if let Err(e) = parsed {
        eprintln!("Failed to parse file: {:?} - {:?}", path, e);
        return Err(anyhow::anyhow!("Failed to parse file: {:?}", path));
    }

    Ok(parsed?)
}

pub fn parse_dir<T, P: AsRef<Path>>(path: P) -> Result<Vec<T>>
where
    T: serde::de::DeserializeOwned,
{
    let dirs = std::fs::read_dir(path)?.filter_map(|dir| {
        dir.ok().filter(|dir| {
            dir.path().is_file() && dir.path().extension().map_or(false, |ext| ext == "json")
        })
    });

    Ok(dirs
        .filter_map(|dir| {
            let path = dir.path();

            let ext = path.extension()?.to_str()?;

            if ext != "json" {
                return None;
            }

            let parsed = parse_file(&path);

            parsed.ok()
        })
        .collect())
}

pub fn recurse_category<T, P: AsRef<Path> + std::fmt::Debug>(path: P) -> Result<Vec<T>>
where
    T: serde::de::DeserializeOwned + crate::CategoryMut,
{
    let cats = std::fs::read_dir(&path)?.filter_map(|cat| {
        let cat = cat.ok()?;
        let cat_path = cat.path();
        if !cat_path.is_dir() {
            return None;
        }

        let cat_name = cat_path.file_name()?.to_str()?.to_string();

        Some((cat_name, cat_path))
    });

    Ok(cats
        .filter_map(|(cat, path)| {
            parse_dir(path).ok().map(|mut vec| {
                vec.iter_mut()
                    .for_each(|item: &mut T| *(item.category_mut()) = cat.clone());
                vec
            })
        })
        .flatten()
        .collect())
}

pub fn get_races<P: AsRef<Path>>(resource_path: P) -> Result<Vec<Race>> {
    let resource_path = resource_path.as_ref();

    let race_path = resource_path.join(RACE_PATH);

    recurse_category(race_path)
}

pub fn get_backgrounds<P: AsRef<Path>>(resource_path: P) -> Result<Vec<Background>> {
    let resource_path = resource_path.as_ref();

    let background_path = resource_path.join(BACKGROUND_PATH);

    recurse_category(background_path)
}

pub fn get_feats<P: AsRef<Path>>(resource_path: P) -> Result<Vec<crate::feat::Feat>> {
    let resource_path = resource_path.as_ref();

    let feat_path = resource_path.join(FEAT_PATH);

    parse_dir(feat_path)
}
