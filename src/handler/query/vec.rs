use std::fmt;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use serde::{Deserialize, Deserializer};
use serde::de::{self, IntoDeserializer, SeqAccess, Visitor};

#[derive(Debug, PartialEq)]
pub(crate) struct CommaSeparatedVec<T>(Vec<T>);

impl<T> Deref for CommaSeparatedVec<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target { &self.0 }
}

impl<T> DerefMut for CommaSeparatedVec<T> {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}

impl<'de, T> Deserialize<'de> for CommaSeparatedVec<T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(CommaSeparatedVecVisitor::<T> {
            marker: PhantomData,
        })
    }
}

struct CommaSeparatedVecVisitor<T> {
    marker: PhantomData<T>,
}

impl<'de, T> Visitor<'de> for CommaSeparatedVecVisitor<T>
where
    T: Deserialize<'de>,
{
    type Value = CommaSeparatedVec<T>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a comma separated string")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let vec = value
            .split(',')
            .map(|item| T::deserialize(item.trim().into_deserializer()))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(CommaSeparatedVec(vec))
    }
}
