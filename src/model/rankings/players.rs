//! Contains models for the `/rankings/:country_code/players?limit=x` Brawl Stars API endpoint.
//! Included by the feature `"rankings"`; removing that feature will disable the usage of this module.

use serde::{self, Serialize, Deserialize};
use crate::traits::{PropLimRouteable, PropLimFetchable};
use crate::serde::{one_default, oxffffff_default};
use std::ops::Deref;
use crate::util::fetch_route;
use crate::error::Result;

#[cfg(feature = "async")]
use async_trait::async_trait;

#[cfg(feature = "async")]
use crate::util::a_fetch_route;
use crate::http::Client;
use crate::http::routes::Route;

/// Represents a leaderboard of [`PlayerRanking`]s - the top x players in a regional or global
/// leaderboard, sorted by total trophies.
///
/// **NOTE:** The API only allows fetching up to the top 200 players.
///
/// [`PlayerRanking`]: ./struct.PlayerRanking.html
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlayerLeaderboard {
    /// The players in the ranking.
    #[serde(default)]
    pub items: Vec<PlayerRanking>,
}

impl Deref for PlayerLeaderboard {
    type Target = Vec<PlayerRanking>;

    /// Obtain the players in the ranking - dereferencing returns the [`items`] field.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use brawl_api::{Client, PlayerLeaderboard, traits::*};
    ///
    /// # fn main() -> Result<(), Box<dyn ::std::error::Error>> {
    /// let client = Client::new("my auth token");
    /// let top50players = PlayerLeaderboard::fetch(
    ///     &client,   // <- the client containing the auth key
    ///     "global",  // <- the region of the leaderboard to fetch ("global" - world-wide)
    ///     50         // <- limit of rankings to fetch (i.e. top 50)
    /// )?;
    ///
    /// assert_eq!(top50players.items, *top50players);
    ///
    /// #     Ok(())
    /// # }
    ///
    /// ```
    ///
    /// [`items`]: #structfield.items
    fn deref(&self) -> &Vec<PlayerRanking> {
        &self.items
    }
}

#[cfg_attr(feature = "async", async_trait)]
impl PropLimFetchable for PlayerLeaderboard {
    type Property = str;
    type Limit = u8;

    /// (Sync) Fetches the top `limit` players in the regional (two-letter) `country_code`
    /// leaderboard (or global leaderboard, if `country_code == "global"`).
    ///
    /// # Errors
    ///
    /// This function may error:
    /// - While requesting (will return an [`Error::Request`]);
    /// - After receiving a bad status code (API or other error - returns an [`Error::Status`]);
    /// - After a ratelimit is indicated by the API, while also specifying when it is lifted ([`Error::Ratelimited`]);
    /// - While parsing incoming JSON (will return an [`Error::Json`]).
    ///
    /// (All of those, of course, wrapped inside an `Err`.)
    ///
    /// # Examples
    ///
    /// World-wide player leaderboard:
    /// ```rust,ignore
    /// use brawl_api::{PlayerLeaderboard, Client, traits::PropLimFetchable};
    ///
    /// # fn main() -> Result<(), Box<dyn ::std::error::Error>> {
    /// let client = Client::new("my auth key");
    ///
    /// // if the fetch is successful, then the variable below will have the global top 100 players
    /// // in the 'items' field (i.e. '*top100players').
    /// let top100players: PlayerLeaderboard = PlayerLeaderboard::fetch(&client, "global", 100)?;
    ///
    /// // get player ranked #1. The items are usually sorted (i.e. rank 1 on index [0], rank 2
    /// // on index [1] etc.), but, to make your program absolutely safe, might want to .sort()
    /// let player1 = &top100players[0];
    ///
    /// assert_eq!(player1.rank, 1);
    ///
    /// #     Ok(())
    /// # }
    /// ```
    ///
    /// Regional (in this case, zimbabwean) player leaderboard:
    /// ```rust,ignore
    /// use brawl_api::{PlayerLeaderboard, Client, traits::PropLimFetchable};
    ///
    /// # fn main() -> Result<(), Box<dyn ::std::error::Error>> {
    /// let client = Client::new("my auth key");
    ///
    /// // if the fetch is successful, then the variable below will have the top 100 zimbabwean
    /// // players in the 'items' field (i.e. '*top100zwplayers').
    /// let top100zwplayers: PlayerLeaderboard = PlayerLeaderboard::fetch(&client, "ZW", 100)?;
    ///
    /// // get player ranked #1. The items are usually sorted (i.e. rank 1 on index [0], rank 2
    /// // on index [1] etc.), but, to make your program absolutely safe, might want to .sort()
    /// let player1 = &top100zwplayers[0];
    ///
    /// assert_eq!(player1.rank, 1);
    ///
    /// #     Ok(())
    /// # }
    /// ```
    ///
    /// [`Error::Request`]: error/enum.Error.html#variant.Request
    /// [`Error::Status`]: error/enum.Error.html#variant.Status
    /// [`Error::Ratelimited`]: error/enum.Error.html#variant.Ratelimited
    /// [`Error::Json`]: error/enum.Error.html#variant.Json
    fn fetch(client: &Client, country_code: &str, limit: u8) -> Result<PlayerLeaderboard> {
        let route = PlayerLeaderboard::get_route(country_code, limit);
        fetch_route::<PlayerLeaderboard>(client, &route)
    }

    /// (Async) Fetches the top `limit` players in the regional (two-letter) `country_code`
    /// leaderboard (or global leaderboard, if `country_code == "global"`).
    ///
    /// # Errors
    ///
    /// This function may error:
    /// - While requesting (will return an [`Error::Request`]);
    /// - After receiving a bad status code (API or other error - returns an [`Error::Status`]);
    /// - After a ratelimit is indicated by the API, while also specifying when it is lifted ([`Error::Ratelimited`]);
    /// - While parsing incoming JSON (will return an [`Error::Json`]).
    ///
    /// (All of those, of course, wrapped inside an `Err`.)
    ///
    /// # Examples
    ///
    /// World-wide player leaderboard:
    /// ```rust,ignore
    /// use brawl_api::{PlayerLeaderboard, Client, traits::PropLimFetchable};
    ///
    /// # fn main() -> Result<(), Box<dyn ::std::error::Error>> {
    /// let client = Client::new("my auth key");
    ///
    /// // if the fetch is successful, then the variable below will have the global top 100 players
    /// // in the 'items' field (i.e. '*top100players').
    /// let top100players: PlayerLeaderboard = PlayerLeaderboard::a_fetch(&client, "global", 100).await?;
    ///
    /// // get player ranked #1. The items are usually sorted (i.e. rank 1 on index [0], rank 2
    /// // on index [1] etc.), but, to make your program absolutely safe, might want to .sort()
    /// let player1 = &top100players[0];
    ///
    /// assert_eq!(player1.rank, 1);
    ///
    /// #     Ok(())
    /// # }
    /// ```
    ///
    /// Regional (in this case, zimbabwean) player leaderboard:
    /// ```rust,ignore
    /// use brawl_api::{PlayerLeaderboard, Client, traits::PropLimFetchable};
    ///
    /// # async fn main() -> Result<(), Box<dyn ::std::error::Error>> {
    /// let client = Client::new("my auth key");
    ///
    /// // if the fetch is successful, then the variable below will have the top 100 zimbabwean
    /// // players in the 'items' field (i.e. '*top100zwplayers').
    /// let top100zwplayers: PlayerLeaderboard = PlayerLeaderboard::a_fetch(&client, "ZW", 100).await?;
    ///
    /// // get player ranked #1. The items are usually sorted (i.e. rank 1 on index [0], rank 2
    /// // on index [1] etc.), but, to make your program absolutely safe, might want to .sort()
    /// let player1 = &top100zwplayers[0];
    ///
    /// assert_eq!(player1.rank, 1);
    ///
    /// #     Ok(())
    /// # }
    /// ```
    ///
    /// [`Error::Request`]: error/enum.Error.html#variant.Request
    /// [`Error::Status`]: error/enum.Error.html#variant.Status
    /// [`Error::Ratelimited`]: error/enum.Error.html#variant.Ratelimited
    /// [`Error::Json`]: error/enum.Error.html#variant.Json
    #[cfg(feature="async")]
    async fn a_fetch(
        client: &Client, country_code: &'async_trait str, limit: u8
    ) -> Result<PlayerLeaderboard>
        where Self: 'async_trait,
              Self::Property: 'async_trait,
    {
        let route = PlayerLeaderboard::get_route(&country_code, limit);
        a_fetch_route::<PlayerLeaderboard>(client, &route).await
    }
}

impl PropLimRouteable for PlayerLeaderboard {
    type Property = str;
    type Limit = u8;

    /// Get the route for fetching the top `limit` players in the regional `country_code`
    /// leaderboard (or global, if `country_code == "global"`).
    fn get_route(country_code: &str, limit: u8) -> Route {
        Route::PlayerRankings {
            country_code: country_code.to_owned(),
            limit
        }
    }
}

/// Represents a player's ranking, based on a regional or global leaderboard.
/// To obtain the player's full data (a [`Player`] instance), see [`Player::fetch_from`].
///
/// [`Player`]: ../players/player/struct.Player.html
/// [`Player::fetch_from`]: ../players/player/struct.Player.html#method.fetch_from
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerRanking {
    /// The club the player is in - at the moment, only its name is available for this data model.
    /// To convert to a full [`Club`] object, see the [`PlayerRankingClub`] docs.
    ///
    /// [`Player`]: ../../players/struct.Player.html
    /// [`Player::fetch_from`]: ../../players/struct.Player.html#method.fetch_from
    #[serde(default)]
    pub club: PlayerRankingClub,

    /// The player's tag.
    #[serde(default)]
    pub tag: String,

    /// The player's name.
    #[serde(default)]
    pub name: String,

    /// The player's trophies.
    #[serde(default)]
    pub trophies: usize,

    /// The player's rank in the leaderboard.
    #[serde(default = "one_default")]
    pub rank: u8,

    /// The player's name color. Defaults to `0xffffff` (white).
    #[serde(default = "oxffffff_default")]
    pub name_color: usize,
}

/// Represents the club in a player's ranking (a [`PlayerRanking`] object). Since the only data
/// available at the moment is its name, it cannot be converted into a full [`Club`] object
/// using a convenient method. For that, you must have the original `PlayerRanking` object,
/// then convert it into a [`Player`] with [`Player::fetch_from`].
///
/// [`PlayerRanking`]: ./struct.PlayerRanking.html
/// [`Player`]: ../../players/player/struct.Player.html
/// [`Player::fetch_from`]: ../../players/player/struct.Player.html#method.fetch_from
#[non_exhaustive]
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlayerRankingClub {
    /// The club's name.
    #[serde(default)]
    pub name: String,
}

impl PlayerRankingClub {
    /// Creates a new `PlayerRankingClub` instance with the given name.
    pub fn new(name: &str) -> PlayerRankingClub {
        PlayerRankingClub { name: name.to_owned() }
    }
}

impl Default for PlayerRankingClub {
    /// Returns an instance of `PlayerRankingClub` with initial values (empty name).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use brawl_api::model::PlayerRankingClub;
    ///
    /// assert_eq!(
    ///     PlayerRankingClub::default(),
    ///     PlayerRankingClub::new("")
    /// );
    /// ```
    fn default() -> PlayerRankingClub {
        PlayerRankingClub {
            name: String::from(""),
        }
    }
}


