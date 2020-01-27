//! Models for the `players/:tag` API endpoint.
//! Included by the feature `"players"`; removing that feature will disable the usage of this module.

use serde::{self, Serialize, Deserialize};


#[cfg(feature = "async")]


#[cfg(feature = "async")]
use crate::util::a_fetch_route;

#[cfg(feature = "async")]
use async_trait::async_trait;

use crate::traits::{FetchFrom, PropFetchable, GetFetchProp};
use crate::error::{Result};

#[cfg(feature = "clubs")]
use super::super::clubs::ClubMember;

use crate::http::Client;
use crate::http::routes::Route;
use crate::util::{auto_hashtag, fetch_route};
use crate::serde::{deserialize_number_from_string, one_default, oxffffff_default};



use crate::model::players::battlelog::{BattlePlayer};

#[cfg(feature = "rankings")]
use crate::PlayerRanking;

/// A struct representing a Brawl Stars player, with all of its data.
/// Use [`Player::fetch`] to fetch one based on tag. (Make sure the [`PropFetchable`] trait
/// is imported - in general, it is recommended to **at least** `use brawl_api::traits::*`, or,
/// even, `use brawl_api::prelude::*` to bring the models into scope as well.)
///
/// [`PropFetchable`]: traits/trait.PropFetchable.html
/// [`Player::fetch`]: ./struct.Player.html#method.fetch
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Player {

    /// The club the Player is in (as a [`PlayerClub`] instance), or None if none.
    ///
    /// [`PlayerClub`]: ./struct.PlayerClub.html
    #[serde(default)]
    pub club: Option<PlayerClub>,

    /// Whether or not the Player was qualified from the Championship challenge (2020).
    #[serde(default = "false_default")]
    pub is_qualified_from_championship_challenge: bool,

    /// Amount of 3v3 victories the Player has earned.
    #[serde(rename = "3vs3Victories")]
    pub tvt_victories: usize,

    /// The player's tag. **Note: this includes the initial '#'.**
    #[serde(default)]
    pub tag: String,

    /// The player's name.
    #[serde(default)]
    pub name: String,

    /// The player's current trophies.
    #[serde(default)]  // zero
    pub trophies: usize,

    /// The player's highest trophies amount.
    #[serde(default)]  // zero
    pub highest_trophies: usize,

    /// The player's experience level.
    #[serde(default = "one_default")]
    pub exp_level: usize,

    /// The player's experience points.
    #[serde(default)]  // zero
    pub exp_points: usize,

    /// The player's current power play points.
    #[serde(default)]  // zero
    pub power_play_points: usize,

    /// The player's highest power play points.
    #[serde(default)]  // zero
    pub highest_power_play_points: usize,

    /// The player's victories in solo showdown (how many times ranked #1).
    #[serde(default)]  // zero
    pub solo_victories: usize,

    /// The player's victories in duo showdown (how many times ranked #1).
    #[serde(default)]  // zero
    pub duo_victories: usize,

    /// The player's best Robo Rumble time, in seconds.
    #[serde(default)]  // zero
    pub best_robo_rumble_time: usize,

    /// The player's best time as a Big Brawler, in seconds.
    #[serde(default)]  // zero
    pub best_time_as_big_brawler: usize,

    /// The player's brawlers.
    #[serde(default)]
    pub brawlers: Vec<PlayerBrawlerStat>,

    /// The player's name color, as an integer (Default is 0xffffff = 16777215 - this is used
    /// when the data is not available).
    #[serde(default = "oxffffff_default")]
    #[serde(deserialize_with = "deserialize_number_from_string")]  // parse num
    pub name_color: usize,
}
fn false_default() -> bool { false }

impl Default for Player {
    
    /// Initializes a Player instance with default values for each field.
    fn default() -> Player {
        Player {
            club: None,

            is_qualified_from_championship_challenge: false,

            tvt_victories: 0,

            tag: String::from(""),

            name: String::from(""),

            trophies: 0,

            highest_trophies: 0,

            exp_level: 1,

            exp_points: 0,

            power_play_points: 0,

            highest_power_play_points: 0,

            solo_victories: 0,

            duo_victories: 0,

            best_robo_rumble_time: 0,

            best_time_as_big_brawler: 0,

            brawlers: Vec::<PlayerBrawlerStat>::new(),

            name_color: 0xff_ff_ff,
        }
    }
}

impl GetFetchProp for Player {
    type Property = str;

    fn get_fetch_prop(&self) -> &str { &*self.tag }

    fn get_route(tag: &str) -> Route { Route::Player(auto_hashtag(tag)) }
}

#[cfg_attr(feature = "async", async_trait)]
impl PropFetchable for Player {
    type Property = str;

    /// (Sync) Fetches a player from its tag.
    ///
    /// # Errors
    ///
    /// This function may error:
    /// - While requesting (will return an [`Error::Request`]);
    /// - After receiving a bad status code (API or other error - returns an [`Error::Status`]);
    /// - After a ratelimit is indicated by the API, while also specifying when it is lifted ([`Error::Ratelimited`]);
    /// - While parsing incoming JSON (will return an [`Error::Json`]).
    ///
    /// (All of those, of course, wrapped inside an `Err`.)
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use brawl_api::{Client, Player, traits::*};
    ///
    /// # fn main() -> Result<(), Box<dyn ::std::error::Error>> {
    /// let my_client = Client::new("my auth token");
    /// let player = Player::fetch(&my_client, "#PLAYERTAGHERE")?;
    /// // now you have data for the given player.
    ///
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`Error::Request`]: error/enum.Error.html#variant.Request
    /// [`Error::Status`]: error/enum.Error.html#variant.Status
    /// [`Error::Ratelimited`]: error/enum.Error.html#variant.Ratelimited
    /// [`Error::Json`]: error/enum.Error.html#variant.Json
    fn fetch(client: &Client, tag: &str) -> Result<Player> {
        let route = Self::get_route(tag);
        fetch_route::<Player>(client, &route)
    }

    /// (Async) Fetches a player from its tag.
    ///
    /// # Errors
    ///
    /// This function may error:
    /// - While requesting (will return an [`Error::Request`]);
    /// - After receiving a bad status code (API or other error - returns an [`Error::Status`]);
    /// - After a ratelimit is indicated by the API, while also specifying when it is lifted ([`Error::Ratelimited`]);
    /// - While parsing incoming JSON (will return an [`Error::Json`]).
    ///
    /// (All of those, of course, wrapped inside an `Err`.)
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use brawl_api::{Client, Player, traits::*};
    ///
    /// # async fn main() -> Result<(), Box<dyn ::std::error::Error>> {
    /// let my_client = Client::new("my auth token");
    /// let player = Player::a_fetch(&my_client, "#PLAYERTAGHERE").await?;
    /// // now you have data for the given player.
    ///
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`Error::Request`]: error/enum.Error.html#variant.Request
    /// [`Error::Status`]: error/enum.Error.html#variant.Status
    /// [`Error::Ratelimited`]: error/enum.Error.html#variant.Ratelimited
    /// [`Error::Json`]: error/enum.Error.html#variant.Json
    #[cfg(feature="async")]
    async fn a_fetch(client: &Client, tag: &'async_trait str) -> Result<Player>
        where Self: 'async_trait,
              Self::Property: 'async_trait,
    {
        let route = Player::get_route(&tag);
        a_fetch_route::<Player>(client, &route).await
    }
}

#[cfg_attr(feature = "async", async_trait)]
#[cfg(feature = "clubs")]
impl FetchFrom<ClubMember> for Player {
    /// (Sync) Fetches a `Player` instance, given a preexisting `ClubMember` instance.
    ///
    /// # Errors
    ///
    /// See [`Player::fetch`].
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use brawl_api::{Client, Player, Club, traits::*};
    ///
    /// # fn main() -> Result<(), Box<dyn ::std::error::Error>> {
    /// let my_client = Client::new("my auth token");
    /// let club = Club::fetch(&my_client, "#CLUB_TAG_HERE")?;
    /// let some_member = &club.members[0];
    /// let some_player = Player::fetch_from(&my_client, some_member)?;
    /// // now `some_member`'s full data, as a Player, is available for use.
    ///
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`Player::fetch`]: struct.Player.html#method.fetch
    fn fetch_from(client: &Client, member: &ClubMember) -> Result<Player> {
        Player::fetch(client, &member.tag)
    }

    /// (Async) Fetches a `Player` instance, given a preexisting `ClubMember` instance.
    ///
    /// # Errors
    ///
    /// See [`Player::fetch`].
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use brawl_api::{Client, Player, Club, traits::*};
    ///
    /// # async fn main() -> Result<(), Box<dyn ::std::error::Error>> {
    /// let my_client = Client::new("my auth token");
    /// let club = Club::a_fetch(&my_client, "#CLUB_TAG_HERE").await?;
    /// let some_member = &club.members[0];
    /// let some_player = Player::a_fetch_from(&my_client, some_member).await?;
    /// // now `some_member`'s full data, as a Player, is available for use.
    ///
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`Player::fetch`]: struct.Player.html#method.fetch
    #[cfg(feature = "async")]
    async fn a_fetch_from(client: &Client, member: &ClubMember) -> Result<Player> {
        Player::a_fetch(client, &member.tag).await
    }
}

#[cfg_attr(feature = "async", async_trait)]
impl FetchFrom<BattlePlayer> for Player {
    /// (Async) Fetches a `Player` instance, given a preexisting `BattlePlayer` instance.
    ///
    /// # Errors
    ///
    /// See [`Player::fetch`].
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use brawl_api::{
    ///     Client, Player, BattleLog, Battle, BattleResultInfo, BattlePlayer,
    ///     traits::*
    /// };
    ///
    /// # fn main() -> Result<(), Box<dyn ::std::error::Error>> {
    /// let my_client = Client::new("my auth token");
    /// let battlelog = BattleLog::fetch(&my_client, "#PLAYER_TAG_HERE")?;
    /// let most_recent_battle: Option<&Battle> = battlelog.get(0);
    ///
    /// if let Some(battle) = most_recent_battle {
    ///     if let Some(ref teams) = &battle.result.teams {
    ///         let some_b_player: &BattlePlayer = &teams[0][0];
    ///         let some_player = Player::fetch_from(&my_client, some_b_player)?;
    ///         // now `some_b_player`'s full data, as a Player, is available for use.
    ///     }
    /// }
    ///
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`Player::fetch`]: struct.Player.html#method.fetch
    fn fetch_from(client: &Client, b_player: &BattlePlayer) -> Result<Player> {
        Player::fetch(client, &b_player.tag)
    }

    /// (Async) Fetches a `Player` instance, given a preexisting `BattlePlayer` instance.
    ///
    /// # Errors
    ///
    /// See [`Player::fetch`].
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use brawl_api::{
    ///     Client, Player, BattleLog, Battle, BattleResultInfo, BattlePlayer,
    ///     traits::*
    /// };
    ///
    /// # async fn main() -> Result<(), Box<dyn ::std::error::Error>> {
    /// let my_client = Client::new("my auth token");
    /// let battlelog = BattleLog::a_fetch(&my_client, "#PLAYER_TAG_HERE").await?;
    /// let most_recent_battle: Option<&Battle> = battlelog.get(0);
    ///
    /// if let Some(battle) = most_recent_battle {
    ///     if let Some(ref teams) = &battle.result.teams {
    ///         let some_b_player: &BattlePlayer = &teams[0][0];
    ///         let some_player = Player::a_fetch_from(&my_client, some_b_player).await?;
    ///         // now `some_b_player`'s full data, as a Player, is available for use.
    ///     }
    /// }
    ///
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`Player::fetch`]: struct.Player.html#method.fetch
    #[cfg(feature = "async")]
    async fn a_fetch_from(client: &Client, b_player: &BattlePlayer) -> Result<Player> {
        Player::a_fetch(client, &b_player.tag).await
    }
}

#[cfg_attr(feature = "async", async_trait)]
#[cfg(feature = "rankings")]
impl FetchFrom<PlayerRanking> for Player {

    /// (Sync) Fetches a `Player` using data from a [`PlayerRanking`] object.
    ///
    /// [`PlayerRanking`]: ../../rankings/players/struct.PlayerRanking.html
    fn fetch_from(client: &Client, p_ranking: &PlayerRanking) -> Result<Player> {
        Player::fetch(client, &p_ranking.tag)
    }

    /// (Async) Fetches a `Player` using data from a [`PlayerRanking`] object.
    ///
    /// [`PlayerRanking`]: ../../rankings/players/struct.PlayerRanking.html
    #[cfg(feature = "async")]
    async fn a_fetch_from(client: &Client, p_ranking: &PlayerRanking) -> Result<Player> {
        Player::a_fetch(client, &p_ranking.tag).await
    }
}


/// A struct representing a club obtained from [`Player.club`].
/// Note that it does not contain all of a club's information.
/// For that, use [`Club::fetch_from`] (fetches the full Club).
///
/// [`Player.club`]: ./struct.Player.html#structfield.club
/// [`Club::fetch_from`]: ../clubs/struct.Club.html#method.fetch_from
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlayerClub {

    /// The club's tag.
    #[serde(default)]
    pub tag: String,

    /// The club's name
    #[serde(default)]
    pub name: String
}

impl Default for PlayerClub {

    /// Returns an instance of `PlayerClub` with initial values.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use brawl_api::PlayerClub;
    ///
    /// assert_eq!(
    ///     PlayerClub::default(),
    ///     PlayerClub {
    ///         tag: String::from(""),
    ///         name: String::from(""),
    ///     }
    /// );
    /// ```
    fn default() -> PlayerClub {
        PlayerClub {
            tag: String::from(""),
            name: String::from("")
        }
    }
}

/// A struct containing information about a player's brawler (see [`Player.brawlers`]).
///
/// [`Player.brawlers`]: ./struct.Player.html#structfield.brawlers
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerBrawlerStat {

    /// A vector containing the brawler's star powers (represented by [`StarPower`]),
    /// if any (otherwise empty vector).
    ///
    /// [`StarPower`]: ./struct.StarPower.html
    #[serde(default)]
    pub star_powers: Vec<StarPower>,

    /// The brawler's id (an arbitrary number).
    #[serde(default)]  // zero
    pub id: usize,

    /// The brawler's rank.
    #[serde(default = "one_default")]
    pub rank: u16,

    /// The brawler's trophies.
    #[serde(default)]
    pub trophies: usize,

    /// The brawler's highest trophies amount.
    #[serde(default)]  // zero
    pub highest_trophies: usize,

    /// The brawler's power (1-10).
    #[serde(default = "one_default")]
    pub power: u8,

    /// The brawler's name.
    #[serde(default)]
    pub name: String,
}

impl Default for PlayerBrawlerStat {
    
    /// Initializes a new BrawlerStat instance, with default values.
    fn default() -> PlayerBrawlerStat {
        PlayerBrawlerStat {
            star_powers: vec![],
            id: 0,
            rank: 1,
            trophies: 0,
            highest_trophies: 0,
            power: 1,
            name: String::from(""),
        }
    }
}

/// A struct representing a brawler's star power.
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct StarPower {

    /// The star power name.
    #[serde(default)]
    pub name: String,

    /// The star power's id (an arbitrary number).
    #[serde(default)]
    pub id: isize
}

impl Default for StarPower {

    /// Returns an instance of `StarPower` with initial values.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use brawl_api::StarPower;
    ///
    /// assert_eq!(
    ///     StarPower::default(),
    ///     StarPower {
    ///         name: String::from(""),
    ///         id: 0,
    ///     }
    /// );
    /// ```
    fn default() -> StarPower {
        StarPower {
            name: String::from(""),
            id: 0
        }
    }
}
