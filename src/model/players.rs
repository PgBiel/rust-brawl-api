//! Models for the 'players/' API endpoint.
//! Included by the feature 'players'; removing that feature will disable the usage of this module.

use serde::{self, Serialize, Deserialize};
use reqwest::blocking::{
    Response, RequestBuilder
};

#[cfg(feature = "async")]
use reqwest::{
    Response as AResponse
};

#[cfg(feature = "async")]
use async_trait::async_trait;

use crate::traits::{FetchFrom, PropFetchable, Initializable, GetFetchProp, PropRouteable};
use crate::error::{Result, Error};

#[cfg(feature = "clubs")]
use super::clubs::ClubMember;
use crate::http::Client;
use crate::http::routes::Route;
use crate::util::auto_hashtag;
use crate::serde::deserialize_number_from_string;
use reqwest::StatusCode;

/// A struct representing a Brawl Stars player, with all of its data.
/// Use [`Player::fetch`] to fetch one based on tag.
///
/// [`Player::fetch`]: ./struct.Player.html#method.fetch
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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
    #[serde(default = "one_default_usize")]
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
    pub brawlers: Vec<BrawlerStat>,

    /// The player's name color, as an integer (Default is 0xffffff = 16777215 - this is used
    /// when the data is not available).
    #[serde(default = "oxffffff_default_usize")]
    #[serde(deserialize_with = "deserialize_number_from_string")]  // parse num
    pub name_color: usize,
}

fn one_default_usize() -> usize { 1 }
fn false_default() -> bool { false }
fn oxffffff_default_usize() -> usize { 0xffffff }

impl Initializable for Player {

    /// Initializes a Player instance with default values for each field.
    fn new() -> Player {
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

            brawlers: Vec::<BrawlerStat>::new(),

            name_color: 0xffffff,
        }
    }
}

impl GetFetchProp for Player {
    type Property = String;

    fn get_fetch_prop(&self) -> &String { &self.tag }
}

impl PropRouteable for Player {
    type Property = String;

    fn get_route(tag: &String) -> Route { Route::Player(auto_hashtag(tag)) }
}  // PropFetchable is automatically implemented

#[cfg_attr(feature = "async", async_trait)]
#[cfg(feature = "clubs")]
impl FetchFrom<ClubMember> for Player {
    fn fetch_from(client: &Client, member: ClubMember) -> Result<Player> {
        Player::fetch(client, &member.tag)
    }

    #[cfg(feature = "async")]
    async fn a_fetch_from(client: &Client, member: ClubMember) -> Result<Player> {
        Player::a_fetch(client, &member.tag).await
    }
}

// TODO: Battle logs endpoint


/// A struct representing a club obtained from [`Player.club`].
/// Note that it does not contain all of a club's information.
/// For that, use [`Club::fetch_from`] (fetches the full Club).
///
/// [`Player.club`]: ./struct.Player.html#structfield.club
/// [`Club::fetch_from`]: ../clubs/struct.Club.html#method.fetch_from
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlayerClub {

    /// The club's tag.
    #[serde(default)]
    pub tag: String,

    /// The club's name
    #[serde(default)]
    pub name: String
}

impl Initializable for PlayerClub {

    /// Initializes a new PlayerClub instance, with default values.
    fn new() -> PlayerClub {
        PlayerClub {
            tag: String::from(""),
            name: String::from("")
        }
    }
}

/// A struct containing information about a player's brawler (see [`Player.brawlers`]).
///
/// [`Player.brawlers`]: ./struct.Player.html#structfield.brawlers
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrawlerStat {

    /// A vector containing the brawler's star powers (represented by [`StarPower`]),
    /// if any (otherwise empty vector).
    ///
    /// [`StarPower`]: ./struct.StarPower.html
    #[serde(default)]
    pub star_powers: Vec<StarPower>,

    /// The brawler's id (an arbitrary number).
    #[serde(default)]  // zero
    pub id: isize,

    /// The brawler's rank.
    #[serde(default = "one_default_usize")]
    pub rank: u16,

    /// The brawler's trophies.
    #[serde(default)]
    pub trophies: usize,

    /// The brawler's highest trophies amount.
    #[serde(default)]  // zero
    pub highest_trophies: usize,

    /// The brawler's power (1-10).
    #[serde(default = "one_default_usize")]
    pub power: u8,

    /// The brawler's name.
    #[serde(default)]
    pub name: String,
}

impl Initializable for BrawlerStat {

    /// Initializes a new BrawlerStat instance, with default values.
    fn new() -> BrawlerStat {
        BrawlerStat {
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
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StarPower {

    /// The star power name.
    #[serde(default)]
    pub name: String,

    /// The star power's id (an arbitrary number).
    #[serde(default)]
    pub id: isize
}

impl Initializable for StarPower {

    /// Initializes a new StarPower instance, with default values.
    fn new() -> StarPower {
        StarPower {
            name: String::from(""),
            id: 0
        }
    }
}
