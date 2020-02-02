//! Contains the [`TimeLike`] struct, used to indicate strings that contain timestamps which can
//! be parsed using [`TimeLike.parse`] (if the `chrono` feature is enabled, otherwise the method
//! is not implemented).

use serde::{self, Serialize, Deserialize};
use std::fmt::Display;

/// Represents a timestamp provided by the Brawl API. If the `chrono` feature is enabled (it is
/// by default), then it is possible to use helper methods to convert it to `chrono` data
/// structures - see [`TimeLike.parse`] (this is recommended, as it is aware of the correct
/// format).
///
/// [`TimeLike.parse`]: #method.parse
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct TimeLike(pub(crate) String);

impl Default for TimeLike {
    /// Returns an initial `TimeLike` instance containing an empty string.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use brawl_api::TimeLike;
    ///
    /// assert_eq!(
    ///     TimeLike::default().to_string(),
    ///     "",
    /// )
    /// ```
    fn default() -> TimeLike {
        TimeLike(String::from(""))
    }
}

impl Display for TimeLike {
    /// Displays the inner string of the `TimeLike` instance.
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TimeLike {
    /// Returns an immutable reference to the inner string.
    pub fn inner(&self) -> &String {
        &self.0
    }

    /// Returns a mutable reference to the inner string.
    pub fn inner_mut(&mut self) -> &mut String {
        &mut self.0
    }
}

#[cfg(feature = "chrono")]
pub mod _impls {  // public so its implementations can be accessed
    use crate::time::TimeLike;
    use crate::error::{Result, Error};
    use chrono::prelude::*;
    use crate::constants::TIMELIKE_FORMAT;

    impl TimeLike {
        /// Parses this timestamp into a [`chrono::DateTime<chrono::Utc>`], using the API's
        /// format (see [`constants::TIMELIKE_FORMAT`]).
        ///
        /// # Errors
        ///
        /// If the string is invalid, an [`Error::ParseTimeLike`] is returned.
        /// Generally, when requesting from the API, this shouldn't ever happen.
        ///
        /// # Examples
        ///
        /// ```rust
        /// use brawl_api::prelude::*;
        /// use brawl_api::{TimeLike, Battle};
        /// use chrono::{DateTime, Utc};
        ///
        ///
        /// // after obtaining a fetched battlelog's battle
        /// let battle: Battle;
        /// # battle = Battle::default();
        ///
        /// let mut the_time: TimeLike = battle.battle_time;
        /// # let mut _the_str = the_time.inner_mut();
        /// # *_the_str = String::from("20200129T042143.000Z");
        /// let parsed_time: DateTime<Utc> = the_time.parse()?;
        /// // the parsed time of the battle is now available for use.
        ///
        /// # Ok::<(), Box<dyn ::std::error::Error>>(())
        /// ```
        ///
        /// [`chrono::DateTime<chrono::Utc>`]: https://docs.rs/chrono/*/chrono/struct.DateTime.html
        /// [`chrono::ParseError`]: https://docs.rs/chrono/*/chrono/format/struct.ParseError.html
        /// [`Error::ParseTimeLike`]: ../error/enum.Error.html#variant.ParseTimeLike
        /// [`constants::TIMELIKE_FORMAT`]: ../constants/constant.TIMELIKE_FORMAT.html
        pub fn parse(&self) -> Result<DateTime<Utc>> {
            Utc.datetime_from_str(
                &self.0, TIMELIKE_FORMAT
            ).map_err(|e| {
                Error::ParseTimeLike {
                    reason: e.to_string(),
                    offender: Some(self.0.clone()),
                    original_err: Some(e)
                }
            })
        }
    }
}

#[cfg(feature = "datetime")]
pub use _impls::*;

///////////////////////////////////   tests   ///////////////////////////////////

#[cfg(test)]
mod tests {
    use super::TimeLike;

    /// Tests TimeLike to DateTime<Utc> conversion.
    #[test]
    #[cfg(feature = "chrono")]
    fn timelike_to_datetime_convert() -> Result<(), Box<dyn ::std::error::Error>> {
        use chrono::prelude::*;
        let time_str = "20200129T042143.000Z";
        let time = TimeLike(String::from(time_str));

        let dt: DateTime<Utc> = Utc
            .ymd(2020, 01, 29).and_hms(04, 21, 43);

        assert_eq!(time.parse()?, dt);

        Ok(())
    }
}
