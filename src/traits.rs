use crate::error::Error;

/// A trait representing a type whose instance can be fetched from the API using some property.
/// This is usually the object's tag.
pub trait Fetchable: Sized {
    type Property;

    fn fetch(prop: Self::Property) -> Result<Self, Error>;
}
// TODO: async equivalents & stuff

/// A trait indicating that another type can be converted into this one by fetching from the API.
/// Note that, thanks to a blanket implementation, implementing this implies implementing
/// [FetchInto].
pub trait FetchFrom<T>: Sized {
    /// Performs the conversion by fetching the equivalent.
    fn fetch_from(value: T) -> Result<Self, Error>;
}

/// A trait indicating that this type can be converted into another by fetching from the API.
/// Note that [FetchFrom] should be implemented.
pub trait FetchInto<T>: Sized {
    fn fetch_into(self) -> Result<T, Error>;
}

// FetchFrom implies FetchInto
impl<T, U> FetchInto<U> for T where U: FetchFrom<T>
{
    fn fetch_into(self) -> Result<U, Error> {
        U::fetch_from(self)
    }
}

// FetchFrom (and thus FetchInto) is reflexive
impl<T> FetchFrom<T> for T {
    fn fetch_from(t: T) -> Result<T, Error> { Ok(t) }
}
