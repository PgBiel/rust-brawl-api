//! Contains models for the `/rankings/:country_code/brawlers/:brawler_id?limit=x` Brawl Stars API
//! endpoint.
//! Included by the feature `"rankings"`; removing that feature will disable the usage of this module.

use serde::{self, Serialize, Deserialize};

use std::ops::Deref;
use crate::util::fetch_route;
use crate::error::Result;

#[cfg(feature = "async")]
use crate::util::a_fetch_route;

#[cfg(feature = "async")]


use crate::http::Client;
use crate::http::routes::Route;
use super::players::PlayerRanking;


/// Represents a leaderboard of [`PlayerRanking`]s - the top x players in a regional or global
/// leaderboard, **considering their trophies with a specific brawler**.
///
/// **NOTE:** The API only allows fetching up to the top 200 players with any brawler.
///
/// Also, this follows the Smart Pointer model, meaning that vector methods are accepted thanks
/// to the `Deref` trait.
///
/// See [`BrawlerLeaderboard::fetch`] for fetching this model from the API. (There's
/// also [`BrawlerLeaderboard::a_fetch`] for async.)
///
/// [`PlayerRanking`]: ../players/struct.PlayerRanking.html
/// [`BrawlerLeaderboard::fetch`]: #method.fetch
/// [`BrawlerLeaderboard::a_fetch`]: #method.a_fetch
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct BrawlerLeaderboard {
    /// The players in the brawler ranking.
    ///
    /// **NOTE:** When the targeted brawler is a recent/new one, this vector *could* be empty, even
    /// if for a very short period (global/US leaderboard), or, for country codes containing less
    /// active players, it may not appear for a while (a few days?).
    #[serde(default)]
    pub items: Vec<PlayerRanking>,
}

impl Deref for BrawlerLeaderboard {
    type Target = Vec<PlayerRanking>;

    /// Obtain the players in the ranking - dereferencing returns the [`items`] field.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use brawl_api::{Client, BrawlerLeaderboard, Brawlers, traits::*};
    ///
    /// # fn main() -> Result<(), Box<dyn ::std::error::Error>> {
    /// let client = Client::new("my auth token");
    /// let top50colts = BrawlerLeaderboard::fetch(
    ///     &client,   // <- the client containing the auth key
    ///     "global",  // <- the region of the leaderboard to fetch ("global" - world-wide)
    ///     Brawlers::Colt as usize,  // <- whose brawler should this leaderboard be
    ///     50         // <- limit of rankings to fetch (i.e. top 50)
    /// )?;
    ///
    /// assert_eq!(top50colts.items, *top50colts);
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

impl BrawlerLeaderboard {
    /// (Sync) Fetches the top `limit <= 200` players in the regional (two-letter) `country_code`
    /// leaderboard (or global leaderboard, if `country_code == "global"`), **sorted by their
    /// trophies with the brawler represented by `brawler_id`.** (Tip: use the [`Brawler`] enum
    /// to easily obtain ID values, although it requires being updated, so fetching from the API
    /// might be more appropriate.)
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
    /// World-wide Tara leaderboard:
    /// ```rust,ignore
    /// use brawl_api::{
    ///     BrawlerLeaderboard, Client, Brawlers, PlayerRanking,
    /// };
    ///
    /// # fn main() -> Result<(), Box<dyn ::std::error::Error>> {
    /// let client = Client::new("my auth key");
    ///
    /// // if the fetch is successful, then the variable below will have the global top 100 Tara
    /// // players in the 'items' field (i.e. '*top100taras').
    /// let top100taras: BrawlerLeaderboard = BrawlerLeaderboard::fetch(
    ///     &client, "global", Brawlers::Tara as usize, 100
    /// )?;
    ///
    /// // get player ranked #1. The items are usually sorted (i.e. rank 1 on index [0], rank 2
    /// // on index [1] etc.), but, to make your program absolutely safe, might want to .sort()
    /// let tara1: &PlayerRanking = &top100taras[0];
    ///
    /// assert_eq!(tara1.rank, 1);
    ///
    /// #     Ok(())
    /// # }
    /// ```
    ///
    /// Regional (in this case, zimbabwean) Shelly leaderboard:
    /// ```rust,ignore
    /// use brawl_api::{
    ///     BrawlerLeaderboard, Client, Brawlers, PlayerRanking,
    /// };
    ///
    /// # fn main() -> Result<(), Box<dyn ::std::error::Error>> {
    /// let client = Client::new("my auth key");
    ///
    /// // if the fetch is successful, then the variable below will have the global top 150
    /// // zimbabwean Shelly players in the 'items' field (i.e. '*top150_zw_shelly').
    /// let top150_zw_shelly: BrawlerLeaderboard = BrawlerLeaderboard::fetch(
    ///     &client, "ZW", Brawlers::Shelly as usize, 150
    /// )?;
    ///
    /// // get player ranked #1. The items are usually sorted (i.e. rank 1 on index [0], rank 2
    /// // on index [1] etc.), but, to make your program absolutely safe, might want to .sort()
    /// let shelly1_zw: &PlayerRanking = &top150_zw_shelly[0];
    ///
    /// assert_eq!(shelly1_zw.rank, 1);
    ///
    /// #     Ok(())
    /// # }
    /// ```
    ///
    /// [`Error::Request`]: error/enum.Error.html#variant.Request
    /// [`Error::Status`]: error/enum.Error.html#variant.Status
    /// [`Error::Ratelimited`]: error/enum.Error.html#variant.Ratelimited
    /// [`Error::Json`]: error/enum.Error.html#variant.Json
    pub fn fetch(
        client: &Client, country_code: &str, brawler_id: usize, limit: u8,
    ) -> Result<BrawlerLeaderboard> {
        let route = Route::BrawlerRankings {
            country_code: country_code.to_owned(),
            brawler_id,
            limit
        };
        fetch_route::<BrawlerLeaderboard>(client, &route)
    }

    /// (Async) Fetches the top `limit <= 200` players in the regional (two-letter) `country_code`
    /// leaderboard (or global leaderboard, if `country_code == "global"`), **sorted by their
    /// trophies with the brawler represented by `brawler_id`.** (Tip: use the [`Brawler`] enum
    /// to easily obtain ID values, although it requires being updated, so fetching from the API
    /// might be more appropriate.)
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
    /// World-wide Tara leaderboard:
    /// ```rust,ignore
    /// use brawl_api::{
    ///     BrawlerLeaderboard, Client, Brawlers, PlayerRanking,
    /// };
    ///
    /// # async fn main() -> Result<(), Box<dyn ::std::error::Error>> {
    /// let client = Client::new("my auth key");
    ///
    /// // if the fetch is successful, then the variable below will have the global top 100 Tara
    /// // players in the 'items' field (i.e. '*top100taras').
    /// let top100taras: BrawlerLeaderboard = BrawlerLeaderboard::a_fetch(
    ///     &client, "global", Brawlers::Tara as usize, 100
    /// ).await?;
    ///
    /// // get player ranked #1. The items are usually sorted (i.e. rank 1 on index [0], rank 2
    /// // on index [1] etc.), but, to make your program absolutely safe, might want to .sort()
    /// let tara1: &PlayerRanking = &top100taras[0];
    ///
    /// assert_eq!(tara1.rank, 1);
    ///
    /// #     Ok(())
    /// # }
    /// ```
    ///
    /// Regional (in this case, zimbabwean) Shelly leaderboard:
    /// ```rust,ignore
    /// use brawl_api::{
    ///     BrawlerLeaderboard, Client, Brawlers, PlayerRanking,
    /// };
    ///
    /// # async fn main() -> Result<(), Box<dyn ::std::error::Error>> {
    /// let client = Client::new("my auth key");
    ///
    /// // if the fetch is successful, then the variable below will have the global top 150
    /// // zimbabwean Shelly players in the 'items' field (i.e. '*top150_zw_shelly').
    /// let top150_zw_shelly: BrawlerLeaderboard = BrawlerLeaderboard::a_fetch(
    ///     &client, "ZW", Brawlers::Shelly as usize, 150
    /// ).await?;
    ///
    /// // get player ranked #1. The items are usually sorted (i.e. rank 1 on index [0], rank 2
    /// // on index [1] etc.), but, to make your program absolutely safe, might want to .sort()
    /// let shelly1_zw: &PlayerRanking = &top150_zw_shelly[0];
    ///
    /// assert_eq!(shelly1_zw.rank, 1);
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
    pub async fn a_fetch(
        client: &Client, country_code: &str, brawler_id: usize, limit: u8,
    ) -> Result<BrawlerLeaderboard> {
        let route = Route::BrawlerRankings {
            country_code: country_code.to_owned(),
            brawler_id,
            limit
        };
        a_fetch_route::<BrawlerLeaderboard>(client, &route).await
    }
}
