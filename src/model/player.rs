use crate::model::clubs::ClubMember;
use crate::traits::{FetchFrom, Fetchable};
use crate::error::Error;

/// A struct representing a Brawl Stars player, with all of its data.
/// Use [Player::fetch] to fetch one based on tag.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Player {

    /// The club the Player is in (as a [PlayerClub] instance), or None if none.
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

impl Fetchable for Player {
    type Property = &'static str;

    fn fetch(tag: &str) -> Result<Player, Error> {
        // TODO: Implement TagFetchable for Player (be able to fetch a player)
    }
}

impl FetchFrom<ClubMember> for Player {
    fn fetch_from(member: ClubMember) -> Result<Player, Error> {
        Player::fetch(&member.tag)
    }
}

// TODO: Battle logs endpoint


/// A struct representing a club obtained from [Player.club].
/// Note that it does not contain all of a club's information.
/// For that, use [Club::fetch_from] (fetches the full Club).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlayerClub {

    /// The club's tag.
    pub tag: String,

    /// The club's name
    pub name: String
}

/// A struct containing information about a player's brawler (see [Player.brawlers]).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BrawlerStat {

    /// A vector containing the brawler's star powers (represented by [StarPower]),
    /// if any (otherwise empty vector).
    star_powers: Vec<StarPower>,

    /// The brawler's id (an arbitrary number).
    id: isize,

    /// The brawler's rank.
    rank: u16,

    /// The brawler's trophies.
    trophies: usize,

    /// The brawler's highest trophies amount.
    highest_trophies: usize,

    /// The brawler's power (1-10).
    power: u8,

    /// The brawler's name.
    name: String,
}

/// A struct representing a brawler's star power.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StarPower {

    /// The star power name.
    name: String,

    /// The star power's id (an arbitrary number).
    id: isize
}
