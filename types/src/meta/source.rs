#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
pub enum Source {
    Official(String),
    Homebrew(String),
    #[default]
    Unknown,
}

impl std::fmt::Display for Source {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Source::Official(s) => write!(f, "{}", s),
            Source::Homebrew(s) => write!(f, "{}", s),
            Source::Unknown => write!(f, "Unknown"),
        }
    }
}

impl Source {
    pub fn is_official(&self) -> bool {
        matches!(self, Source::Official(_))
    }

    pub fn is_homebrew(&self) -> bool {
        matches!(self, Source::Homebrew(_))
    }
}
