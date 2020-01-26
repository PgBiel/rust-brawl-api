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
    /// [`PropFetchable`]: ./trait.PropFetchable.html
    /// [`GetFetchProp`]: ./trait.GetFetchProp.html
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
        fn fetch(client: &Client, prop: &Self::Property) -> Result<Self>;

        /// (Async) Fetch and construct a new instance of this type.
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
        fn fetch(client: &Client, prop: &Self::Property, limit: Self::Limit) -> Result<Self>;

        /// (Async) Fetch and construct a new instance of this type.
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
/// [`PropFetchable`]: ./trait.PropFetchable.html
/// [`GetFetchProp`]: ./trait.GetFetchProp.html
#[cfg_attr(feature = "async", async_trait)]
pub trait Refetchable: Sized {
    /// (Sync) Causes this instance to be re-fetched (i.e., updated to latest Brawl Stars data).
    fn refetch(self, client: &Client) -> Result<Self>;

    /// (Async) Causes this instance to be re-fetched (i.e., updated to latest Brawl Stars data).
    #[cfg(feature = "async")]
    async fn a_refetch(self, client: &Client) -> Result<Self>;
}

#[cfg_attr(feature = "async", async_trait)]
impl<T> Refetchable for T
    where T: PropFetchable<Property=<T as GetFetchProp>::Property> + GetFetchProp + Sized + Send + Sync,
          <T as GetFetchProp>::Property: Sync + Send + Clone {
    fn refetch(self, client: &Client) -> Result<Self> {
        Self::fetch(client, self.get_fetch_prop())
    }

    #[cfg(feature = "async")]
    async fn a_refetch(self, client: &Client) -> Result<Self>
        where T: 'async_trait,
        <T as GetFetchProp>::Property: 'async_trait,
    {
        Self::a_fetch(client, self.get_fetch_prop()).await
    }
}


/// A trait indicating that another type can be converted into this one by fetching from the API.
/// Note that, thanks to a blanket implementation, implementing this implies implementing
/// [`FetchInto`].
///
/// [`FetchInto`]: ./trait.FetchInto.html
#[cfg_attr(feature = "async", async_trait)]
pub trait FetchFrom<T>: Sized {
    /// Performs the conversion by fetching the equivalent.
    fn fetch_from(client: &Client, value: T) -> Result<Self>;

    #[cfg(feature = "async")]
    async fn a_fetch_from(client: &Client, value: T) -> Result<Self>;
}

/// A trait indicating that this type can be converted into another by fetching from the API.
/// Note that [`FetchFrom`] should be implemented, in order to apply the respective blanket
/// implementation of this trait.
///
/// [`FetchFrom`]: ./trait.FetchFrom.html
#[cfg_attr(feature = "async", async_trait)]
pub trait FetchInto<T>: Sized {
    fn fetch_into(self, client: &Client) -> Result<T>;

    #[cfg(feature = "async")]
    async fn a_fetch_into(self, client: &Client) -> Result<T>
        where T: 'async_trait;
}

// FetchFrom implies FetchInto
#[cfg_attr(feature = "async", async_trait)]
impl<T, U> FetchInto<U> for T
    where T: Sync + Send, U: FetchFrom<T> + Sync + Send
{
    fn fetch_into(self, client: &Client) -> Result<U> {
        U::fetch_from(client, self)
    }

    #[cfg(feature = "async")]
    async fn a_fetch_into(self, client: &Client) -> Result<U>
        where U: 'async_trait
    {
        U::a_fetch_from(client, self).await
    }
}

// FetchFrom (and thus FetchInto) is reflexive
#[cfg_attr(feature = "async", async_trait)]
impl<T: Sync + Send> FetchFrom<T> for T {
    fn fetch_from(_: &Client, t: T) -> Result<T> { Ok(t) }

    #[cfg(feature = "async")]
    async fn a_fetch_from(_: &Client, t: T) -> Result<Self> { Ok(t) }
}
