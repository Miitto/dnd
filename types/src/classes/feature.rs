use crate::mechanics::Table;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ClassFeature {
    pub name: String,
    pub description: String,
    #[serde(default)]
    pub tables: Vec<Table>,
}

impl PartialEq<str> for ClassFeature {
    fn eq(&self, other: &str) -> bool {
        self.name == other
    }
}

impl PartialEq<ClassFeature> for ClassFeature {
    fn eq(&self, other: &ClassFeature) -> bool {
        self.name == other.name
    }
}
