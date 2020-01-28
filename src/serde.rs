#![doc(hidden)]
use std::str::FromStr;
use std::fmt::Display;
use std::result::Result as StdResult;  // in order to not confuse with the library's Result
use serde::{self, Deserializer, Deserialize, Serialize, Serializer};
use num_traits::PrimInt;
use std::ops::{Deref, DerefMut};

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

/// Serializes a smart pointer class.
pub(crate) fn serialize_smt_pointer<T, S>(
    object: &T, serializer: S
) -> StdResult<S::Ok, S::Error>
    where
        S: Serializer,
        T: Deref,
        <T as Deref>::Target: ::serde::Serialize
{
    object.serialize(serializer)
}

/// Deserializes a smart pointer class.
pub(crate) fn deserialize_default_smt_pointer<'de, T, D>(deserializer: D) -> StdResult<T, D::Error>
    where
        D: Deserializer<'de>,
        T: ::serde::Deserialize<'de> + DerefMut + Default,  // could remove the Deserialize req here
        <T as Deref>::Target: Sized + ::serde::Deserialize<'de>,
        // T::Err: Display,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum Detect<SmtPointer, Val> {
        Ptr(SmtPointer),
        ValidValue(Val),
    }

    let unserialized: Detect<T, <T as Deref>::Target> = Detect::deserialize(
        deserializer
    )?;

    match unserialized {
        Detect::Ptr(p) => Ok(p),  // if it's already the desired type, then return it
        Detect::ValidValue(val) => Ok({
            let mut new_smt_point = T::default();
            *new_smt_point = val;
            new_smt_point
        }),
    }
}

/// Obtains '1' for an arbitrary number type.
pub(crate) fn one_default<T>() -> T
    where T: PrimInt + FromStr,
          <T as FromStr>::Err: ::std::fmt::Debug
{ "1".parse().unwrap() }

/// Obtains 0xffffff for an arbitrary number type.
pub(crate) fn oxffffff_default<T>() -> T
    where T: PrimInt + FromStr,
          <T as FromStr>::Err: ::std::fmt::Debug
{ "0xffffff".parse().unwrap() }
