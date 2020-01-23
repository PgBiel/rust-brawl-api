use crate::error::{Result, Error};

#[cfg(feature = "async")]
use async_trait::async_trait;
use crate::http::Client;
use serde::de::DeserializeOwned;
use crate::util::fetch_route;

/// A trait representing a struct/enum which can generate an instance with default values
/// for its properties on `Type::new()`.
pub trait Initializable {
    fn new() -> Self;
}

impl<T: Initializable> Default for T {
    fn default() -> T { T::new() }  // that's the whole point of it, after all
}

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
    fn fetch(client: &Client, prop: &Self::Property) -> Result<Self>;

    /// (Async) Fetch and construct a new instance of this type.
    #[cfg(feature = "async")]
    async fn a_fetch(client: &Client, prop: &Self::Property) -> Result<Self>;

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
    fn get_route(prop: &Self::Property) -> Self::Property;
}

#[cfg_attr(feature = "async", async_trait)]
impl<T, U> PropFetchable for T
    where T: PropRouteable<Property=U> + GetFetchProp + DeserializeOwned + Sized {
    type Property = U;

    fn fetch(client: &Client, prop: &Self::Property) -> Result<T> {
        let route = Self::get_route(prop);
        fetch_route::<T>(client, route)
    }

    #[cfg(feature="async")]
    async fn a_fetch(client: &Client, prop: &Self::Property) -> Result<T> {
        let route = Self::get_route(prop);
        // TODO: a_fetch_route::<T>(client, route)
    }
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
    where T: GetFetchProp + PropFetchable + Sized {
    fn refetch(self, client: &Client) -> Result<Self> {
        Self::fetch(client, &self.get_fetch_prop())
    }

    #[cfg(feature = "async")]
    async fn a_refetch(self, client: &Client) -> Result<Self> {
        Self::a_fetch(client, &self.get_fetch_prop()).await
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
    fn fetch_from(value: T, client: &Client) -> Result<Self>;

    #[cfg(feature = "async")]
    async fn a_fetch_from(value: T, client: &Client) -> Result<Self>;
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
    async fn a_fetch_into(self, client: &Client) -> Result<T>;
}

// FetchFrom implies FetchInto
#[cfg_attr(feature = "async", async_trait)]
impl<T, U> FetchInto<U> for T where U: FetchFrom<T>
{
    fn fetch_into(self, client: &Client) -> Result<U> {
        U::fetch_from(self, client)
    }

    #[cfg(feature = "async")]
    async fn a_fetch_into(self, client: &Client) -> Result<U> {
        U::a_fetch_from(self, client).await
    }
}

// FetchFrom (and thus FetchInto) is reflexive
#[cfg_attr(feature = "async", async_trait)]
impl<T> FetchFrom<T> for T {
    fn fetch_from(t: T, _: &Client) -> Result<T> { Ok(t) }

    #[cfg(feature = "async")]
    async fn a_fetch_from(t: T, _: &Client) -> Result<Self> { Ok(t) }
}
