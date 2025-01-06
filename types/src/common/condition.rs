#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct Condition {
    pub name: String,
    pub description: String,
}
