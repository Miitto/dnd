use crate::Named;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum Link<T> {
    Found(T),
    NotFound(String),
}

impl<T> PartialEq for Link<T>
where
    T: Named,
{
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Found(a), Self::Found(b)) => a.name() == b.name(),
            (Self::NotFound(a), Self::NotFound(b)) => a == b,
            _ => false,
        }
    }
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

impl<T> Link<T> {
    pub fn found(&mut self, value: T) -> &mut Self {
        *self = Self::Found(value);

        self
    }
}
