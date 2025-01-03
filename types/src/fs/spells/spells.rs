use std::{fs::DirEntry, path::Path};

use anyhow::Result;

use crate::{
    fs::{parse_file, recurse_dirs, SPELL_LIST_PATH, SPELL_PATH},
    spells::Spell,
};

pub fn get_spells<P: AsRef<Path>>(resource_path: P) -> Result<Vec<Spell>> {
    let spell_path = resource_path.as_ref().join(SPELL_PATH);

    let list_path = resource_path.as_ref().join(SPELL_LIST_PATH);

    let child_dirs = std::fs::read_dir(&spell_path)?
        .filter_map(|dir| {
            let dir = dir.ok()?;
            let path = dir.path();

            if path == list_path {
                return None;
            }

            Some(path)
        })
        .collect::<Vec<_>>();

    let mut dirs: Vec<DirEntry> = vec![];
    for dir in child_dirs.iter() {
        let _ = recurse_dirs(dir, &mut dirs);
    }

    let spells = dirs
        .iter()
        .filter_map(|entry| {
            Some(entry).filter(|entry| {
                entry
                    .path()
                    .extension()
                    .map(|ext| ext == "json")
                    .unwrap_or(false)
            })
        })
        .filter_map(|spell| {
            let path = spell.path();

            let parsed: Result<Spell> = parse_file(path);

            parsed.ok()
        })
        .collect();

    Ok(spells)
}

impl Spell {
    pub fn serialize(&self) -> Result<String> {
        serde_json::to_string(self).map_err(Into::into)
    }

    pub fn serialize_pretty(&self) -> Result<String> {
        serde_json::to_string_pretty(self).map_err(Into::into)
    }
}
