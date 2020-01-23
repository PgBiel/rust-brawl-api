#[cfg(feature = "async")]
use async_trait::async_trait;

use serde::{self, Serialize, Deserialize};

use crate::traits::{PropFetchable, FetchFrom, Initializable, GetFetchProp, PropRouteable};
use crate::error::Result;

#[cfg(feature = "players")]
use super::players::PlayerClub;
use crate::http::Client;
use crate::serde::deserialize_number_from_string;
use crate::http::routes::Route;
use crate::util::auto_hashtag;

/// The type of club (whether it's open, invite-only, or closed).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ClubType {
    Open,
    InviteOnly,
    Closed,
}

impl Default for ClubType {
    /// Defaults to [`ClubType::Open`].
    ///
    /// [`ClubType::Open`]: ./enum.ClubType.html#variant.Open
    fn default() -> ClubType { ClubType::Open }
}

/// A struct representing a Brawl Stars club, with all of its data.
/// Use [`Club::fetch`] to fetch one based on tag.
///
/// [`Club::fetch`]: ./struct.Club.html#method.fetch
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Club {

    /// The club's tag. **Note: this includes the initial '#'.**
    #[serde(default)]
    pub tag: String,

    /// The club's name.
    #[serde(default)]
    pub name: String,

    /// The club's description.
    #[serde(default)]
    pub description: Option<String>,

    /// The club's trophies.
    #[serde(default)]
    pub trophies: usize,

    /// The amount of trophies required to enter on this club, or 0 if it allows any amount.
    #[serde(default)]
    pub required_trophies: usize,

    /// The members in this club, as a vector of [`ClubMember`].
    ///
    /// [`ClubMember`]: ./struct.ClubMember.html
    #[serde(default)]
    pub members: Vec<ClubMember>,

    /// The type of club (see [`ClubType`] docs).
    ///
    /// [`ClubType`]: ./enum.ClubType.html
    #[serde(rename = "type")]
    #[serde(default)]
    pub club_type: ClubType
}

fn one_default_usize() -> usize { 1 }
fn false_default() -> bool { false }
fn oxffffff_default_usize() -> usize { 0xffffff }

impl Initializable for Club {

    /// Initializes a new Club instance, with default values.
    fn new() -> Club {
        Club {
            tag: String::from(""),
            name: String::from(""),
            description: None,
            trophies: 0,
            required_trophies: 0,
            members: vec![],
            club_type: ClubType::Open,
        }
    }
}

impl GetFetchProp for Club {
    type Property = String;

    fn get_fetch_prop(&self) -> &String { &self.tag }
}

impl PropRouteable for Club {
    type Property = String;

    fn get_route(tag: &String) -> Route { Route::Club(auto_hashtag(tag)) }
}  // PropFetchable is automatically implemented

#[cfg_attr(feature = "async", async_trait)]
#[cfg(feature = "players")]
impl FetchFrom<PlayerClub> for Club {
    fn fetch_from(client: &Client, p_club: PlayerClub) -> Result<Club> {
        Club::fetch(client, &p_club.tag)
    }

    #[cfg(feature = "async")]
    async fn a_fetch_from(client: &Client, p_club: PlayerClub) -> Result<Club> {
        Club::a_fetch(client, &p_club.tag).await
    }
}

/// An enum representing a member's possible roles (See [`ClubMember`]).
///
/// [`ClubMember`]: ./struct.ClubMember.html
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ClubMemberRole {
    Member,
    Senior,
    VicePresident,
    President,
}

impl Default for ClubMemberRole {
    /// Defaults to [`ClubMemberRole::Member`].
    ///
    /// [`ClubMemberRole::Member`]: ./enum.ClubMemberRole.html#variant.Member
    fn default() -> ClubMemberRole { ClubMemberRole::Member }
}

/// A struct representing a Brawl Stars club's member, with its club-relevant data
/// (most importantly, its role). Use [`Player::fetch_from`] to fetch the full player data.
///
/// [`ClubMember`]: ../players/struct.Player.html#method.fetch_from
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClubMember {

    /// The member's tag.
    #[serde(default)]
    pub tag: String,

    /// The member's name.
    #[serde(default)]
    pub name: String,

    /// The member's trophies.
    #[serde(default)]
    pub trophies: usize,

    /// The member's role in the guild. (Default is [`ClubMemberRole::Member`])
    ///
    /// [`ClubMemberRole::Member`]: ./enum.ClubMemberRole.html#variant.Member
    #[serde(default)]
    pub role: ClubMemberRole,

    /// The member's name color, as an integer (Default is 0xffffff = 16777215 - this is used
    /// when the data is not available).
    #[serde(default = "oxffffff_default_usize")]
    #[serde(deserialize_with = "deserialize_number_from_string")]  // parse num
    pub name_color: usize
}

impl Initializable for ClubMember {
    fn new() -> ClubMember {
        ClubMember {
            tag: String::from(""),
            name: String::from(""),
            trophies: 0,
            role: ClubMemberRole::Member,
            name_color: 0xffffff
        }
    }
}

// TODO: clubs/<tag>/members endpoint
