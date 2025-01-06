use std::path::Path;

pub mod classes;
pub mod constants;
pub mod spells;
pub mod weapons;

pub mod deserializers;
pub mod parsers;

use constants::*;

use anyhow::Result;

use parsers::{parse_dir, recurse_category};

use crate::{background::Background, race::Race, spells::SpellList, CategoryMut};

fn cat<P: AsRef<Path>, T>(resource_path: P, join: &str) -> Result<Vec<T>>
where
    T: serde::de::DeserializeOwned + CategoryMut,
{
    let resource_path = resource_path.as_ref();

    let path = resource_path.join(join);

    recurse_category(path)
}

fn dir<P: AsRef<Path>, T>(resource_path: P, join: &str) -> Result<Vec<T>>
where
    T: serde::de::DeserializeOwned,
{
    let resource_path = resource_path.as_ref();

    let path = resource_path.join(join);

    parse_dir(path)
}

pub fn get_races<P: AsRef<Path>>(resource_path: P) -> Result<Vec<Race>> {
    cat(resource_path, RACE_PATH)
}

pub fn get_backgrounds<P: AsRef<Path>>(resource_path: P) -> Result<Vec<Background>> {
    cat(resource_path, BACKGROUND_PATH)
}

pub fn get_feats<P: AsRef<Path>>(resource_path: P) -> Result<Vec<crate::feat::Feat>> {
    dir(resource_path, FEAT_PATH)
}

pub fn get_stat_blocks<P: AsRef<Path>>(
    resource_path: P,
) -> Result<Vec<crate::stat_block::StatBlock>> {
    dir(resource_path, STAT_BLOCK_PATH)
}

pub fn get_spell_lists<P: AsRef<Path>>(resource_path: P) -> Result<Vec<SpellList>> {
    dir(resource_path, SPELL_LIST_PATH)
}
