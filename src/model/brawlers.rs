//! Contains models related to the `/brawlers/...` endpoint of the Brawl Stars API.
//! Included by the feature `"brawlers"`; removing that feature will disable the usage of this
//! module.

use std::ops::{Deref, DerefMut};
use crate::traits::{FetchFrom, Refetchable};
use crate::http::routes::Route;
use crate::util::{fetch_route, a_fetch_route};
use serde::{self, Serialize, Deserialize};
use crate::error::Result;

#[cfg(feature = "async")]
use async_trait::async_trait;
use crate::http::Client;

use super::common::StarPower;

#[cfg(feature = "players")]
use super::players::{
    player::PlayerBrawlerStat,
    battlelog::BattleBrawler,
};
use crate::Brawlers;

// region:BrawlerList

/// Represents a fetchable list of all brawlers in the game, with data for each of them, such
/// as their available star powers, names etc.
///
/// Use [`BrawlerList::fetch`] to fetch all brawlers.
///
/// [`BrawlerList::fetch`]: #method.fetch
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct BrawlerList {
    /// The brawlers in the game.
    #[serde(default)]
    pub items: Vec<Brawler>
}

impl Deref for BrawlerList {
    type Target = Vec<Brawler>;

    /// Obtain the brawlers vector - dereferencing returns the [`items`] field.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use brawl_api::{Client, BrawlerList, traits::*};
    ///
    /// # fn main() -> Result<(), Box<dyn ::std::error::Error>> {
    /// let client = Client::new("my auth token");
    /// let brawlers = BrawlerList::fetch(
    ///     &client,            // <- the client containing the auth key
    /// )?;
    ///
    /// assert_eq!(brawlers.items, *brawlers);
    ///
    /// #     Ok(())
    /// # }
    ///
    /// ```
    ///
    /// [`items`]: #structfield.items
    fn deref(&self) -> &Vec<Brawler> {
        &self.items
    }
}

impl DerefMut for BrawlerList {
    /// Obtain the brawlers vector - dereferencing returns the [`items`] field.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use brawl_api::{Client, BrawlerList, traits::*};
    ///
    /// # fn main() -> Result<(), Box<dyn ::std::error::Error>> {
    /// let client = Client::new("my auth token");
    /// let brawlers = BrawlerList::fetch(
    ///     &client,            // <- the client containing the auth key
    /// )?;
    ///
    /// assert_eq!(brawlers.items, *brawlers);
    ///
    /// #     Ok(())
    /// # }
    ///
    /// ```
    ///
    /// [`items`]: #structfield.items
    fn deref_mut(&mut self) -> &mut Vec<Brawler> {
        &mut self.items
    }
}

impl BrawlerList {

    /// Returns the [`Route`] object required for fetching a `BrawlerList` instance.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use brawl_api::{BrawlerList, http::Route};
    ///
    /// assert_eq!(
    ///     BrawlerList::get_route(),
    ///     Route::Brawlers
    /// );
    /// ```
    ///
    /// [`Route`]: http/routes/struct.Route.html
    pub fn get_route() -> Route {
        Route::Brawlers
    }

    /// (Sync) Fetches data for all brawlers in the game (see [`Brawler`]). To fetch for a specific
    /// brawler, see [`Brawler::fetch`].
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
    /// ```rust,ignore
    /// use brawl_api::{Client, BrawlerList};
    ///
    /// # fn main() -> Result<(), Box<dyn ::std::error::Error>> {
    /// let my_client = Client::new("my auth token");
    /// let brawlers = BrawlerList::fetch(&my_client)?;
    /// // now a vector with data for all brawlers in the game is available for use.
    ///
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`Error::Request`]: error/enum.Error.html#variant.Request
    /// [`Error::Status`]: error/enum.Error.html#variant.Status
    /// [`Error::Ratelimited`]: error/enum.Error.html#variant.Ratelimited
    /// [`Error::Json`]: error/enum.Error.html#variant.Json
    /// [`Brawler`]: struct.Brawler.html
    /// [`Brawler::fetch`]: struct.Brawler.html#method.fetch
    pub fn fetch(client: &Client) -> Result<BrawlerList> {
        let route = BrawlerList::get_route();
        fetch_route::<BrawlerList>(client, &route)
    }

    /// (Sync) Fetches data for all brawlers in the game (see [`Brawler`]). To fetch for a specific
    /// brawler, see [`Brawler::fetch`].
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
    /// ```rust,ignore
    /// use brawl_api::{Client, BrawlerList};
    ///
    /// # async fn main() -> Result<(), Box<dyn ::std::error::Error>> {
    /// let my_client = Client::new("my auth token");
    /// let player_brawlers = BrawlerList::a_fetch(&my_client).await?;
    /// // now a vector with data for all brawlers in the game is available for use.
    ///
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`Error::Request`]: error/enum.Error.html#variant.Request
    /// [`Error::Status`]: error/enum.Error.html#variant.Status
    /// [`Error::Ratelimited`]: error/enum.Error.html#variant.Ratelimited
    /// [`Error::Json`]: error/enum.Error.html#variant.Json
    /// [`Brawler`]: struct.Brawler.html
    /// [`Brawler::fetch`]: struct.Brawler.html#method.fetch
    #[cfg(feature = "async")]
    pub async fn a_fetch(client: &Client) -> Result<BrawlerList> {
        let route = BrawlerList::get_route();
        a_fetch_route::<BrawlerList>(client, &route).await
    }
}

// endregion:BrawlerList

/// Contains information for a specific brawler, and allows for it to be fetched through
/// [`Brawler::fetch`].
///
/// [`Brawler::fetch`]: #method.fetch
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Brawler {
    /// The brawler's name, in CAPS LOCK. E.g.: `"SHELLY"` for Shelly.
    #[serde(default)]
    pub name: String,

    /// The brawler's ID (an arbitrary number representing it).
    #[serde(default)]
    pub id: usize,

    /// The brawler's star powers, as a vector (note that this does **not** have a fixed size:
    /// new brawlers start with 1 star power, while older ones have at least 2.)
    #[serde(default)]
    pub star_powers: Vec<StarPower>,
}

impl Default for Brawler {


    /// Returns an instance of `Brawler` with initial values.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use brawl_api::Brawler;
    ///
    /// assert_eq!(
    ///     Brawler::default(),
    ///     Brawler {
    ///         name: String::from(""),
    ///         id: 0,
    ///         star_powers: vec![]
    ///     }
    /// );
    /// ```
    fn default() -> Brawler {
        Brawler {
            name: String::from(""),
            id: 0,
            star_powers: vec![]
        }
    }
}

impl Brawler {
    /// Returns this Brawler's ID, which is used for fetching.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use brawl_api::{Brawler, traits::*};
    ///
    /// // given an existing Brawler object
    /// let brawler: Brawler;
    /// # brawler = Brawler::default();
    ///
    /// assert_eq!(brawler.get_fetch_prop(), brawler.id);
    /// ```
    pub fn get_fetch_prop(&self) -> usize { self.id }

    /// Returns the [`Route`] object required for fetching a `Brawler` instance.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use brawl_api::{BrawlerList, http::Route};
    ///
    /// assert_eq!(
    ///     BrawlerList::get_route(),
    ///     Route::Brawlers
    /// );
    /// ```
    ///
    /// [`Route`]: http/routes/struct.Route.html
    pub fn get_route(id: usize) -> Route { Route::Brawler(id) }

    /// (Sync) Fetches data for a brawler with a specific ID (see the [`Brawlers`] enum for a
    /// humanized list with IDs).
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
    /// ```rust,ignore
    /// use brawl_api::{Client, Brawler, Brawlers, traits::*};
    ///
    /// # fn main() -> Result<(), Box<dyn ::std::error::Error>> {
    /// let my_client = Client::new("my auth token");
    /// let shelly = Brawler::fetch(&my_client, Brawlers::Shelly as usize)?;
    /// // now the data for Shelly is available.
    ///
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`Error::Request`]: error/enum.Error.html#variant.Request
    /// [`Error::Status`]: error/enum.Error.html#variant.Status
    /// [`Error::Ratelimited`]: error/enum.Error.html#variant.Ratelimited
    /// [`Error::Json`]: error/enum.Error.html#variant.Json
    pub fn fetch(client: &Client, id: usize) -> Result<Brawler> {
        let route = Brawler::get_route(id);
        fetch_route::<Brawler>(client, &route)
    }

    /// (Async) Fetches data for a brawler with a specific ID (see the [`Brawlers`] enum for a
    /// humanized list with IDs).
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
    /// ```rust,ignore
    /// use brawl_api::{Client, Brawler, Brawlers, traits::*};
    ///
    /// # async fn main() -> Result<(), Box<dyn ::std::error::Error>> {
    /// let my_client = Client::new("my auth token");
    /// let shelly = Brawler::a_fetch(&my_client, Brawlers::Shelly as usize).await?;
    /// // now the data for Shelly is available.
    ///
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`Error::Request`]: error/enum.Error.html#variant.Request
    /// [`Error::Status`]: error/enum.Error.html#variant.Status
    /// [`Error::Ratelimited`]: error/enum.Error.html#variant.Ratelimited
    /// [`Error::Json`]: error/enum.Error.html#variant.Json
    #[cfg(feature="async")]
    pub async fn a_fetch(client: &Client, id: usize) -> Result<Brawler> {
        let route = Brawler::get_route(id);
        a_fetch_route::<Brawler>(client, &route).await
    }
}

#[cfg_attr(feature = "async", async_trait)]
impl Refetchable for Brawler {
    /// (Sync) Fetches data for this brawler again.
    fn refetch(&self, client: &Client) -> Result<Brawler> {
        Brawler::fetch(client, self.id)
    }

    /// (Async) Fetches data for this brawler again.
    #[cfg(feature = "async")]
    async fn a_refetch(&self, client: &Client) -> Result<Brawler> {
        Brawler::a_fetch(client, self.id).await
    }
}

#[cfg_attr(feature = "async", async_trait)]
#[cfg(feature = "players")]
impl FetchFrom<PlayerBrawlerStat> for Brawler {
    /// (Sync) Attempts to fetch a `Brawler` from an existing [`PlayerBrawlerStat`] instance.
    ///
    /// [`PlayerBrawlerStat`]: ../players/player/struct.PlayerBrawlerStat.html
    fn fetch_from(client: &Client, p_brawler: &PlayerBrawlerStat) -> Result<Brawler> {
        Brawler::fetch(client, p_brawler.id)
    }

    /// (Async) Attempts to fetch a `Brawler` from an existing [`PlayerBrawlerStat`] instance.
    ///
    /// [`PlayerBrawlerStat`]: ../players/player/struct.PlayerBrawlerStat.html
    #[cfg(feature = "async")]
    async fn a_fetch_from(client: &Client, p_brawler: &PlayerBrawlerStat) -> Result<Brawler> {
        Brawler::a_fetch(client, p_brawler.id).await
    }
}

#[cfg_attr(feature = "async", async_trait)]
#[cfg(feature = "players")]
impl FetchFrom<BattleBrawler> for Brawler {
    /// (Sync) Attempts to fetch a `Brawler` from an existing [`BattleBrawler`] instance.
    ///
    /// [`BattleBrawler`]: ../players/battlelog/struct.BattleBrawler.html
    fn fetch_from(client: &Client, b_brawler: &BattleBrawler) -> Result<Brawler> {
        Brawler::fetch(client, b_brawler.id)
    }

    /// (Async) Attempts to fetch a `Brawler` from an existing [`BattleBrawler`] instance.
    ///
    /// [`BattleBrawler`]: ../players/battlelog/struct.BattleBrawler.html
    #[cfg(feature = "async")]
    async fn a_fetch_from(client: &Client, b_brawler: &BattleBrawler) -> Result<Brawler> {
        Brawler::a_fetch(client, b_brawler.id).await
    }
}

#[cfg_attr(feature = "async", async_trait)]
impl FetchFrom<Brawlers> for Brawler {
    /// (Sync) Attempts to fetch a `Brawler` from an existing [`Brawlers`] variant.
    ///
    /// [`Brawlers`]: ../constants/enum.Brawlers.html
    fn fetch_from(client: &Client, b_brawler: &Brawlers) -> Result<Brawler> {
        Brawler::fetch(client, b_brawler.to_owned() as usize)
    }

    /// (Async) Attempts to fetch a `Brawler` from an existing [`Brawlers`] variant.
    ///
    /// [`Brawlers`]: ../constants/enum.Brawlers.html
    #[cfg(feature = "async")]
    async fn a_fetch_from(client: &Client, b_brawler: &Brawlers) -> Result<Brawler> {
        Brawler::a_fetch(client, b_brawler.to_owned() as usize).await
    }
}
