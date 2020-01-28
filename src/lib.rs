//! The `brawl-api` crate creates models and fetching methods employing the
//! [official Brawl Stars API](https://developer.brawlstars.com). It aims to be an usable,
//! predictable and programmer-friendly library. Anywhere it is not seen as so, contribution is
//! very welcome.
//!
//! # Requesting
//!
//! All the requesting work is done by the [`reqwest` crate].
//! The library supports both sync (blocking) and, with the `async` feature (enabled by default),
//! async function styles - all fetching methods with that duality have a sync variant (with its
//! normal name) and an async one (named by prefixing the sync name with `a_` - e.g.: `fetch` (sync)
//! and `a_fetch` (async)).
//!
//! # Deserialization and data structure
//!
//! Deserialization of JSON is done thanks to the [`serde`] and [`serde-json`] libraries, which are
//! of invaluable aid on that aspect. This means that **all library models** implement the
//! [`Serialize`] and [`Deserialize`] serde traits, optimized to match the JSON formats documented
//! in the API's official documentation.
//!
//! In addition, it is noticeable that most API models implement the `Default` trait. This is for
//! usage with `serde`, in order to ensure that the values will be there, regardless of an API
//! failure. Fields not marked with `Option<T>` **should be there** by most standards - note that
//! the official Brawl Stars API docs state that all fields everywhere are optional (which isn't
//! helpful), even though most of them will always occur unless a server fault or some unexpected,
//! new use-case was added (in which case, the library can be blamed for not being updated - **feel
//! free to contribute with a PR, or just an issue poking me about this, adding this to the
//! library!**).
//!
//! For more info on models, see the [`model`] module.
//!
//! # Recommended Usage
//!
//! It is recommended to import the library using its [`prelude`] module:
//!
//! ```rust
//! use brawl_api::prelude::*;
//! ```
//!
//! This brings into scope all of the library's traits, models and the helper [`Brawlers`] enum.
//!
//! If you do not wish to bring all models into scope, then at least **import all traits** so that
//! models work properly:
//!
//! ```rust
//! use brawl_api::traits::*;
//! ```
//!
//! # Feature Flags
//!
//! The crate has a few feature flags available (all enabled by default):
//!
//! - `async` flag:
//!     - Enables the usage of async (non-blocking) fetch functions - `a_fetch`, `a_fetch_from`,
//! `a_fetch_into`, `a_refetch` - where applicable.
//!     - Adds `async_trait` as a dependency.
//! - `auto-hashtag` flag: Enables the smart insertion of hashtags on anywhere a tag is required.
//!     - This means, for example, that on a [`Player::fetch`] call, which requires the tag of the
//! player to be fetched, one can pass a string containing a hashtag at the start (in which case,
//! the library simply uses it) ***or without*** (then, with this feature on, the lib adds it for
//! you).
//!     - Disabling this requires passing hashtags at the start of every tag string. This is due to
//! how the API parses tags, and not much can be done about it.
//! - `players` flag: Enables the usage of the [`model::players`] module (for the `/players` endpoint).
//! - `clubs` flag: Enables the usage of the [`model::clubs`] module (for the `/clubs` endpoint).
//! - `rankings` flag: Enables the usage of the [`model::rankings`] module (for the `/rankings` endpoint).
//! - `brawlers` flag: Enables the usage of the [`model::brawlers`] module (for the `/brawlers` endpoint).
//!
//! [`reqwest` crate]: https://crates.io/crate/reqwest
//! [`serde`]: https://crates.io/crate/serde
//! [`serde-json`]: https://crates.io/crate/serde-json
//! [`Serialize`]: https://docs.rs/serde/*/ser/trait.Serialize.html
//! [`Deserialize`]: https://docs.rs/serde/*/de/trait.Deserialize.html
//! [`Player::fetch`]: model/players/player/struct.Player.html#method.fetch
//! [`model`]: model/
//! [`prelude`]: prelude/
//! [`Brawlers`]: constants/enum.Brawlers.html
//! [`model::players`]: model/players/
//! [`model::clubs`]: model/clubs/
//! [`model::rankings`]: model/rankings/
//! [`model::brawlers`]: model/brawlers/

pub(crate) mod util;

pub(crate) mod serde;

pub mod constants;
pub use constants::Brawlers;

pub mod http;
pub use http::client::Client;

mod macros;

pub mod model;

#[cfg(feature = "players")]
pub use model::players::{
    Player, PlayerClub, PlayerBrawlerStat, StarPower,
    battlelog::{
        BattleLog,
        Battle, BattleEvent, BattleResultInfo,
        BattlePlayer, BattleBrawler, BattleOutcome,
    },
};

pub mod traits;

#[cfg(feature = "clubs")]
pub use model::clubs::{Club, ClubMember, ClubMembers, ClubMemberRole, ClubType};

#[cfg(feature = "rankings")]
pub use model::rankings::{
    players::{PlayerLeaderboard, PlayerRanking, PlayerRankingClub},
    clubs::{ClubLeaderboard, ClubRanking},
    brawlers::BrawlerLeaderboard,
};

pub mod error;
pub use error::{Error, Result};

pub mod prelude;

//#[cfg(test)]
//mod tests {
//    #[test]
//    fn it_works() {
//        assert_eq!(2 + 2, 4);
//    }
//}
