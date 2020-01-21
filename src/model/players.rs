//! Models for the 'players/' API endpoint.
//! Included by the feature 'players'; removing that feature will disable the usage of this module.

#![warn(missing_docs)]

#[cfg(feature = "async")]
use async_trait::async_trait;
use crate::traits::{FetchFrom, PropFetchable, Initializable};
use crate::error::{Result, Error};
use crate::macros::b_api_concat;

#[cfg(feature = "clubs")]
use crate::model::clubs::ClubMember;

/// A struct representing a Brawl Stars player, with all of its data.
/// Use [`Player::fetch`] to fetch one based on tag.
///
/// [`Player::fetch`]: ./struct.Player.html#method.fetch
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Player {

    /// The club the Player is in (as a [`PlayerClub`] instance), or None if none.
    ///
    /// [`PlayerClub`]: ./struct.PlayerClub.html
    pub club: Option<PlayerClub>,

    /// Whether or not the Player was qualified from the Championship challenge (2020).
    pub is_qualified_from_championship_challenge: bool,

    /// Amount of 3v3 victories the Player has earned.
    pub tvt_victories: usize,

    /// The player's tag.
    pub tag: String,

    /// The player's name.
    pub name: String,

    /// The player's current trophies.
    pub trophies: usize,

    /// The player's highest trophies amount.
    pub highest_trophies: usize,

    /// The player's experience level.
    pub exp_level: usize,

    /// The player's experience points.
    pub exp_points: usize,

    /// The player's current power play points.
    pub power_play_points: usize,

    /// The player's highest power play points.
    pub highest_power_play_points: usize,

    /// The player's victories in solo showdown (how many times ranked #1).
    pub solo_victories: usize,

    /// The player's victories in duo showdown (how many times ranked #1).
    pub duo_victories: usize,

    /// The player's best Robo Rumble time, in seconds.
    pub best_robo_rumble_time: usize,

    /// The player's best time as a Big Brawler, in seconds.
    pub best_time_as_big_brawler: usize,

    /// The player's brawlers.
    pub brawlers: Vec<BrawlerStat>,

    /// The player's name color, as an integer (Default is 0xffffff = 16777215 - this is used
    /// when the data is not available).
    pub name_color: usize,
}

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

#[cfg_attr(feature = "async", async_trait)]
impl<'a> PropFetchable for Player {
    type Property = &'a str;

    /// (Sync) Fetches a Player through its tag.
    fn fetch(tag: &'a str) -> Result<Player> {
        // TODO: Implement TagFetchable for Player (be able to fetch a player)
    }

    /// (Async) Fetches a player through its tag.
    #[cfg(feature = "async")]
    async fn a_fetch(tag: &'a str) -> Result<Player> {

    }

    fn get_fetch_prop(&self) -> &'a str {
        &*self.tag
    }
}

#[cfg_attr(feature = "async", async_trait)]
#[cfg(feature = "clubs")]
impl FetchFrom<ClubMember> for Player {
    fn fetch_from(member: ClubMember) -> Result<Player> {
        Player::fetch(&member.tag)
    }

    #[cfg(feature = "async")]
    async fn a_fetch_from(member: ClubMember) -> Result<Player> {
        Player::a_fetch(&member.tag).await
    }
}

// TODO: Battle logs endpoint


/// A struct representing a club obtained from [`Player.club`].
/// Note that it does not contain all of a club's information.
/// For that, use [`Club::fetch_from`] (fetches the full Club).
///
/// [`Player.club`]: ./struct.Player.html#structfield.club
/// [`Club::fetch_from`]: ../clubs/struct.Club.html#method.fetch_from
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlayerClub {

    /// The club's tag.
    pub tag: String,

    /// The club's name
    pub name: String
}

/// A struct containing information about a player's brawler (see [`Player.brawlers`]).
///
/// [`Player.brawlers`]: ./struct.Player.html#structfield.brawlers
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BrawlerStat {

    /// A vector containing the brawler's star powers (represented by [`StarPower`]),
    /// if any (otherwise empty vector).
    ///
    /// [`StarPower`]: ./struct.StarPower.html
    pub star_powers: Vec<StarPower>,

    /// The brawler's id (an arbitrary number).
    pub id: isize,

    /// The brawler's rank.
    pub rank: u16,

    /// The brawler's trophies.
    pub trophies: usize,

    /// The brawler's highest trophies amount.
    pub highest_trophies: usize,

    /// The brawler's power (1-10).
    pub power: u8,

    /// The brawler's name.
    pub name: String,
}

/// A struct representing a brawler's star power.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StarPower {

    /// The star power name.
    pub name: String,

    /// The star power's id (an arbitrary number).
    pub id: isize
}
