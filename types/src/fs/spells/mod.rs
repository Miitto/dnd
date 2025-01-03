#![allow(clippy::module_inception)]

mod lists;
mod spells;

pub use lists::get_spell_lists;
pub use spells::get_spells;
