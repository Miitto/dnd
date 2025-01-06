use std::marker::PhantomData;

use serde::{
    de::{MapAccess, SeqAccess, Visitor},
    Deserialize,
};

pub fn deserialize_vec_or_map<'de, D, T>(deserializer: D) -> Result<Vec<T>, D::Error>
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

pub fn deserialize_hashmap<'de, D, K, T>(
    d: D,
) -> std::result::Result<std::collections::HashMap<K, T>, D::Error>
where
    D: serde::Deserializer<'de>,
    K: std::str::FromStr + Eq + std::hash::Hash,
    T: serde::Deserialize<'de>,
{
    fn deserialize_string_key<'de, D, S>(d: D) -> std::result::Result<S, D::Error>
    where
        D: serde::Deserializer<'de>,
        S: std::str::FromStr,
    {
        let s: String = serde::Deserialize::deserialize(d).map_err(serde::de::Error::custom)?;
        s.parse::<S>()
            .map_err(|_| serde::de::Error::custom(format!("Invalid key: {}", s)))
    }

    #[derive(serde::Deserialize, Hash, Eq, PartialEq)]
    struct Wrapper<S: std::str::FromStr>(#[serde(deserialize_with = "deserialize_string_key")] S);

    let dict: std::collections::HashMap<Wrapper<K>, T> = serde::Deserialize::deserialize(d)?;
    Ok(dict.into_iter().map(|(Wrapper(k), v)| (k, v)).collect())
}
