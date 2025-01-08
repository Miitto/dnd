use crate::meta::Description;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct Condition {
    pub name: String,
    pub description: Description,
}
