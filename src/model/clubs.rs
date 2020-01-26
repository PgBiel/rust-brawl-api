#[cfg(feature = "async")]
use async_trait::async_trait;

use serde::{self, Serialize, Deserialize};

use crate::traits::{PropFetchable, FetchFrom, GetFetchProp};
use crate::error::Result;

#[cfg(feature = "async")]
use crate::util::a_fetch_route;

#[cfg(feature = "players")]
use super::players::PlayerClub;
use crate::http::Client;
use crate::serde::deserialize_number_from_string;
use crate::http::routes::Route;
use crate::util::{auto_hashtag, fetch_route};

use std::fmt::{Display, Formatter};
use crate::model::rankings::ClubRanking;
use std::cmp::Ordering;

/// The type of club (whether it's open, invite-only, or closed).
#[non_exhaustive]
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
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

impl Display for ClubType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(
            f, "{}",
            match self {
                ClubType::Open => "Open",
                ClubType::InviteOnly => "InviteOnly",
                ClubType::Closed => "Closed",
            }
        )
    }
}

/// A struct representing a Brawl Stars club, with all of its data.
/// Use [`Club::fetch`] to fetch one based on tag.
///
/// [`Club::fetch`]: ./struct.Club.html#method.fetch
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
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

fn oxffffff_default_usize() -> usize { 0xff_ff_ff }

impl Default for Club {
    

    /// Initializes a new Club instance, with default values.
    fn default() -> Club {
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

    fn get_route(tag: &String) -> Route { Route::Club(auto_hashtag(tag)) }
}

#[cfg_attr(feature = "async", async_trait)]
impl PropFetchable for Club {
    type Property = String;

    /// (Sync) Fetches a club from its tag.
    fn fetch(client: &Client, tag: &String) -> Result<Club> {
        let route = Club::get_route(&tag);
        fetch_route::<Club>(client, &route)
    }

    /// (Async) Fetches a club from its tag.
    #[cfg(feature="async")]
    async fn a_fetch(client: &Client, tag: &'async_trait String) -> Result<Club>
        where Self: 'async_trait,
              Self::Property: 'async_trait,
    {
        let route = Club::get_route(&tag);
        a_fetch_route::<Club>(client, &route).await
    }
}

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

#[cfg_attr(feature = "async", async_trait)]
#[cfg(feature = "rankings")]
impl FetchFrom<ClubRanking> for Club {

    /// (Sync) Fetches a `Club` using data from a [`ClubRanking`] object.
    ///
    /// [`ClubRanking`]: ../../rankings/clubs/struct.ClubRanking.html
    fn fetch_from(client: &Client, c_ranking: ClubRanking) -> Result<Club> {
        Club::fetch(client, &c_ranking.tag)
    }

    /// (Async) Fetches a `Club` using data from a [`ClubRanking`] object.
    ///
    /// [`ClubRanking`]: ../../rankings/clubs/struct.ClubRanking.html
    #[cfg(feature = "async")]
    async fn a_fetch_from(client: &Client, c_ranking: ClubRanking) -> Result<Club> {
        Club::a_fetch(client, &c_ranking.tag).await
    }
}

/// An enum representing a member's possible roles (See [`ClubMember`]).
///
/// [`ClubMember`]: ./struct.ClubMember.html
#[non_exhaustive]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ClubMemberRole {
    Member = 0,
    Senior = 1,
    VicePresident = 2,
    President = 3,
}

impl Display for ClubMemberRole {
    fn fmt(&self, f: &mut Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f, "{}",
            match *self {
                ClubMemberRole::Member => "Member",
                ClubMemberRole::Senior => "Senior",
                ClubMemberRole::VicePresident => "VicePresident",
                ClubMemberRole::President => "President",
            }
        )
    }
}

impl PartialOrd for ClubMemberRole {
    /// Compares and determines which `ClubMemberRole` is higher in the hierarchy:
    /// `Member < Senior < VicePresident < President`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use brawl_api::ClubMemberRole;
    ///
    /// // vice-president has more power (is higher in the hierarchy) than a normal Member
    /// assert!(ClubMemberRole::Member < ClubMemberRole::VicePresident);
    /// assert!(ClubMemberRole::President > ClubMemberRole::VicePresident);
    /// assert!(ClubMemberRole::Senior > ClubMemberRole::Member);
    /// assert!(ClubMemberRole::Member >= ClubMemberRole::Member);
    /// ```
    fn partial_cmp(&self, other: &ClubMemberRole) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ClubMemberRole {
    /// Compares and determines which `ClubMemberRole` is higher in the hierarchy:
    /// `Member < Senior < VicePresident < President`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use brawl_api::ClubMemberRole;
    ///
    /// // vice-president has more power (is higher in the hierarchy) than a normal Member
    /// assert!(ClubMemberRole::Member < ClubMemberRole::VicePresident);
    /// assert!(ClubMemberRole::President > ClubMemberRole::VicePresident);
    /// assert!(ClubMemberRole::Senior > ClubMemberRole::Member);
    /// assert!(ClubMemberRole::Member >= ClubMemberRole::Member);
    /// ```
    fn cmp(&self, other: &ClubMemberRole) -> Ordering {
        (*self as u8).cmp(&(*other as u8))
    }
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
/// [`ClubMember`]: ../players/player/struct.Player.html#method.fetch_from
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
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

impl PartialOrd for ClubMember {
    /// Compares and determines which `ClubMember` has a higher role.
    ///
    /// # Examples
    ///
    /// (**NOTE:** Club members are not meant to be initialized, but rather obtained from
    /// a fetched [`Club`] instance. They are only instantiated here for this example.)
    ///
    /// ```rust
    /// use brawl_api::{ClubMember, ClubMemberRole};
    ///
    /// let member_1 = ClubMember { role: ClubMemberRole::Member, ..ClubMember::default() };
    /// let member_2 = ClubMember { role: ClubMemberRole::VicePresident, ..ClubMember::default() };
    ///
    /// assert!(member_1 < member_2)  // vice-president has more power than a normal Member
    /// ```
    ///
    /// [`Club`]: struct.Club.html
    fn partial_cmp(&self, other: &ClubMember) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ClubMember {
    /// Compares and determines which `ClubMember` has a higher role.
    ///
    /// # Examples
    ///
    /// (**NOTE:** Club members are not meant to be initialized, but rather obtained from
    /// a fetched [`Club`] instance. They are only instantiated here for this example.)
    ///
    /// ```rust
    /// use brawl_api::{ClubMember, ClubMemberRole};
    ///
    /// let member_1 = ClubMember { role: ClubMemberRole::Member, ..ClubMember::default() };
    /// let member_2 = ClubMember { role: ClubMemberRole::VicePresident, ..ClubMember::default() };
    ///
    /// assert!(member_1 < member_2)  // vice-president has more power than a normal Member
    /// ```
    ///
    /// [`Club`]: struct.Club.html
    fn cmp(&self, other: &ClubMember) -> Ordering {
        self.role.cmp(&other.role)
    }
}

impl Default for ClubMember {
    
    fn default() -> ClubMember {
        ClubMember {
            tag: String::from(""),
            name: String::from(""),
            trophies: 0,
            role: ClubMemberRole::Member,
            name_color: 0xff_ff_ff
        }
    }
}

// TODO: clubs/<tag>/members endpoint
