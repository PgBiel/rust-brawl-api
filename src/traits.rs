use crate::error::{Result, Error};

#[cfg(feature = "async")]
use async_trait::async_trait;

/// A trait representing a struct/enum which can generate an instance with default values
/// for its properties.
pub trait Initializable {
    fn new() -> Self;
}

/// A trait representing a type whose instance can be fetched from the API using some property.
/// This is usually the object's tag.
#[cfg_attr(feature = "async", async_trait)]
pub trait Fetchable: Sized {
    type Property;

    fn fetch(prop: Self::Property) -> Result<Self>;

    #[cfg(feature = "async")]
    async fn a_fetch(prop: Self::Property) -> Result<Self>;
}
// TODO: async equivalents & stuff

/// A trait indicating that another type can be converted into this one by fetching from the API.
/// Note that, thanks to a blanket implementation, implementing this implies implementing
/// [`FetchInto`].
///
/// [`FetchInto`]: ./trait.FetchInto.html
#[cfg_attr(feature = "async", async_trait)]
pub trait FetchFrom<T>: Sized {
    /// Performs the conversion by fetching the equivalent.
    fn fetch_from(value: T) -> Result<Self>;

    #[cfg(feature = "async")]
    async fn a_fetch_from(value: T) -> Result<Self>;
}

/// A trait indicating that this type can be converted into another by fetching from the API.
/// Note that [`FetchFrom`] should be implemented, in order to apply the respective blanket
/// implementation of this trait.
///
/// [`FetchFrom`]: ./trait.FetchFrom.html
#[cfg_attr(feature = "async", async_trait)]
pub trait FetchInto<T>: Sized {
    fn fetch_into(self) -> Result<T>;

    #[cfg(feature = "async")]
    async fn a_fetch_into(self) -> Result<T>;
}

// FetchFrom implies FetchInto
#[cfg_attr(feature = "async", async_trait)]
impl<T, U> FetchInto<U> for T where U: FetchFrom<T>
{
    fn fetch_into(self) -> Result<U> {
        U::fetch_from(self)
    }

    #[cfg(feature = "async")]
    async fn a_fetch_into(self) -> Result<U> {
        U::a_fetch_from(self).await
    }
}

// FetchFrom (and thus FetchInto) is reflexive
#[cfg_attr(feature = "async", async_trait)]
impl<T> FetchFrom<T> for T {
    fn fetch_from(t: T) -> Result<T> { Ok(t) }

    #[cfg(feature = "async")]
    async fn a_fetch_from(t: T) -> Result<Self> { Ok(t) }
}
