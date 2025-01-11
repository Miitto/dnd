use crate::{mechanics::Attributes, meta::Description};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
pub struct Feat {
    pub name: String,
    pub description: Description,
    pub attributes: Attributes,
    pub benefits: Vec<String>,
}

impl PartialEq<str> for Feat {
    fn eq(&self, other: &str) -> bool {
        self.name == other
    }
}

impl PartialEq<Feat> for Feat {
    fn eq(&self, other: &Feat) -> bool {
        self.name == other.name
    }
}

impl crate::Named for Feat {
    fn name(&self) -> String {
        self.name.clone()
    }
}
