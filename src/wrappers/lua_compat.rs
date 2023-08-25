//! Lua tables are a bit funky and an empty array is the same as an empty object in that language.
//! In places where we are specifically expecting an array, we can use this deserializer to ensure
//! serde doesn't get hung up on the fact that our "array" is actually an empty object.

use std::ops::{Deref, DerefMut};

use serde::de::{Deserialize, DeserializeOwned, Deserializer, Error, Visitor};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct LuaVec<T: DeserializeOwned>(#[serde(deserialize_with = "deserialize_with")] pub Vec<T>);

impl<T: DeserializeOwned> Deref for LuaVec<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: DeserializeOwned> DerefMut for LuaVec<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug)]
pub(crate) enum EmptyVecOrEmptyObject<T> {
    Vec(Vec<T>),
    Object,
}

impl<T> EmptyVecOrEmptyObject<T> {
    pub fn into_vec(self) -> Vec<T> {
        match self {
            Self::Vec(v) => v,
            Self::Object => Vec::new(),
        }
    }
}

struct EmptyVecOrEmptyObjectVisitor<T>(std::marker::PhantomData<T>);

pub fn deserialize_with<'de, D: Deserializer<'de>, T: Deserialize<'de>>(
    d: D,
) -> Result<Vec<T>, D::Error> {
    Ok(EmptyVecOrEmptyObject::deserialize(d)?.into_vec())
}

impl<'de, T> Visitor<'de> for EmptyVecOrEmptyObjectVisitor<T>
where
    T: Deserialize<'de>,
{
    type Value = EmptyVecOrEmptyObject<T>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("an empty object or an array")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        match map.size_hint() {
            Some(size) if size > 0 => Err(A::Error::custom(
                "expected an empty object, found a non-empty object",
            )),
            Some(size) if size == 0 => Ok(EmptyVecOrEmptyObject::Object),
            _ => {
                if map.next_entry::<Value, Value>()?.is_some() {
                    Err(A::Error::custom(
                        "expected an empty object, found a non-empty object",
                    ))
                } else {
                    Ok(EmptyVecOrEmptyObject::Object)
                }
            }
        }
    }

    fn visit_seq<A>(self, seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        let values: Result<Vec<T>, A::Error> =
            serde::Deserialize::deserialize(serde::de::value::SeqAccessDeserializer::new(seq));
        match values {
            Ok(vec) => Ok(EmptyVecOrEmptyObject::Vec(vec)),
            Err(_) => Err(A::Error::custom("expected an empty object or an array")),
        }
    }
}

impl<'de, T> Deserialize<'de> for EmptyVecOrEmptyObject<T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<EmptyVecOrEmptyObject<T>, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(EmptyVecOrEmptyObjectVisitor(std::marker::PhantomData))
    }
}
