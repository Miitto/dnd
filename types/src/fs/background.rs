use std::path::Path;

use crate::background::Background;

use super::{constants::BACKGROUND_PATH, recurse_category};

use anyhow::Result;

pub fn get_backgrounds<P: AsRef<Path>>(resource_path: P) -> Result<Vec<Background>> {
    let resource_path = resource_path.as_ref();

    let background_path = resource_path.join(BACKGROUND_PATH);

    recurse_category(background_path)
}
