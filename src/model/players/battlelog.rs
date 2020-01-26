use std::ops::Deref;
use crate::traits::{GetFetchProp, PropFetchable};
use crate::http::routes::Route;
use crate::util::{fetch_route, a_fetch_route};
use serde::{self, Serialize, Deserialize};
use crate::error::Result;
use crate::serde::one_default;

#[cfg(feature = "async")]
use async_trait::async_trait;
use crate::http::Client;

// region:BattleLog

/// Represents a list of a Player's most recent battles.
/// (NOTE: It may take up to 30 minutes for a new battle to appear in the battlelog.)
///
/// Use [`BattleLog::fetch`] to fetch the battle logs for a given player tag.
/// One may also [`BattleLog::fetch_from`] with an existing [`Player`] instance in order to use its
/// tag.
///
/// [`BattleLog::fetch`]: #method.fetch
/// [`BattleLog::fetch_from`]: #method.fetch_from
/// [`Player`]: model/players/player/struct.Player.html
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct BattleLog {
    /// The tag of the player whose BattleLog (most recent battles) was fetched.
    #[serde(skip)]  // artificial
    pub tag: String,

    /// The items (battles) of this battle log.
    #[serde(default)]
    pub items: Vec<Battle>
}

impl Deref for BattleLog {
    type Target = Vec<Battle>;

    fn deref(&self) -> &Vec<Battle> {
        &self.items
    }
}

impl GetFetchProp for BattleLog {
    type Property = String;

    fn get_fetch_prop(&self) -> &String {
        &self.tag
    }

    fn get_route(tag: &String) -> Route {
        Route::PlayerBattlelogs(tag.clone())
    }
}

#[cfg_attr(feature = "async", async_trait)]
impl PropFetchable for BattleLog {
    type Property = String;

    /// (Sync) Fetches a player's battlelog (most recent battles).
    fn fetch(client: &Client, tag: &String) -> Result<BattleLog> {
        let route = Self::get_route(&tag);
        let mut battle_log = fetch_route::<BattleLog>(client, &route)?;
        battle_log.tag = tag.clone();
        Ok(battle_log)
    }

    /// (Async) Fetches a player's battlelog (most recent battles).
    #[cfg(feature="async")]
    async fn a_fetch(client: &Client, tag: &'async_trait String) -> Result<BattleLog>
        where Self: 'async_trait,
              Self::Property: 'async_trait,
    {
        let route = BattleLog::get_route(&tag);
        let mut battle_log = a_fetch_route::<BattleLog>(client, &route).await?;
        battle_log.tag = tag.clone();
        Ok(battle_log)
    }
}

// endregion:BattleLog

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Battle {
    /// The time at which this battle occurred, in ISO format.
    #[serde(default)]
    pub battle_time: String,  // TODO: Chrono

    /// Data about the event in which this battle occurred.
    #[serde(default)]
    pub event: BattleEvent,

    /// Data about the battle itself and its outcome.
    #[serde(default)]
    pub result: BattleResultInfo,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct BattleEvent {
    /// The id of the event (an arbitrary number).
    #[serde(default)]
    pub id: isize,

    /// The event mode (e.g. "brawlBall", "soloShowdown"...).
    #[serde(default)]
    pub mode: String,

    /// The name of the map where this battle happened.
    #[serde(default)]
    pub map: String,
}

impl BattleEvent {
    /// Returns a default BattleEvent - see [`BattleEvent::default`].
    ///
    /// [`BattleEvent::default`]: #method.default
    pub fn new() -> BattleEvent { BattleEvent::default() }
}

impl Default for BattleEvent {
    /// Returns a default BattleEvent, with all default values initialized.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use brawl_api::BattleEvent;
    ///
    /// assert_eq!(
    ///     BattleEvent::default(),
    ///     BattleEvent { id: 0, mode: String::from(""), map: String::from("") }
    /// )
    /// ```
    fn default() -> BattleEvent {
        BattleEvent {
            id: 0,
            mode: String::from(""),
            map: String::from(""),
        }
    }
}

#[non_exhaustive]
#[derive(Debug, Clone, Hash, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum BattleOutcome {
    Victory,
    Defeat,
    Draw,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BattleResultInfo {
    /// The event mode (e.g. "brawlBall", "soloShowdown"...). Should be the same as [`BattleEvent.mode`].
    ///
    /// [`BattleEvent.mode`]: ./struct.BattleEvent.html#structfield.mode
    #[serde(default)]
    pub mode: String,

    /// The type of battle (e.g. "ranked").
    #[serde(default)]
    #[serde(rename = "type")]
    pub battle_type: String,

    /// The duration of this battle, in seconds.
    #[serde(default)]
    pub duration: usize,

    /// The difference in trophies applied to the player after the battle. E.g. -4 (lost 4 trophies)
    #[serde(default)]
    pub trophy_change: usize,

    /// If this was a solo mode match, then this is the player's final rank (1-10). Otherwise,
    /// `None`.
    #[serde(default)]
    pub rank: Option<u8>,

    /// If this was a match with teams, then this is the outcome for the player
    /// (Victory/Defeat/Draw), otherwise `None`.
    #[serde(default)]
    pub result: Option<BattleOutcome>,

    /// The data indicating who was the Star Player in the match. This is generally from the
    /// winning team, unless a draw occurred, in which case it can be from either team.
    /// If this was a solo mode or boss fight match, for instance, then there is no star player
    /// (None).
    #[serde(default)]
    pub star_player: Option<BattlePlayer>,

    /// If this was a match with teams, then this is an array with both teams of players
    /// (as vectors).
    /// Otherwise, `None`.
    #[serde(default)]
    pub teams: Option<[Vec<BattlePlayer>; 2]>,

    /// If this was a solo match or a mode without teams, such as Showdown, then this is a vector
    /// with all the players in the match. Otherwise, `None`.
    #[serde(default)]
    pub players: Option<Vec<BattlePlayer>>
}

impl Default for BattleResultInfo {
    fn default() -> BattleResultInfo {
        BattleResultInfo {
            mode: String::from(""),
            battle_type: String::from(""),
            duration: 0,
            trophy_change: 0,
            rank: None,
            star_player: None,
            result: None,
            teams: None,
            players: None,
        }
    }
}

/// Represents a player in a [`BattleResultInfo`] object, with only partial data about it (note that
/// the `brawler` field is exclusive to this struct, representing the brawler the player was using
/// during the battle).
/// One can use [`Player::fetch_from`] to obtain a full [`Player`] instance from an existing
/// `BattlePlayer` instance.
///
/// [`BattleResultInfo`]: ./struct.BattleResult.html
/// [`Player`]: ../player/struct.Player.html
/// [`Player::fetch_from`]: ../player/struct.Player.html#method.fetch_from
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct BattlePlayer {
    /// The player's tag.
    #[serde(default)]
    pub tag: String,

    /// The player's name.
    #[serde(default)]
    pub name: String,

    /// The brawler the player was using during the battle.
    #[serde(default)]
    pub brawler: BattleBrawler,
}

impl Default for BattlePlayer {
    fn default() -> BattlePlayer {
        BattlePlayer {
            tag: String::from(""),
            name: String::from(""),
            brawler: BattleBrawler::default(),
        }
    }
}

/// Represents the brawler a player was using in a [`BattlePlayer`] object.
///
/// [`BattlePlayer`]: ./struct.BattlePlayer.html
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct BattleBrawler {
    /// The brawler's id (an arbitrary number).
    #[serde(default)]
    pub id: isize,

    /// The brawler's name (e.g. "PENNY", "ROSA", "BROCK"...)
    #[serde(default)]
    pub name: String,

    /// The brawler's power (1-10).
    #[serde(default = "one_default")]
    pub power: u8,

    #[serde(default)]
    pub trophies: usize,
}

impl Default for BattleBrawler {
    fn default() -> BattleBrawler {
        BattleBrawler {
            id: 0,
            name: String::from(""),
            power: 1,
            trophies: 0,
        }
    }
}
