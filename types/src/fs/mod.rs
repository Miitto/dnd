use std::{fs::DirEntry, path::Path};

pub mod background;
pub mod constants;
pub mod race;
pub mod weapons;

fn recurse_dirs(dir: &Path, vec: &mut Vec<DirEntry>) -> std::io::Result<()> {
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
