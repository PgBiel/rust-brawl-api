#![doc(hidden)]
use std::str::FromStr;
use std::fmt::Display;
use serde::{self, Deserializer, Deserialize};
use num_traits::PrimInt;

// Credit to `serde-aux` crate
pub(crate) fn deserialize_number_from_string<'de, T, D>(deserializer: D) -> Result<T, D::Error>
    where
        D: Deserializer<'de>,
        T: FromStr + serde::Deserialize<'de>,
        <T as FromStr>::Err: Display,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StringOrInt<T> {
        String(String),
        Number(T),
    }

    match StringOrInt::<T>::deserialize(deserializer)? {
        StringOrInt::String(s) => s.parse::<T>().map_err(serde::de::Error::custom),
        StringOrInt::Number(i) => Ok(i),
    }
}

/// Obtains '1' for an arbitrary number type.
pub(crate) fn one_default<T>() -> T
    where T: PrimInt + FromStr,
          <T as FromStr>::Err: ::std::fmt::Debug
{ "1".parse().unwrap() }
