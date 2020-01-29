//! Contains constant values used within the lib.

/// The initial URL path to the Brawl Stars API v1.
pub const API_URI: &str = "https://api.brawlstars.com/v1/";

/// The user agent to use indicating this lib was used to request.
pub const USER_AGENT: &str = concat!(
    "Rust (brawl-api crate, ", env!("CARGO_PKG_VERSION"),
    " - https://github.com/PgBiel/rust-brawl-api)"
);

/// The format used in [`TimeLike.parse`]. (Feature-gated with the `chrono` feature)
///
/// `"%Y%m%dT%H%M%S%.fZ"`
///
/// See [this table] for more info.
///
/// [`TimeLike.parse`]: ../time/struct.TimeLike.html#method.parse
/// [this table]: https://docs.rs/chrono/*/chrono/format/strftime/index.html
#[cfg(feature = "chrono")]
pub const TIMELIKE_FORMAT: &str = "%Y%m%dT%H%M%S%.fZ";

/// This eunm is an effort to aid the programmer's usage of brawler-related endpoints, by mapping
/// human-readable brawler names to their respective IDs. (Use by casting to int; e.g. `x as usize`)
///
/// This is by no means a final enum and must be updated on every new Brawler release.
///
/// If a permanently up-to-date list is needed, one can fetch the `/brawlers/` endpoint using
/// the available models. If still using this enum, though, rest assured that we will do our best
/// to keep it updated - if it is not, why not contribute with a PR? ;)
#[non_exhaustive]
#[derive(Copy, Debug, Clone, Hash, PartialEq, Eq)]
pub enum Brawlers {
    Shelly = 16000000,
    Colt = 16000001,
    Bull = 16000002,
    Brock = 16000003,
    Rico = 16000004,
    Spike = 16000005,
    Barley = 16000006,
    Jessie = 16000007,
    Nita = 16000008,
    Dynamike = 16000009,
    ElPrimo = 16000010,
    Mortis = 16000011,
    Crow = 16000012,
    Poco = 16000013,
    Bo = 16000014,
    Piper = 16000015,
    Pam = 16000016,
    Tara = 16000017,
    Darryl = 16000018,
    Penny = 16000019,
    Frank = 16000020,
    Gene = 16000021,
    Tick = 16000022,
    Leon = 16000023,
    Rosa = 16000024,
    Carl = 16000025,
    Bibi = 16000026,
    EightBit = 16000027,
    Sandy = 16000028,
    Bea = 16000029,
    Emz = 16000030,
    MrP = 16000031,
    Max = 16000032,
}
