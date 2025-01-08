use std::{fs::DirEntry, path::Path};

use anyhow::Result;

#[allow(dead_code)]
pub fn recurse_dirs<P: AsRef<Path>>(dir: P, vec: &mut Vec<DirEntry>) -> Result<()> {
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

    let json = match read {
        Ok(json) => json,
        Err(e) => {
            return Err(anyhow::anyhow!("Failed to read file: {:?} | {:?}", path, e));
        }
    };

    let parsed = serde_json::from_str::<T>(&json);

    match parsed {
        Ok(parsed) => Ok(parsed),
        Err(e) => Err(anyhow::anyhow!(
            "Failed to parse file: {:?} | {:?}",
            path,
            e
        )),
    }
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

            if let Err(e) = &parsed {
                eprintln!("{:?}", e);
            }

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
