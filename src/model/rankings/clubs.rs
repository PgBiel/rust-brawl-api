use serde::{self, Serialize, Deserialize};
use crate::traits::{PropLimRouteable, PropLimFetchable};
use crate::serde::one_default;
use std::ops::Deref;
use crate::util::fetch_route;
use crate::error::Result;

#[cfg(feature = "async")]
use async_trait::async_trait;

#[cfg(feature = "async")]
use crate::util::a_fetch_route;
use crate::http::Client;
use crate::http::routes::Route;

/// Represents a leaderboard of [`ClubRanking`]s - the top x clubs in a regional or global
/// leaderboard.
///
/// **NOTE:** The API only allows fetching up to the top 200 clubs.
///
/// [`ClubRanking`]: ./struct.ClubRanking.html
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClubLeaderboard {
    /// The clubs in the ranking.
    #[serde(default)]
    pub items: Vec<ClubRanking>,
}

impl Deref for ClubLeaderboard {
    type Target = Vec<ClubRanking>;

    fn deref(&self) -> &Vec<ClubRanking> {
        &self.items
    }
}

#[cfg_attr(feature = "async", async_trait)]
impl PropLimFetchable for ClubLeaderboard {
    type Property = str;
    type Limit = u8;

    /// (Sync) Fetches the top `limit <= 200` clubs in the regional (two-letter) `country_code`
    /// leaderboard (or global leaderboard, if `country_code == "global"`).
    ///
    /// # Examples
    ///
    /// World-wide club leaderboard:
    /// ```rust
    /// use brawl_api::{ClubLeaderboard, Client};
    ///
    /// # fn main() -> Result<(), Box<dyn ::std::error::Error>> {
    ///
    /// let client = Client::new("my auth key");
    ///
    /// // if the fetch is successful, then the variable below will have the global top 100 clubs
    /// // in the 'items' field (i.e. '*top100clubs').
    /// let top100clubs: ClubLeaderboard = ClubLeaderboard::fetch(&client, "global", 100)?;
    ///
    /// // get club ranked #1. The items are usually sorted (i.e. rank 1 on index [0], rank 2
    /// // on index [1] etc.), but, to make your program absolutely safe, might want to .sort()
    /// let club1 = &top100clubs[0];
    ///
    /// assert_eq!(club1.rank, 1);
    ///
    /// #     Ok(())
    /// # }
    /// ```
    ///
    /// Regional (in this case, zimbabwean) club leaderboard:
    /// ```rust
    /// use brawl_api::{ClubLeaderboard, Client};
    ///
    /// # fn main() -> Result<(), Box<dyn ::std::error::Error>> {
    ///
    /// let client = Client::new("my auth key");
    ///
    /// // if the fetch is successful, then the variable below will have the top 100 zimbabwean clubs
    /// // in the 'items' field (i.e. '*top100zwclubs').
    /// let top100zwclubs: ClubLeaderboard = ClubLeaderboard::fetch(&client, "ZW", 100)?;
    ///
    /// // get club ranked #1. The items are usually sorted (i.e. rank 1 on index [0], rank 2
    /// // on index [1] etc.), but, to make your program absolutely safe, might want to .sort()
    /// let club1 = &top100zwclubs[0];
    ///
    /// assert_eq!(club1.rank, 1);
    ///
    /// #     Ok(())
    /// # }
    /// ```
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
    /// [`Error::Request`]: error/enum.Error.html#variant.Request
    /// [`Error::Status`]: error/enum.Error.html#variant.Status
    /// [`Error::Ratelimited`]: error/enum.Error.html#variant.Ratelimited
    /// [`Error::Json`]: error/enum.Error.html#variant.Json
    fn fetch(client: &Client, country_code: &str, limit: u8) -> Result<ClubLeaderboard> {
        let route = ClubLeaderboard::get_route(country_code, limit);
        fetch_route::<ClubLeaderboard>(client, &route)
    }

    /// (Async) Fetches the top `limit <= 200` clubs in the regional (two-letter) `country_code`
    /// leaderboard (or global leaderboard, if `country_code == "global"`).
    ///
    /// # Examples
    ///
    /// World-wide club leaderboard:
    /// ```rust
    /// use brawl_api::{ClubLeaderboard, Client};
    ///
    /// let client = Client::new("my auth key");
    ///
    /// // if the fetch is successful, then the variable below will have the global top 100 clubs
    /// // in the 'items' field (i.e. '*top100clubs').
    /// let top100clubs: ClubLeaderboard = ClubLeaderboard::a_fetch(&client, "global", 100).await?;
    ///
    /// // get club ranked #1. The items are usually sorted (i.e. rank 1 on index [0], rank 2
    /// // on index [1] etc.), but, to make your program absolutely safe, might want to .sort()
    /// let club1 = &top100clubs[0];
    /// ```
    ///
    /// Regional (in this case, zimbabwean) club leaderboard:
    /// ```rust
    /// use brawl_api::{ClubLeaderboard, Client};
    ///
    /// # async fn main() -> Result<(), Box<dyn ::std::error::Error>> {
    ///
    /// let client = Client::new("my auth key");
    ///
    /// // if the fetch is successful, then the variable below will have the top 100 zimbabwean clubs
    /// // in the 'items' field (i.e. '*top100zwclubs').
    /// let top100zwclubs: ClubLeaderboard = ClubLeaderboard::a_fetch(&client, "ZW", 100).await?;
    ///
    /// // get club ranked #1. The items are usually sorted (i.e. rank 1 on index [0], rank 2
    /// // on index [1] etc.), but, to make your program absolutely safe, might want to .sort()
    /// let club1 = &top100zwclubs[0];
    ///
    /// assert_eq!(club1.rank, 1);
    ///
    /// #     Ok(())
    /// # }
    /// ```
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
    /// [`Error::Request`]: error/enum.Error.html#variant.Request
    /// [`Error::Status`]: error/enum.Error.html#variant.Status
    /// [`Error::Ratelimited`]: error/enum.Error.html#variant.Ratelimited
    /// [`Error::Json`]: error/enum.Error.html#variant.Json
    #[cfg(feature="async")]
    async fn a_fetch(client: &Client, country_code: &'async_trait str, limit: u8) -> Result<ClubLeaderboard>
        where Self: 'async_trait,
              Self::Property: 'async_trait,
    {
        let route = ClubLeaderboard::get_route(&country_code, limit);
        a_fetch_route::<ClubLeaderboard>(client, &route).await
    }
}

impl PropLimRouteable for ClubLeaderboard {
    type Property = str;
    type Limit = u8;

    /// Get the route for fetching the top `limit` clubs in the regional `country_code`
    /// leaderboard (or global, if `country_code == "global"`).
    fn get_route(country_code: &str, limit: u8) -> Route {
        Route::ClubRankings {
            country_code: country_code.to_owned(),
            limit
        }
    }
}

/// Represents a club's ranking, based on a regional or global leaderboard.
/// To obtain the club's full data (a [`Club`] instance), see [`Club::fetch_from`].
///
/// [`Club`]: ../clubs/club/struct.Club.html
/// [`Club::fetch_from`]: ../clubs/club/struct.Club.html#method.fetch_from
#[derive(Debug, Hash, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClubRanking {
    /// The club's tag.
    #[serde(default)]
    pub tag: String,

    /// The club's name.
    #[serde(default)]
    pub name: String,

    /// The club's current trophies.
    #[serde(default)]
    pub trophies: usize,

    /// The club's current rank in the leaderboard.
    #[serde(default = "one_default")]
    pub rank: u8,

    /// The amount of members in this club.
    #[serde(default)]
    pub member_count: usize,
}

impl Default for ClubRanking {
    fn default() -> ClubRanking {
        ClubRanking {
            tag: String::from(""),
            name: String::from(""),
            trophies: 0,
            rank: 1,
            member_count: 0,
        }
    }
}

impl PartialOrd for ClubRanking {
    /// Compares and determines which `ClubRanking` has a higher rank (i.e., smaller rank number).
    ///
    /// # Examples
    ///
    /// (**NOTE:** Club rankings are not meant to be initialized, but rather obtained from
    /// a fetched [`ClubLeaderboard`] instance. They are only instantiated here for this example.)
    ///
    /// ```rust
    /// use brawl_api::ClubRanking;
    ///
    /// let club_1 = ClubRanking { rank: 1, ..ClubRanking::default() };
    /// let club_2 = ClubRanking { rank: 5, ..ClubRanking::default() };
    ///
    /// assert!(club_1 > club_2)  // smaller rank number = higher position
    /// ```
    ///
    /// [`ClubLeaderboard`]: struct.ClubLeaderboard.html
    fn partial_cmp(&self, other: &ClubRanking) -> Option<::std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ClubRanking {  // TODO: Review whether rank 1 > rank 5 is a good design choice
    /// Compares and determines which `ClubRanking` has a higher rank (i.e., smaller rank number).
    ///
    /// # Examples
    ///
    /// (**NOTE:** Club rankings are not meant to be initialized, but rather obtained from
    /// a fetched [`ClubLeaderboard`] instance. They are only instantiated here for this example.)
    ///
    /// ```rust
    /// use brawl_api::ClubRanking;
    ///
    /// let club_1 = ClubRanking { rank: 1, ..ClubRanking::default() };
    /// let club_2 = ClubRanking { rank: 5, ..ClubRanking::default() };
    ///
    /// assert!(club_1 > club_2)  // smaller rank number = higher position
    /// ```
    ///
    /// [`ClubLeaderboard`]: struct.ClubLeaderboard.html
    fn cmp(&self, other: &ClubRanking) -> ::std::cmp::Ordering {
        self.rank.cmp(&other.rank).reverse()
    }
}