//! Traits used by the library. Those are of mostly internal use and normally shouldn't need
//! implementation by some user-made type.
//!
//! Note that it is recommended to always import this module for things to work fine, such as
//! using `X::fetch` methods, where `X` is any model implementing `PropFetchable` or
//! `PropLimFetchable` (there are some that do not implement either, and rather have their own
//! implementation of a `fetch` function, because they have 3 or more parameters).

use crate::error::{Result};

#[cfg(feature = "async")]
use async_trait::async_trait;
use crate::http::Client;
// use serde::de::DeserializeOwned;

use crate::http::routes::Route;

pub mod propfetch {
    use super::*;

    /// A trait representing a type with a property used to be fetched, and a route with which to
    /// fetch it. This can be either a tag or the limit of fetching, for example.
    ///
    /// The property must be returned by the `get_fetch_prop` function.
    /// The route must be returned by the `get_route` function.
    /// This trait is used in parallel with `PropFetchable`.
    ///
    /// [`PropFetchable`]: traits/propfetch/trait.PropFetchable.html
    /// [`GetFetchProp`]: traits/propfetch/trait.GetFetchProp.html
    pub trait GetFetchProp: Sized {
        type Property: ?Sized;

        /// Obtain the revelant property for fetching.
        fn get_fetch_prop(&self) -> &Self::Property;
        // necessary for Refetchable blanket impl

        /// Obtain the route for fetching by using a property.
        fn get_route(prop: &Self::Property) -> Route;
    }

    /// A trait representing a weaker variant of [`GetFetchProp`]; only indicates the fetching route can
    /// be obtained by the `get_route` method by specifying a property, not that such property is
    /// obtainable. Note that all `GetFetchProp` implementers also implement `PropRouteable`, thanks
    /// to a blanket impl.
    pub trait PropRouteable: Sized {
        type Property: ?Sized;

        /// Obtain the route for fetching by using a property.
        fn get_route(prop: &Self::Property) -> Route;
    }

    impl<T: GetFetchProp> PropRouteable for T {
        type Property = <T as GetFetchProp>::Property;

        fn get_route(prop: &<Self as PropRouteable>::Property) -> Route {
            <Self as GetFetchProp>::get_route(prop)
        }
    }

    /// A trait representing a type whose instance can be fetched from the API using some property.
    /// This is usually the object's tag.
    #[cfg_attr(feature = "async", async_trait)]
    pub trait PropFetchable: Sized {
        type Property: ?Sized;

        /// (Sync) Fetch and construct a new instance of this type.
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
        /// Fetching a [`Player`] instance from a player tag:
        ///
        /// ```rust,ignore
        /// use brawl_api::{Client, Player, traits::*};
        ///
        /// # fn main() -> Result<(), Box<dyn ::std::error::Error>> {
        /// let my_client = Client::new("my auth token");
        /// let player = Player::fetch(&my_client, "#PLAYERTAGHERE")?;
        /// // now the data for the given player is available for use
        ///
        /// #     Ok(())
        /// # }
        /// ```
        ///
        /// [`Player`]: model/players/player/struct.Player.html
        /// [`Error::Request`]: error/enum.Error.html#variant.Request
        /// [`Error::Status`]: error/enum.Error.html#variant.Status
        /// [`Error::Ratelimited`]: error/enum.Error.html#variant.Ratelimited
        /// [`Error::Json`]: error/enum.Error.html#variant.Json
        fn fetch(client: &Client, prop: &Self::Property) -> Result<Self>;

        /// (Async) Fetch and construct a new instance of this type.
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
        /// Fetching a [`Player`] instance from a player tag:
        ///
        /// ```rust,ignore
        /// use brawl_api::{Client, Player, traits::*};
        ///
        /// # async fn main() -> Result<(), Box<dyn ::std::error::Error>> {
        /// let my_client = Client::new("my auth token");
        /// let player = Player::a_fetch(&my_client, "#PLAYERTAGHERE").await?;
        /// // now the data for the given player is available for use
        ///
        /// #     Ok(())
        /// # }
        /// ```
        ///
        /// [`Player`]: model/players/player/struct.Player.html
        /// [`Error::Request`]: error/enum.Error.html#variant.Request
        /// [`Error::Status`]: error/enum.Error.html#variant.Status
        /// [`Error::Ratelimited`]: error/enum.Error.html#variant.Ratelimited
        /// [`Error::Json`]: error/enum.Error.html#variant.Json
        #[cfg(feature = "async")]
        async fn a_fetch(client: &Client, prop: &'async_trait Self::Property) -> Result<Self>
            where Self: 'async_trait,
                  Self::Property: 'async_trait;
    }
}

pub use propfetch::*;

pub mod proplimfetch {
    use super::*;
    use num_traits::PrimInt;

    /// A trait representing a type that returns a [`Route`] object given a property and a limit
    /// of how many to fetch.
    pub trait PropLimRouteable {
        type Property: ?Sized;
        type Limit: PrimInt;  // must be numeric!

        /// Obtain the route for fetching by using a property and specifying the limit of fetching.
        fn get_route(prop: &Self::Property, limit: Self::Limit) -> Route;
    }


    /// A trait representing a type whose instance can be fetched from the API using some property
    /// and specifying a limit of how many objects to fetch.
    ///
    /// **Note:** types which simply require the limit for fetching use [`PropFetchable`] instead
    /// (with the limit being the property itself).
    #[cfg_attr(feature = "async", async_trait)]
    pub trait PropLimFetchable: Sized {
        type Property: ?Sized;
        type Limit: PrimInt;  // numeric

        /// (Sync) Fetch and construct a new instance of this type.
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
        /// Fetching a world-wide player leaderboard ([`PlayerLeaderboard`]):
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
        /// // on index [1] etc.), but, to make the program absolutely safe, might want to .sort()
        /// let player1 = &top100players[0];
        ///
        /// assert_eq!(player1.rank, 1);
        ///
        /// #     Ok(())
        /// # }
        /// ```
        ///
        /// [`PlayerLeaderboard`]: model/rankings/players/struct.PlayerLeaderboard.html
        /// [`Error::Request`]: error/enum.Error.html#variant.Request
        /// [`Error::Status`]: error/enum.Error.html#variant.Status
        /// [`Error::Ratelimited`]: error/enum.Error.html#variant.Ratelimited
        /// [`Error::Json`]: error/enum.Error.html#variant.Json
        fn fetch(client: &Client, prop: &Self::Property, limit: Self::Limit) -> Result<Self>;

        /// (Async) Fetch and construct a new instance of this type.
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
        /// Fetching a world-wide player leaderboard ([`PlayerLeaderboard`]):
        /// ```rust,ignore
        /// use brawl_api::{PlayerLeaderboard, Client, traits::PropLimFetchable};
        ///
        /// # async fn main() -> Result<(), Box<dyn ::std::error::Error>> {
        /// let client = Client::new("my auth key");
        ///
        /// // if the fetch is successful, then the variable below will have the global top 100 players
        /// // in the 'items' field (i.e. '*top100players').
        /// let top100players: PlayerLeaderboard = PlayerLeaderboard::a_fetch(&client, "global", 100).await?;
        ///
        /// // get player ranked #1. The items are usually sorted (i.e. rank 1 on index [0], rank 2
        /// // on index [1] etc.), but, to make the program absolutely safe, might want to .sort()
        /// let player1 = &top100players[0];
        ///
        /// assert_eq!(player1.rank, 1);
        ///
        /// #     Ok(())
        /// # }
        /// ```
        ///
        /// [`PlayerLeaderboard`]: model/rankings/players/struct.PlayerLeaderboard.html
        /// [`Error::Request`]: error/enum.Error.html#variant.Request
        /// [`Error::Status`]: error/enum.Error.html#variant.Status
        /// [`Error::Ratelimited`]: error/enum.Error.html#variant.Ratelimited
        /// [`Error::Json`]: error/enum.Error.html#variant.Json
        #[cfg(feature = "async")]
        async fn a_fetch(client: &Client, prop: &'async_trait Self::Property, limit: Self::Limit) -> Result<Self>
            where Self: 'async_trait,
                  Self::Property: 'async_trait,
                  Self::Limit: 'async_trait;
    }
}

pub use proplimfetch::*;

// endregion:PropFetch

/// A trait representing a type whose instance can be fetched again.
/// Note that all types implementing [`GetFetchProp`] and [`PropFetchable`] also implement
/// [`Refetchable`] due to a blanket implementation.
///
/// [`PropFetchable`]: traits/propfetch/trait.PropFetchable.html
/// [`GetFetchProp`]: traits/propfetch/trait.GetFetchProp.html
#[cfg_attr(feature = "async", async_trait)]
pub trait Refetchable: Sized {
    /// (Sync) Causes this instance to be re-fetched (i.e., updated to latest Brawl Stars data).
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use brawl_api::prelude::*;
    ///
    /// # fn main() -> Result<(), Box<dyn ::std::error::Error>> {
    /// let my_client = Client::new("my auth key");
    ///
    /// let player = Player::fetch(&my_client, "#PLAYER_TAG_HERE")?;
    ///
    /// // after using it a bit, we want to update its data
    ///
    /// let player = player.refetch(&my_client)?;  // `refetch` does not mutate the instance!
    ///
    /// // player variable is now up-to-date.
    ///
    /// #     Ok(())
    /// # }
    /// ```
    fn refetch(&self, client: &Client) -> Result<Self>;

    /// (Sync) Like `refetch`, but mutates the instance, returning an immutable reference to it.
    ///
    /// Its usage and errors are the same (it is called, after all), but the old variable does
    /// not need to be assigned; rather, that is done for the programmer (the variable's value
    /// **is entirely replaced** by a new one, if the fetching is successful).
    fn refetch_update(&mut self, client: &Client) -> Result<&Self> {
        *self = self.refetch(client)?;
        Ok(self as &Self)
    }

    /// (Async) Causes this instance to be re-fetched (i.e., updated to latest Brawl Stars data).
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use brawl_api::prelude::*;
    ///
    /// # async fn main() -> Result<(), Box<dyn ::std::error::Error>> {
    /// let my_client = Client::new("my auth key");
    ///
    /// let player = Player::a_fetch(&my_client, "#PLAYER_TAG_HERE").await?;
    ///
    /// // after using it a bit, we want to update its data
    ///
    /// let player = player.a_refetch(&my_client).await?;  // this does not mutate the old instance!
    ///
    /// // player variable is now up-to-date.
    ///
    /// #     Ok(())
    /// # }
    /// ```
    #[cfg(feature = "async")]
    async fn a_refetch(&self, client: &Client) -> Result<Self>;

    /// (Async) Like `a_refetch`, but mutates the instance, returning an immutable reference to it.
    ///
    /// Its usage and errors are the same (it is called, after all), but the old variable does
    /// not need to be assigned; rather, that is done for the programmer (the variable's value
    /// **is entirely replaced** by a new one, if the fetching is successful).
    #[cfg(feature = "async")]
    async fn a_refetch_update(&'async_trait mut self, client: &Client) -> Result<&'async_trait Self>
        where Self: Send + Sync,
    {
        *self = self.a_refetch(client).await?;
        Ok(self as &Self)
    }
}

#[cfg_attr(feature = "async", async_trait)]
impl<T> Refetchable for T
    where T: PropFetchable<Property=<T as GetFetchProp>::Property> + GetFetchProp + Sized + Send + Sync,
          <T as GetFetchProp>::Property: Sync + Send {
    fn refetch(&self, client: &Client) -> Result<Self> {
        Self::fetch(client, self.get_fetch_prop())
    }

    #[cfg(feature = "async")]
    async fn a_refetch(&self, client: &Client) -> Result<Self>
        where T: 'async_trait,
        <T as GetFetchProp>::Property: 'async_trait,
    {
        Self::a_fetch(client, self.get_fetch_prop()).await
    }
}


/// A trait indicating that another type can be converted into this one by fetching from the API.
/// Note that, thanks to a blanket implementation, implementing this implies implementing
/// [`FetchInto`] for the other type.
///
/// [`FetchInto`]: traits/trait.FetchInto.html
#[cfg_attr(feature = "async", async_trait)]
pub trait FetchFrom<T>: Sized {
    /// (Sync) Attempts to request to the API and return a new instance of the type being turned
    /// into.
    ///
    /// # Errors
    ///
    /// See the respective struct's `fetch` implementation (or `PropFetchable`/`PropLimFetchable`
    /// implementation).
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use brawl_api::{Client, Player, Club, traits::*};
    ///
    /// # fn main() -> Result<(), Box<dyn ::std::error::Error>> {
    /// let my_client = Client::new("my auth token");
    /// let club = Club::fetch(&my_client, "#CLUB_TAG_HERE")?;
    /// let some_member = &club.members[0];
    /// let some_player = Player::fetch_from(&my_client, some_member)?;
    /// // now `some_member`'s full data, as a Player, is available for use.
    ///
    /// #     Ok(())
    /// # }
    /// ```
    fn fetch_from(client: &Client, value: &T) -> Result<Self>;

    /// (Async) Attempts to request to the API and return a new instance of the type being turned
    /// into.
    ///
    /// # Errors
    ///
    /// See the respective struct's `a_fetch` implementation (or `PropFetchable`/`PropLimFetchable`
    /// implementation).
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use brawl_api::{Client, Player, Club, traits::*};
    ///
    /// # async fn main() -> Result<(), Box<dyn ::std::error::Error>> {
    /// let my_client = Client::new("my auth token");
    /// let club = Club::a_fetch(&my_client, "#CLUB_TAG_HERE").await?;
    /// let some_member = &club.members[0];
    /// let some_player = Player::a_fetch_from(&my_client, some_member).await?;
    /// // now `some_member`'s full data, as a Player, is available for use.
    ///
    /// #     Ok(())
    /// # }
    /// ```
    #[cfg(feature = "async")]
    async fn a_fetch_from(client: &Client, value: &T) -> Result<Self>;
}

/// A trait indicating that this type can be converted into another by fetching from the API.
/// Note that [`FetchFrom`] should be implemented, in order to apply the respective blanket
/// implementation of this trait.
///
/// [`FetchFrom`]: traits/trait.FetchFrom.html
#[cfg_attr(feature = "async", async_trait)]
pub trait FetchInto<T>: Sized {
    /// (Sync) Attempts to request to the API and return a new instance of the type being turned
    /// into.
    ///
    /// # Errors
    ///
    /// See the respective into-type's `fetch` implementation (or `PropFetchable`/`PropLimFetchable`
    /// implementation).
    ///
    /// # Examples
    ///
    /// See [`FetchFrom::<T>::fetch_from`].
    fn fetch_into(&self, client: &Client) -> Result<T>;

    #[cfg(feature = "async")]
    /// (Async) Attempts to request to the API and return a new instance of the type being turned
    /// into.
    ///
    /// # Errors
    ///
    /// See the respective into-type's `a_fetch` implementation (or
    /// `PropFetchable`/`PropLimFetchable` implementation).
    ///
    /// # Examples
    ///
    /// See [`FetchFrom::<T>::a_fetch_from`].
    async fn a_fetch_into(&self, client: &Client) -> Result<T>
        where T: 'async_trait;
}

// FetchFrom implies FetchInto
#[cfg_attr(feature = "async", async_trait)]
impl<T, U> FetchInto<U> for T
    where T: Sync + Send, U: FetchFrom<T> + Sync + Send
{
    fn fetch_into(&self, client: &Client) -> Result<U> {
        U::fetch_from(client, self)
    }

    #[cfg(feature = "async")]
    async fn a_fetch_into(&self, client: &Client) -> Result<U>
        where U: 'async_trait
    {
        U::a_fetch_from(client, self).await
    }
}

// FetchFrom (and thus FetchInto) is reflexive
#[cfg_attr(feature = "async", async_trait)]
impl<T: Sync + Send + Clone> FetchFrom<T> for T {
    /// (Sync) Returns a copy of the current instance when attempting to fetch from itself.
    /// In order to re-fetch, see [`Refetchable`].
    ///
    /// # Errors
    ///
    /// Never errors; is only a [`Result`] in order to match the trait signature.
    ///
    /// [`Refetchable`]: trait.Refetchable.html
    /// [`Result`]: ../error/type.Result.html
    fn fetch_from(_: &Client, t: &T) -> Result<T> { Ok(t.to_owned()) }

    /// (Async) Returns a copy of the current instance when attempting to fetch from itself.
    /// In order to re-fetch, see [`Refetchable`].
    ///
    /// # Errors
    ///
    /// Never errors; is only a [`Result`] in order to match the trait signature.
    ///
    /// [`Refetchable`]: trait.Refetchable.html
    /// [`Result`]: ../error/type.Result.html
    #[cfg(feature = "async")]
    async fn a_fetch_from(_: &Client, t: &T) -> Result<Self> { Ok(t.to_owned()) }
}
