use std::{marker::PhantomData, vec};

use crate::{
    common::{Attribute, Condition, Damage, Dice},
    IsFalse,
};

use super::{Components, OnSave};

use serde::{
    de::{MapAccess, SeqAccess, Visitor},
    Deserialize,
};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
pub struct Spell {
    pub name: String,
    pub level: u8,
    pub school: String,
    pub components: Components,
    pub cast_time: String,
    pub range: String,
    pub duration: String,
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub at_higher_levels: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub save: Option<Attribute>,
    #[serde(
        default,
        deserialize_with = "deserialize_vec_or_map",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub damage: Vec<Damage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heal: Option<Dice>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<Condition>,
    #[serde(default, skip_serializing_if = "bool::is_false")]
    pub concentration: bool,
    #[serde(default, skip_serializing_if = "bool::is_false")]
    pub ritual: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_save: Option<OnSave>,
}

impl PartialEq<str> for Spell {
    fn eq(&self, other: &str) -> bool {
        self.name == other
    }
}

impl PartialEq for Spell {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

fn deserialize_vec_or_map<'de, D, T>(deserializer: D) -> Result<Vec<T>, D::Error>
where
    D: serde::Deserializer<'de>,
    T: serde::Deserialize<'de>,
{
    struct VecOrSingle<T>(PhantomData<T>);
    impl<'de, T> Visitor<'de> for VecOrSingle<T>
    where
        T: serde::Deserialize<'de>,
    {
        type Value = Vec<T>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a sequence or a single element")
        }

        fn visit_seq<A>(self, seq: A) -> Result<Self::Value, A::Error>
        where
            A: SeqAccess<'de>,
        {
            Deserialize::deserialize(serde::de::value::SeqAccessDeserializer::new(seq))
        }

        fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
        where
            A: MapAccess<'de>,
        {
            Deserialize::deserialize(serde::de::value::MapAccessDeserializer::new(map))
                .map(|x| vec![x])
        }
    }

    let res: Result<Vec<T>, D::Error> = deserializer.deserialize_any(VecOrSingle(PhantomData));

    res
}
