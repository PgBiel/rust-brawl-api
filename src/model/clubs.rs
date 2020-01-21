#[cfg(feature = "async")]
use async_trait::async_trait;

use crate::traits::{PropFetchable, FetchFrom};
use crate::error::Result;

#[cfg(feature = "players")]
use crate::model::players::PlayerClub;

/// A struct representing a Brawl Stars club, with all of its data.
/// Use [`Club::fetch`] to fetch one based on tag.
///
/// [`Club::fetch`]: ./struct.Club.html#method.fetch
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Club {

    /// The club's tag.
    pub tag: String,

    /// The club's name.
    pub name: String,

    /// The club's description.
    pub description: String,

    /// The club's trophies.
    pub trophies: usize,

    /// The amount of trophies required to enter on this club, or 0 if it allows any amount.
    pub required_trophies: usize,

    /// The members in this club, as a vector of [`ClubMember`].
    ///
    /// [`ClubMember`]: ./struct.ClubMember.html
    pub members: Vec<ClubMember>,

    /// The type of club.
    pub club_type: String
}

#[cfg_attr(feature = "async", async_trait)]
impl<'a> PropFetchable for Club {
    type Property = &'a str;

    /// (Sync) Fetch a Club from its tag.
    fn fetch(tag: &'a str) -> Result<Club> {
        // TODO: Implement TagFetchable for Club (be able to fetch a club)
    }

    #[cfg(feature = "async")]
    async fn a_fetch(tag: &'a str) -> Result<Club> {

    }

    fn get_fetch_prop(&self) -> &'a str {
        &*self.tag
    }
}

#[cfg_attr(feature = "async", async_trait)]
#[cfg(feature = "players")]
impl FetchFrom<PlayerClub> for Club {
    fn fetch_from(p_club: PlayerClub) -> Result<Club> {
        Club::fetch(&p_club.tag)
    }

    #[cfg(feature = "async")]
    async fn a_fetch_from(p_club: PlayerClub) -> Result<Club> {
        Club::a_fetch(&p_club.tag).await
    }
}

/// An enum representing a member's possible roles (See [`ClubMember`]).
///
/// [`ClubMember`]: ./struct.ClubMember.html
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClubMemberRole {
    Member,
    Senior,
    VicePresident,
    President,
}

/// A struct representing a Brawl Stars club's member, with its club-relevant data
/// (most importantly, its role). Use [`Player::fetch_from`] to fetch the full player data.
///
/// [`ClubMember`]: ../players/struct.Player.html#method.fetch_from
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClubMember {

    /// The member's tag.
    pub tag: String,

    /// The member's name.
    pub name: String,

    /// The member's trophies.
    pub trophies: usize,

    /// The member's role in the guild. (Default is [`ClubMemberRole::Member`])
    ///
    /// [`ClubMemberRole::Member`]: ./enum.ClubMemberRole.html#variant.Member
    pub role: ClubMemberRole,

    /// The member's name color, as an integer (Default is 0xffffff = 16777215 - this is used
    /// when the data is not available).
    pub name_color: String
}

// TODO: clubs/<tag>/members endpoint
