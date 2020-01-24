use crate::error::{Result, Error};

use std::error::Error as StdError;

use serde_json::Error as JsonError;

#[cfg(feature = "async")]
use async_trait::async_trait;
use crate::http::Client;
use serde::de::DeserializeOwned;
use crate::util::fetch_route;
use crate::http::routes::Route;

#[cfg(feature = "async")]
use crate::util::{a_fetch_route, deser_a_fetch};
#[cfg(feature = "async")]
use std::pin::Pin;
#[cfg(feature = "async")]
use std::future::Future;

/// A trait representing a type with a property used to be fetched.
/// This property must be returned by the `get_fetch_prop` function.
/// This trait is used in parallel with `PropFetchable`.
pub trait GetFetchProp: Sized {
    type Property;

    /// Obtain the revelant property for fetching.
    #[doc(hidden)]
    fn get_fetch_prop(&self) -> &Self::Property;  // necessary for Refetchable blanket impl
}

/// A trait representing a type whose instance can be fetched from the API using some property.
/// This is usually the object's tag.
#[cfg_attr(feature = "async", async_trait)]
pub trait PropFetchable: Sized {
    type Property;

    /// (Sync) Fetch and construct a new instance of this type.
    fn fetch(client: &Client, prop: Self::Property) -> Result<Self>;

    /// (Async) Fetch and construct a new instance of this type.
    #[cfg(feature = "async")]
    async fn a_fetch(client: &Client, prop: Self::Property) -> Result<Self>
        where Self: 'async_trait,
              Self::Property: 'async_trait;

    // /// Fetches an object once again.
    //    fn refetch(&self) -> Result<Self> {
    //        Self::fetch()
    //    }
}

/// A trait representing a type which holds some method of obtaining a route.
/// This causes a blanket implementation of [`PropFetchable`] using the library's
/// `fetch_route` tool for types implementing [`GetFetchProp`] and [`serde::de::DeserializeOwned`].
///
/// [`PropFetchable`]: ./trait.PropFetchable.html
/// [`GetFetchProp`]: ./trait.GetFetchProp.html
/// [`serde::de::DeserializeOwned`]: https://docs.rs/serde/*/serde/de/trait.DeserializeOwned.html
pub trait PropRouteable: Sized {
    type Property;

    /// Obtain the relevant route for fetching, given the property.
    #[doc(hidden)]
    fn get_route(prop: &Self::Property) -> Route;
}

#[cfg_attr(feature = "async", async_trait)]
impl<'a, T> PropFetchable for T
    where T: PropRouteable<Property=<T as GetFetchProp>::Property> + GetFetchProp + DeserializeOwned + Sized + Sync,
    <T as GetFetchProp>::Property: Sync + Send, {
    type Property = <T as GetFetchProp>::Property;

    /// (Sync) Fetches this instance.
    fn fetch(client: &Client, prop: Self::Property) -> Result<T> {
        let route = Self::get_route(&prop);
        fetch_route::<T>(client, &route)
    }

    /// (Async) Fetches this instance.
    #[cfg(feature="async")]
    async fn a_fetch(client: &Client, prop: Self::Property) -> Result<Self>
        where Self: 'async_trait,
              Self::Property: 'async_trait,
    {
        let route = Self::get_route(&prop);
        a_fetch_route::<Self>(client, &route).await
    }
//    fn a_fetch<'b: 'a>(
//        client: &Client, prop: Self::Property,
//    ) -> Pin<Box<dyn std::future::Future<Output = Result<T>> + Send + 'b>>
//        where
//            Self: Sync + 'b,
//    {
//        async fn a_fetch<S, U>(client: &Client, prop: U) -> Result<S>
//            where S: PropRouteable<Property=U> + GetFetchProp + DeserializeOwned + Sized + Sync,
//            U: Sync + Send, {
//            let route = S::get_route(&prop);
//            a_fetch_route::<S>(client, &route).await
//        }
//
//        Box::pin(a_fetch::<T, U>(client, prop))
//    }
}

/// A trait representing a type whose instance can be fetched again.
/// Note that all types implementing [`GetFetchProp`] and [`PropFetchable`] also implement
/// [`Refetchable`] due to a blanket implementation.
///
/// [`PropFetchable`]: ./traits/trait.PropFetchable.html
/// [`GetFetchProp`]: ./traits/trait.GetFetchProp.html
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
    where T: GetFetchProp + PropFetchable<Property=<T as GetFetchProp>::Property> + Sized + Send + Sync,
          <T as GetFetchProp>::Property: Sync + Send + Clone {
    fn refetch(self, client: &Client) -> Result<Self> {
        Self::fetch(client, (*self.get_fetch_prop()).clone())
    }

    #[cfg(feature = "async")]
    async fn a_refetch(self, client: &Client) -> Result<Self>
        where T: 'async_trait,
        <T as GetFetchProp>::Property: 'async_trait,
    {
        Self::a_fetch(client, (*self.get_fetch_prop()).clone()).await
    }
}

// vvvvv can't access fields from traits; perhaps do something about this?
//pub trait TagFetchable: for<'a> Fetchable<Property = &'a str> + Sized {}
//
//impl<'a, T: Fetchable<Property = &'a str>> TagFetchable for T {}
//
//impl<'a> Refetchable for TagFetchable {
//    fn refetch(self) -> Result<Self> {
//        let new_res = Self::fetch(self.tag);
//
//    }
//}


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
