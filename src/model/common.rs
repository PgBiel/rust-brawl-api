//! Models shared for usage by more than one endpoint. Note that, if all the relevant endpoints'
//! features are disabled, then the respective models here are also disabled.

use serde::{self, Serialize, Deserialize};

/// A struct representing a brawler's star power. Note that, if **both** `players` and `brawlers`
/// features are turned off, then this struct is also removed (it is required by both, so if neither
/// are enabled anymore, this isn't either).
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[cfg(any(feature = "players", feature = "brawlers"))]
pub struct StarPower {

    /// The star power name.
    #[serde(default)]
    pub name: String,

    /// The star power's id (an arbitrary number).
    #[serde(default)]
    pub id: usize
}

impl Default for StarPower {

    /// Returns an instance of `StarPower` with initial values.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use brawl_api::StarPower;
    ///
    /// assert_eq!(
    ///     StarPower::default(),
    ///     StarPower {
    ///         name: String::from(""),
    ///         id: 0,
    ///     }
    /// );
    /// ```
    fn default() -> StarPower {
        StarPower {
            name: String::from(""),
            id: 0
        }
    }
}
