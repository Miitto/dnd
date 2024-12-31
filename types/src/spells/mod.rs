mod list;
mod spell;

pub use list::SpellList;
pub use spell::Spell;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Components {
    #[serde(default)]
    pub verbal: bool,
    #[serde(default)]
    pub somatic: bool,
    #[serde(default)]
    pub material: Vec<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum OnSave {
    Half,
    None,
}
