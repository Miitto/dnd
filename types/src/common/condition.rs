#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Condition {
    pub name: String,
    pub description: String,
}
