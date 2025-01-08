use crate::Named;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
#[serde(untagged)]
pub enum Link<T>
where
    T: Named,
{
    Found(T),
    NotFound(String),
}

impl<T> Link<T>
where
    T: Named,
{
    pub fn name(&self) -> String {
        match self {
            Link::Found(t) => t.name(),
            Link::NotFound(n) => n.to_owned(),
        }
    }
}
