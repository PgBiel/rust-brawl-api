//! The `brawl-api` crate creates models and fetching methods employing the
//! [official Brawl Stars API](https://developer.brawlstars.com). It aims to be an usable,
//! predictable and programmer-friendly library. Anywhere it is not seen as so, contribution is
//! very welcome.
//!
//! All the requesting work is done by the [`reqwest` crate].
//! The library supports both sync and, with the `async` feature (enabled by default), async
//! function styles - all fetching methods with that duality have a sync variant (with its normal
//! name) and an async one (named by prefixing the sync name with `a_` - e.g.: `fetch` (sync) and
//! `a_fetch` (async)).
//!
//! Deserialization of JSON is done thanks to the [`serde`] and [`serde-json`] libraries, which are
//! of invaluable aid on that aspect. This means that **all library models** implement the
//! [`Serialize`] and [`Deserialize`] serde traits, optimized to match the JSON formats documented
//! in the API's official documentation.
//!
//! [`reqwest` crate]: https://crates.io/crate/reqwest
//! [`serde`]: https://crates.io/crate/serde
//! [`serde-json`]: https://crates.io/crate/serde-json
//! [`Serialize`]: https://docs.rs/serde/*/ser/trait.Serialize.html
//! [`Deserialize`]: https://docs.rs/serde/*/de/trait.Deserialize.html

pub(crate) mod util;

pub(crate) mod serde;

pub mod traits;

pub mod constants;

pub mod http;
pub use http::Client;

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

#[cfg(feature = "clubs")]
pub use model::clubs::{Club, ClubMember, ClubMemberRole, ClubType};

#[cfg(feature = "rankings")]
pub use model::rankings::{
    players::{PlayerLeaderboard, PlayerRanking, PlayerRankingClub},
    clubs::{ClubLeaderboard, ClubRanking},
};

pub mod error;

//#[cfg(test)]
//mod tests {
//    #[test]
//    fn it_works() {
//        assert_eq!(2 + 2, 4);
//    }
//}
