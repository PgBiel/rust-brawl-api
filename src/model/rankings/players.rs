use serde::{self, Serialize, Deserialize};
use crate::traits::{PropLimRouteable, PropLimFetchable};
use crate::serde::{one_default, oxffffff_default};
use std::ops::Deref;
use crate::util::fetch_route;
use crate::error::Result;

#[cfg(feature = "async")]
use async_trait::async_trait;

#[cfg(feature = "async")]
use crate::util::a_fetch_route;
use crate::http::Client;
use crate::http::routes::Route;

/// Represents a leaderboard of [`PlayerRanking`]s - the top x players in a regional or global
/// leaderboard.
///
/// **NOTE:** The API only allows fetching up to the top 200 players.
///
/// [`PlayerRanking`]: ./struct.PlayerRanking.html
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlayerLeaderboard {
    /// The players in the ranking.
    #[serde(default)]
    pub items: Vec<PlayerRanking>,
}

impl Deref for PlayerLeaderboard {
    type Target = Vec<PlayerRanking>;

    fn deref(&self) -> &Vec<PlayerRanking> {
        &self.items
    }
}

#[cfg_attr(feature = "async", async_trait)]
impl PropLimFetchable for PlayerLeaderboard {
    type Property = str;
    type Limit = u8;

    /// (Sync) Fetches the top `limit` players in the regional (two-letter) `country_code`
    /// leaderboard (or global leaderboard, if `country_code == "global"`).
    fn fetch(client: &Client, country_code: &str, limit: u8) -> Result<PlayerLeaderboard> {
        let route = PlayerLeaderboard::get_route(country_code, limit);
        fetch_route::<PlayerLeaderboard>(client, &route)
    }

    /// (Async) Fetches the top `limit` players in the regional (two-letter) `country_code`
    /// leaderboard (or global leaderboard, if `country_code == "global"`).
    #[cfg(feature="async")]
    async fn a_fetch(client: &Client, country_code: &'async_trait str, limit: u8) -> Result<PlayerLeaderboard>
        where Self: 'async_trait,
              Self::Property: 'async_trait,
    {
        let route = PlayerLeaderboard::get_route(&country_code, limit);
        a_fetch_route::<PlayerLeaderboard>(client, &route).await
    }
}

impl PropLimRouteable for PlayerLeaderboard {
    type Property = str;
    type Limit = u8;

    /// Get the route for fetching the top `limit` players in the regional `country_code`
    /// leaderboard (or global, if `country_code == "global"`).
    fn get_route(country_code: &str, limit: u8) -> Route {
        Route::PlayerRankings {
            country_code: country_code.to_owned(),
            limit
        }
    }
}

/// Represents a player's ranking, based on a regional or global leaderboard.
/// To obtain the player's full data (a [`Player`] instance), see [`Player::fetch_from`].
///
/// [`Player`]: ../players/player/struct.Player.html
/// [`Player::fetch_from`]: ../players/player/struct.Player.html#method.fetch_from
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerRanking {
    /// The club the player is in - at the moment, only its name is available for this data model.
    /// To convert to a full [`Club`] object, see the [`PlayerRankingClub`] docs.
    ///
    /// [`Club`]: ../../clubs/struct.Club.html
    /// [`PlayerRankingClub`]: ./struct.PlayerRankingClub.html
    #[serde(default)]
    pub club: PlayerRankingClub,

    /// The player's tag.
    #[serde(default)]
    pub tag: String,

    /// The player's name.
    #[serde(default)]
    pub name: String,

    /// The player's trophies.
    #[serde(default)]
    pub trophies: usize,

    /// The player's rank in the leaderboard.
    #[serde(default = "one_default")]
    pub rank: u8,

    /// The player's name color. Defaults to `0xffffff` (white).
    #[serde(default = "oxffffff_default")]
    pub name_color: usize,
}

/// Represents the club in a player's ranking (a [`PlayerRanking`] object). Since the only data
/// available at the moment is its name, it cannot be converted into a full [`Club`] object
/// using a convenient method. For that, you must have the original `PlayerRanking` object,
/// convert it into a [`Player`] with [`Player::fetch_from`], then convert [`Player.club`] into
/// a `Club` with [`Club::fetch_from`] - this requires two calls to the API.
///
/// [`PlayerRanking`]: ./struct.PlayerRanking.html
/// [`Player`]: ../../players/player/struct.Player.html
/// [`Player::fetch_from`]: ../../players/player/struct.Player.html#method.fetch_from
/// [`Player.club`]: ../../players/player/struct.Player.html#structfield.club
/// [`Club`]: ../../clubs/struct.Club.html
/// [`Club::fetch_from`]: ../../clubs/struct.Club.html#method.fetch_from
#[non_exhaustive]
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlayerRankingClub {
    /// The club's name.
    #[serde(default)]
    pub name: String,
}

impl Default for PlayerRankingClub {
    fn default() -> PlayerRankingClub {
        PlayerRankingClub {
            name: String::from(""),
        }
    }
}
