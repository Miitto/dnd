use crate::mechanics::Attribute;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ASI {
    pub attribute: Attribute,
    pub change: i8,
}