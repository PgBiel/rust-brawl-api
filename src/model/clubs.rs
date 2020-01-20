use crate::traits::{Fetchable, FetchFrom};
use crate::model::player::PlayerClub;
use crate::error::Error;

/// A struct representing a Brawl Stars club, with all of its data.
/// Use [Club::fetch] to fetch one based on tag.
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

    /// The members in this club, as a vector of [ClubMember].
    pub members: Vec<ClubMember>,

    /// The type of club.
    pub club_type: String
}

impl Fetchable for Club {
    type Property = &'static str;

    fn fetch(tag: &str) -> Result<Club, Error> {
        // TODO: Implement TagFetchable for Club (be able to fetch a club)
    }
}

impl FetchFrom<PlayerClub> for Club {
    fn fetch_from(p_club: PlayerClub) -> Result<Club, Error> {
        Club::fetch(&p_club.tag)
    }
}

/// An enum representing a member's possible roles. See [ClubMember].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClubMemberRole {
    Member,
    Senior,
    VicePresident,
    President,
}

/// A struct representing a Brawl Stars club's member, with its club-relevant data
/// (most importantly, its role). Use [Player::try_from] to fetch the full player data.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClubMember {

    /// The member's tag.
    pub tag: String,

    /// The member's name.
    pub name: String,

    /// The member's trophies.
    pub trophies: usize,

    /// The member's role in the guild. (Default is [ClubMemberRole::Member])
    pub role: ClubMemberRole,

    /// The member's name color, as an integer (Default is 0xffffff = 16777215 - this is used
    /// when the data is not available).
    pub name_color: String
}

// TODO: clubs/<tag>/members endpoint
