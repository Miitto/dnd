use std::borrow::Borrow;

use crate::common::Attribute;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub enum Skill {
    Acrobatics,
    AnimalHandling,
    Arcana,
    Athletics,
    Deception,
    History,
    Insight,
    Intimidation,
    Investigation,
    Medicine,
    Nature,
    Perception,
    Performance,
    Persuasion,
    Religion,
    SleightOfHand,
    Stealth,
    Survival,
}

impl From<Skill> for Attribute {
    fn from(skill: Skill) -> Attribute {
        match skill {
            Skill::Acrobatics => Attribute::Dexterity,
            Skill::AnimalHandling => Attribute::Wisdom,
            Skill::Arcana => Attribute::Intelligence,
            Skill::Athletics => Attribute::Strength,
            Skill::Deception => Attribute::Charisma,
            Skill::History => Attribute::Intelligence,
            Skill::Insight => Attribute::Wisdom,
            Skill::Intimidation => Attribute::Charisma,
            Skill::Investigation => Attribute::Intelligence,
            Skill::Medicine => Attribute::Wisdom,
            Skill::Nature => Attribute::Intelligence,
            Skill::Perception => Attribute::Wisdom,
            Skill::Performance => Attribute::Charisma,
            Skill::Persuasion => Attribute::Charisma,
            Skill::Religion => Attribute::Intelligence,
            Skill::SleightOfHand => Attribute::Dexterity,
            Skill::Stealth => Attribute::Dexterity,
            Skill::Survival => Attribute::Wisdom,
        }
    }
}

impl std::fmt::Display for Skill {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Skill::AnimalHandling => write!(f, "Animal Handling"),
            Skill::SleightOfHand => write!(f, "Sleight of Hand"),
            _ => write!(f, "{:?}", self),
        }
    }
}

impl Borrow<str> for Skill {
    fn borrow(&self) -> &str {
        match self {
            Skill::Acrobatics => "Acrobatics",
            Skill::AnimalHandling => "Animal Handling",
            Skill::Arcana => "Arcana",
            Skill::Athletics => "Athletics",
            Skill::Deception => "Deception",
            Skill::History => "History",
            Skill::Insight => "Insight",
            Skill::Intimidation => "Intimidation",
            Skill::Investigation => "Investigation",
            Skill::Medicine => "Medicine",
            Skill::Nature => "Nature",
            Skill::Perception => "Perception",
            Skill::Performance => "Performance",
            Skill::Persuasion => "Persuasion",
            Skill::Religion => "Religion",
            Skill::SleightOfHand => "Sleight of Hand",
            Skill::Stealth => "Stealth",
            Skill::Survival => "Survival",
        }
    }
}
