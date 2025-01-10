#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub enum Alignment {
    LawfulGood,
    NeutralGood,
    ChaoticGood,
    LawfulNeutral,
    TrueNeutral,
    ChaoticNeutral,
    LawfulEvil,
    NeutralEvil,
    ChaoticEvil,
}

impl std::fmt::Display for Alignment {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Alignment::LawfulGood => write!(f, "Lawful Good"),
            Alignment::NeutralGood => write!(f, "Neutral Good"),
            Alignment::ChaoticGood => write!(f, "Chaotic Good"),
            Alignment::LawfulNeutral => write!(f, "Lawful Neutral"),
            Alignment::TrueNeutral => write!(f, "True Neutral"),
            Alignment::ChaoticNeutral => write!(f, "Chaotic Neutral"),
            Alignment::LawfulEvil => write!(f, "Lawful Evil"),
            Alignment::NeutralEvil => write!(f, "Neutral Evil"),
            Alignment::ChaoticEvil => write!(f, "Chaotic Evil"),
        }
    }
}
