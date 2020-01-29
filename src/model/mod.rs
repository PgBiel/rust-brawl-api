//! Contains models for each documented Brawl Stars API endpoint.
//!
//! All models derive/implement the following traits:
//!
//! - `Debug`
//! - `Clone`
//! - `Hash`
//! - `PartialEq`
//! - `Eq`
//! - [`serde::ser::Serialize`]
//! - [`serde::de::Deserialize`]
//! - `Default` (except for Smart Pointer models, such as all `-Leaderboard`s and [`BattleLog`]).
//!
//! A few may also implement `PartialOrd` and `Ord`, such as all `-Ranking` structs and the pair
//! [`ClubMember`] and [`ClubMemberRole`].
//!
//! In addition, each endpoint has a different way of fetching, and is associated with a submodule:
//!
//! - `/players/:tag` -> [`Player::fetch`] (through the [`PropFetchable`] trait),
//! [`model::players::player`] module;
//! - `/players/:tag/battlelog` -> [`BattleLog::fetch`] (through the [`PropFetchable`] trait),
//! [`model::players::battlelog`] module;
//! - `/clubs/:tag` -> [`Club::fetch`] (through the [`PropFetchable`] trait),
//! [`model::clubs`] module;
//! - `/clubs/:tag/members` -> [`ClubMembers::fetch`] (through the [`PropFetchable`] trait),
//! [`model::clubs::members`] module;
//! - `/rankings/:country_code/players?limit=x` -> [`PlayerLeaderboard::fetch`] (through the
//! [`PropLimFetchable`] trait), [`model::rankings::players`] module;
//! - `/rankings/:country_code/clubs?limit=x` -> [`ClubLeaderboard::fetch`] (through the
//! [`PropLimFetchable`] trait), [`model::rankings::clubs`] module;
//! - `/rankings/:country_code/brawlers/:brawler_id?limit=x` -> [`BrawlerLeaderboard::fetch`]
//! (direct implementation; no fetching-related traits), [`model::rankings::brawlers`] module;
//! - `/brawlers/` -> [`BrawlerList::fetch`] (direct implementation), [`model::brawlers`] module;
//! - `/brawlers/:id` -> [`Brawler::fetch`] (direct implementation), [`model::brawlers`] module.
//!
//! [`serde::ser::Serialize`]: https://docs.rs/serde/*/ser/trait.Serialize.html
//! [`serde::de::Deserialize`]: https://docs.rs/serde/*/de/trait.Deserialize.html
//! [`BattleLog`]: ./players/battlelog/struct.BattleLog.html
//! [`ClubMember`]: ./clubs/struct.ClubMember.html
//! [`ClubMemberRole`]: ./clubs/enum.ClubMemberRole.html
//! [`Player::fetch`]: ./players/player/struct.Player.html#method.fetch
//! [`BattleLog::fetch`]: ./players/battlelog/struct.BattleLog.html#method.fetch
//! [`Club::fetch`]: ./clubs/struct.Club.html#method.fetch
//! [`ClubMembers::fetch`]: ./clubs/members/struct.ClubMembers.html#method.fetch
//! [`PlayerLeaderboard::fetch`]: ./rankings/players/struct.PlayerLeaderboard.html#method.fetch
//! [`ClubLeaderboard::fetch`]: ./rankings/players/struct.ClubLeaderboard.html#method.fetch
//! [`BrawlerLeaderboard::fetch`]: ./rankings/players/struct.BrawlerLeaderboard.html#method.fetch
//! [`BrawlerList::fetch`]: ./brawlers/struct.BrawlerList.html#method.fetch
//! [`Brawler::fetch`]: ./brawlers/struct.Brawler.html#method.fetch
//! [`PropFetchable`]: traits/propfetch/trait.PropFetchable.html
//! [`PropLimFetchable`]: traits/proplimfetch/trait.PropLimFetchable.html
//! [`model::players::player`]: ./players/player/index.html
//! [`model::players::battlelog`]: ./players/battlelog/index.html
//! [`model::clubs`]: ./clubs/index.html
//! [`model::clubs::members`]: ./clubs/members/index.html
//! [`model::rankings::players`]: ./rankings/players/index.html
//! [`model::rankings::clubs`]: ./rankings/clubs/index.html
//! [`model::rankings::brawlers`]: ./rankings/brawlers/index.html
//! [`model::brawlers`]: ./brawlers/index.html

pub mod common;
pub use common::*;

#[cfg(feature = "players")]
pub mod players;
#[cfg(feature = "players")]
pub use players::*;

#[cfg(feature = "clubs")]
pub mod clubs;
#[cfg(feature = "clubs")]
pub use clubs::*;

#[cfg(feature = "rankings")]
pub mod rankings;
#[cfg(feature = "rankings")]
pub use rankings::*;

#[cfg(feature = "brawlers")]
pub mod brawlers;
#[cfg(feature = "brawlers")]
pub use brawlers::*;
