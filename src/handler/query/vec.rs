use std::fmt;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use serde::{Deserialize, Deserializer};
use serde::de::{self, IntoDeserializer, SeqAccess, Visitor};
use serde::de::value::StrDeserializer;

#[derive(Debug, PartialEq)]
pub(crate) struct CommaSeparatedVec<T>(Vec<T>);

impl<T> CommaSeparatedVec<T> {
    pub fn from_iter<I: IntoIterator<Item=T>>(iter: I) -> Self {
        Self(iter.into_iter().collect())
    }
}

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
        deserializer.deserialize_str(CommaSeparatedVecVisitor::<T> { marker: PhantomData })
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
            .map(|item| item.trim())
            .filter(|item| !item.is_empty())
            .map(|item| {
                T::deserialize(item.into_deserializer())
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(CommaSeparatedVec(vec))
    }
}

#[cfg(test)]
mod tests {
    use serde::de::Error;
    use serde::de::value::StrDeserializer;
    use crate::handler::query::AutoFeature;
    use super::*;

    type E = de::value::Error;

    #[test]
    fn test_comma_separated_vec_deserialize() {
        let testcases = vec![
            ("", vec![]),
            ("compress", vec![AutoFeature::Compress]),
            ("compress, format", vec![AutoFeature::Compress, AutoFeature::Format]),
        ];

        for testcase in testcases {
            assert_eq!(
                CommaSeparatedVec::deserialize::<StrDeserializer<E>>(
                    testcase.0.into_deserializer()
                ),
                Ok(CommaSeparatedVec(testcase.1))
            );
        }
    }

    #[test]
    fn test_comma_separated_vec_deserialize_error() {
        assert_eq!(
            CommaSeparatedVec::<AutoFeature>::deserialize::<StrDeserializer<E>>(
                "compress,format,invalid".into_deserializer()
            ),
            Err(Error::custom("unknown variant `invalid`, expected `compress` or `format`"))
        );
    }
}
