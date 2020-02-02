//! Models for the `/clubs/:tag` Brawl Stars API endpoint.
//! Included by the feature `"clubs"`; removing that feature will disable the usage of this module.

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
use crate::serde::{
    serialize_smt_pointer, deserialize_number_from_string, deserialize_default_smt_pointer,
    oxffffff_default,
};
use crate::http::routes::Route;
use crate::util::{auto_hashtag, fetch_route};

use std::fmt::{Display, Formatter};
use crate::model::rankings::ClubRanking;
use std::cmp::Ordering;

pub use members::ClubMembers;

/// The type of club (whether it's open, invite-only, or closed).
#[non_exhaustive]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ClubType {
    Open,
    InviteOnly,
    Closed,
}

impl Default for ClubType {
    /// Defaults to [`ClubType::Open`] (new clubs start open).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use brawl_api::ClubType;
    ///
    /// assert_eq!(ClubType::default(), ClubType::Open);
    /// ```
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

    /// The members in this club, as a vector of [`ClubMember`] (note that the [`ClubMembers`]
    /// struct is simply a smart pointer for `Vec<ClubMember>`).
    ///
    /// [`ClubMember`]: struct.ClubMember.html
    /// [`ClubMembers`]: ./members/struct.ClubMembers.html
    #[serde(default)]
    #[serde(serialize_with="serialize_smt_pointer")]
    #[serde(deserialize_with="deserialize_default_smt_pointer")]
    pub members: ClubMembers, // Vec<ClubMember>,

    /// The type of club (see [`ClubType`] docs).
    ///
    /// [`ClubType`]: ./enum.ClubType.html
    #[serde(rename = "type")]
    #[serde(default)]
    pub club_type: ClubType
}

impl Default for Club {


    /// Returns an instance of `Club` with initial values.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use brawl_api::{Club, ClubType, ClubMembers};
    ///
    /// assert_eq!(
    ///     Club::default(),
    ///     Club {
    ///         tag: String::from(""),
    ///         name: String::from(""),
    ///         description: None,
    ///         trophies: 0,
    ///         required_trophies: 0,
    ///         members: ClubMembers::default(),
    ///         club_type: ClubType::Open,
    ///     }
    /// );
    /// ```
    fn default() -> Club {
        Club {
            tag: String::from(""),
            name: String::from(""),
            description: None,
            trophies: 0,
            required_trophies: 0,
            members: ClubMembers::default(),
            club_type: ClubType::Open,
        }
    }
}

impl GetFetchProp for Club {
    type Property = str;

    fn get_fetch_prop(&self) -> &str { &self.tag }

    fn get_route(tag: &str) -> Route { Route::Club(auto_hashtag(tag)) }
}

#[cfg_attr(feature = "async", async_trait)]
impl PropFetchable for Club {
    type Property = str;

    /// (Sync) Fetches a club from its tag.
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
    /// use brawl_api::{Client, Club, traits::*};
    ///
    /// # fn main() -> Result<(), Box<dyn ::std::error::Error>> {
    /// let my_client = Client::new("my auth token");
    /// let club = Club::fetch(&my_client, "#CLUBTAGHERE")?;
    /// // now the data for the given club is available for use.
    ///
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`Error::Request`]: error/enum.Error.html#variant.Request
    /// [`Error::Status`]: error/enum.Error.html#variant.Status
    /// [`Error::Ratelimited`]: error/enum.Error.html#variant.Ratelimited
    /// [`Error::Json`]: error/enum.Error.html#variant.Json
    fn fetch(client: &Client, tag: &str) -> Result<Club> {
        let route = Club::get_route(tag);
        let mut club = fetch_route::<Club>(client, &route)?;
        club.members.tag = club.tag.clone();
        Ok(club)
    }

    /// (Async) Fetches a club from its tag.
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
    /// use brawl_api::{Client, Club, traits::*};
    ///
    /// # async fn main() -> Result<(), Box<dyn ::std::error::Error>> {
    /// let my_client = Client::new("my auth token");
    /// let club = Club::a_fetch(&my_client, "#CLUBTAGHERE").await?;
    /// // now the data for the given club is available for use.
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
    async fn a_fetch(client: &Client, tag: &'async_trait str) -> Result<Club>
        where Self: 'async_trait,
              Self::Property: 'async_trait,
    {
        let route = Club::get_route(tag);
        let mut club = a_fetch_route::<Club>(client, &route).await?;
        club.members.tag = club.tag.clone();
        Ok(club)
    }
}

#[cfg_attr(feature = "async", async_trait)]
#[cfg(feature = "players")]
impl FetchFrom<PlayerClub> for Club {
    /// (Sync) Fetches a `Club` using data from a [`PlayerClub`] object.
    ///
    /// [`PlayerClub`]: ../../players/player/struct.PlayerClub.html
    fn fetch_from(client: &Client, p_club: &PlayerClub) -> Result<Club> {
        Club::fetch(client, &p_club.tag)
    }

    /// (Async) Fetches a `Club` using data from a [`PlayerClub`] object.
    ///
    /// [`PlayerClub`]: ../../players/player/struct.PlayerClub.html
    #[cfg(feature = "async")]
    async fn a_fetch_from(client: &Client, p_club: &PlayerClub) -> Result<Club> {
        Club::a_fetch(client, &p_club.tag).await
    }
}

#[cfg_attr(feature = "async", async_trait)]
#[cfg(feature = "rankings")]
impl FetchFrom<ClubRanking> for Club {

    /// (Sync) Fetches a `Club` using data from a [`ClubRanking`] object.
    ///
    /// [`ClubRanking`]: ../../rankings/clubs/struct.ClubRanking.html
    fn fetch_from(client: &Client, c_ranking: &ClubRanking) -> Result<Club> {
        Club::fetch(client, &c_ranking.tag)
    }

    /// (Async) Fetches a `Club` using data from a [`ClubRanking`] object.
    ///
    /// [`ClubRanking`]: ../../rankings/clubs/struct.ClubRanking.html
    #[cfg(feature = "async")]
    async fn a_fetch_from(client: &Client, c_ranking: &ClubRanking) -> Result<Club> {
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
    /// Writes this `ClubMemberRole` variant's name.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use brawl_api::ClubMemberRole;
    ///
    /// assert_eq!(
    ///     format!("{}", ClubMemberRole::Senior),
    ///     String::from("Senior")
    /// );
    /// ```
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
    /// Defaults to [`ClubMemberRole::Member`] - that is the initial role that any club member
    /// adquires after joining (it may be promoted later, though).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use brawl_api::ClubMemberRole;
    ///
    /// assert_eq!(ClubMemberRole::default(), ClubMemberRole::Member);
    /// ```
    ///
    /// [`ClubMemberRole::Member`]: ./enum.ClubMemberRole.html#variant.Member
    fn default() -> ClubMemberRole { ClubMemberRole::Member }
}

/// A struct representing a Brawl Stars club's member, with its club-relevant data
/// (most importantly, its role). Use [`Player::fetch_from`] to fetch the full player data.
///
/// [`Player::fetch_from`]: ../players/player/struct.Player.html#method.fetch_from
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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
    #[serde(default = "oxffffff_default")]
    #[serde(deserialize_with = "deserialize_number_from_string")]  // parse num
    pub name_color: u64
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

    /// Returns an instance of `ClubMember` with initial values.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use brawl_api::model::{ClubMember, ClubMemberRole};
    ///
    /// assert_eq!(
    ///     ClubMember::default(),
    ///     ClubMember {
    ///         tag: String::from(""),
    ///         name: String::from(""),
    ///         trophies: 0,
    ///         role: ClubMemberRole::default(),
    ///         name_color: 0xff_ff_ff
    ///     }
    /// );
    /// ```
    fn default() -> ClubMember {
        ClubMember {
            tag: String::from(""),
            name: String::from(""),
            trophies: 0,
            role: ClubMemberRole::default(),
            name_color: 0xff_ff_ff
        }
    }
}

/// Contains the model for the `/clubs/:tag/members` endpoint, which simply retrieves a club's
/// members without needing to get the rest of the data.
pub mod members {
    use super::*;
    use std::ops::{Deref, DerefMut};

    /// Represents a list of Club members, without relating to a previous [`Club`] object.
    /// This is only used if one does not want to fetch full club data, but only its members.
    ///
    /// Use [`ClubMembers::fetch`] to fetch the members from a specific club tag.
    /// 
    /// [`Club`]: ../struct.Club.html
    /// [`ClubMembers::fetch`]: #method.fetch
    #[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
    pub struct ClubMembers {
        /// The tag of the club whose members were fetched.
        #[serde(skip)]  // artificial
        pub tag: String,

        /// The fetched members of the specified club.
        #[serde(default)]
        pub items: Vec<ClubMember>
    }

    impl Deref for ClubMembers {
        type Target = Vec<ClubMember>;

        /// Obtain the club's members - dereferencing returns the [`items`] field.
        ///
        /// # Examples
        ///
        /// ```rust,ignore
        /// use brawl_api::{Client, ClubMembers, traits::*};
        ///
        /// # fn main() -> Result<(), Box<dyn ::std::error::Error>> {
        /// let client = Client::new("my auth token");
        /// let members = ClubMembers::fetch(
        ///     &client,            // <- the client containing the auth key
        ///     "#CLUB_TAG_HERE"  // <- the club whose members should be fetched
        /// )?;
        ///
        /// assert_eq!(members.items, *members);
        ///
        /// #     Ok(())
        /// # }
        ///
        /// ```
        ///
        /// [`items`]: #structfield.items
        fn deref(&self) -> &Vec<ClubMember> {
            &self.items
        }
    }

    impl DerefMut for ClubMembers {
        /// Obtain the club's members - dereferencing returns the [`items`] field.
        ///
        /// # Examples
        ///
        /// ```rust,ignore
        /// use brawl_api::{Client, ClubMembers, traits::*};
        ///
        /// # fn main() -> Result<(), Box<dyn ::std::error::Error>> {
        /// let client = Client::new("my auth token");
        /// let members = ClubMembers::fetch(
        ///     &client,            // <- the client containing the auth key
        ///     "#CLUB_TAG_HERE"  // <- the club whose members should be fetched
        /// )?;
        ///
        /// assert_eq!(members.items, *members);
        ///
        /// #     Ok(())
        /// # }
        ///
        /// ```
        ///
        /// [`items`]: #structfield.items
        fn deref_mut(&mut self) -> &mut Vec<ClubMember> {
            &mut self.items
        }
    }

    impl GetFetchProp for ClubMembers {
        type Property = str;

        fn get_fetch_prop(&self) -> &str {
            &*self.tag
        }

        fn get_route(tag: &str) -> Route {
            Route::ClubMembers(auto_hashtag(tag))
        }
    }

    impl From<Club> for ClubMembers {
        /// Simply returns a given [`Club`]'s [`members`][Club.members] field.
        ///
        /// [`Club`]: ../struct.Club.html
        /// [Club.members]: ../struct.Club.html#structfield.members
        fn from(club: Club) -> ClubMembers {
            club.members
        }
    }

    impl From<&Club> for ClubMembers {
        /// Simply returns a given [`Club`]'s [`members`][Club.members] field,
        /// **while cloning**.
        ///
        /// [`Club`]: ../struct.Club.html
        /// [Club.members]: ../struct.Club.html#structfield.members
        fn from(club: &Club) -> ClubMembers {
            club.members.to_owned()
        }
    }

    impl<'a> From<&'a Club> for &'a ClubMembers {
        /// Simply returns a given [`Club`]'s [`members`][Club.members] field.
        ///
        /// [`Club`]: ../struct.Club.html
        /// [Club.members]: ../struct.Club.html#structfield.members
        fn from(club: &'a Club) -> Self {
            (&club.members) as &'a ClubMembers
        }
    }

    #[cfg_attr(feature = "async", async_trait)]
    impl PropFetchable for ClubMembers {
        type Property = str;

        /// (Sync) Fetches a club's members, given its tag, without fetching the rest of the data.
        /// (If it is desired to fetch the rest of the data as well, simply fetching a [`Club`] is
        /// enough, since that also fetches all of the members.)
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
        /// use brawl_api::{Client, ClubMembers, traits::*};
        ///
        /// # fn main() -> Result<(), Box<dyn ::std::error::Error>> {
        /// let my_client = Client::new("my auth token");
        /// let club_members = ClubMembers::fetch(&my_client, "#CLUBTAGHERE")?;
        /// // now the members of the club with the given tag are available in the code
        ///
        /// # Ok(())
        /// # }
        /// ```
        ///
        /// [`Club`]: ../struct.Club.html
        /// [`Error::Request`]: error/enum.Error.html#variant.Request
        /// [`Error::Status`]: error/enum.Error.html#variant.Status
        /// [`Error::Ratelimited`]: error/enum.Error.html#variant.Ratelimited
        /// [`Error::Json`]: error/enum.Error.html#variant.Json
        fn fetch(client: &Client, tag: &str) -> Result<ClubMembers> {
            let route = Self::get_route(tag);
            let mut members = fetch_route::<ClubMembers>(client, &route)?;
            members.tag = tag.to_owned();
            Ok(members)
        }

        /// (Async) Fetches a club's members, given its tag, without fetching the rest of the data.
        /// (If it is desired to fetch the rest of the data as well, simply fetching a [`Club`] is
        /// enough, since that also fetches all of the members.)
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
        /// use brawl_api::{Client, ClubMembers, traits::*};
        ///
        /// # async fn main() -> Result<(), Box<dyn ::std::error::Error>> {
        /// let my_client = Client::new("my auth token");
        /// let club_members = ClubMembers::a_fetch(&my_client, "#CLUBTAGHERE").await?;
        /// // now the members of the club with the given tag are available in the code
        ///
        /// # Ok(())
        /// # }
        /// ```
        ///
        /// [`Club`]: ../struct.Club.html
        /// [`Error::Request`]: error/enum.Error.html#variant.Request
        /// [`Error::Status`]: error/enum.Error.html#variant.Status
        /// [`Error::Ratelimited`]: error/enum.Error.html#variant.Ratelimited
        /// [`Error::Json`]: error/enum.Error.html#variant.Json
        #[cfg(feature="async")]
        async fn a_fetch(client: &Client, tag: &'async_trait str) -> Result<ClubMembers>
            where Self: 'async_trait,
                  Self::Property: 'async_trait,
        {
            let route = ClubMembers::get_route(tag);
            let mut members = a_fetch_route::<ClubMembers>(client, &route).await?;
            members.tag = tag.to_owned();
            Ok(members)
        }
    }

    impl Default for ClubMembers {
        /// Returns an instance of `ClubMembers` with initial values.
        ///
        /// # Examples
        ///
        /// ```rust
        /// use brawl_api::model::ClubMembers;
        ///
        /// assert_eq!(
        ///     ClubMembers::default(),
        ///     ClubMembers {
        ///         tag: String::from(""),
        ///         items: vec![],
        ///     }
        /// );
        /// ```
        fn default() -> ClubMembers {
            ClubMembers { tag: String::from(""), items: vec![] }
        }
    }
}

///////////////////////////////////   tests   ///////////////////////////////////

#[cfg(test)]
mod tests {
    use std::result::Result as StdResult;
    use super::*;
    use crate::error::Error as BrawlError;
    use serde_json;

    /// Tests for club deserialization from API-provided JSON.
    #[test]
    fn club_deser() -> StdResult<(), Box<dyn ::std::error::Error>> {
        let club_json_s = r##"{
  "tag": "#GGGGGGG",
  "name": "Club",
  "description": "Brawl Stars club",
  "type": "open",
  "requiredTrophies": 1000,
  "trophies": 60000,
  "members": [
    {
      "tag": "#PPP200JJJ",
      "name": "Member #1",
      "nameColor": "0xffff8afb",
      "role": "vicePresident",
      "trophies": 500
    },
    {
      "tag": "#CCCCCCCCCC",
      "name": "Member #2",
      "nameColor": "0xff1ba5f5",
      "role": "president",
      "trophies": 200
    },
    {
      "tag": "#VVVVVVVVV",
      "name": "Member #3",
      "nameColor": "0xffffff",
      "role": "member",
      "trophies": 8500
    },
    {
      "tag": "#9999999999",
      "name": "Member #4",
      "nameColor": "0xff4ddba2",
      "role": "member",
      "trophies": 20000
    },
    {
      "tag": "#UUUUUU888",
      "name": "Member #5",
      "nameColor": "0xff1ba5f5",
      "role": "senior",
      "trophies": 4500
    },
    {
      "tag": "#JJJJJJJJJ",
      "name": "Member ██▬█",
      "nameColor": "0xff1ba5f5",
      "role": "member",
      "trophies": 26300
    }
  ]
}"##;

        let club: Club = serde_json::from_str::<Club>(club_json_s)
            .map_err(BrawlError::Json)?;

        assert_eq!(
            club,
            Club {
                tag: String::from("#GGGGGGG"),
                name: String::from("Club"),
                description: Some(String::from("Brawl Stars club")),
                club_type: ClubType::Open,
                required_trophies: 1000,
                trophies: 60000,
                members: ClubMembers {
                    items: vec![
                        ClubMember {
                            tag: String::from("#PPP200JJJ"),
                            name: String::from("Member #1"),
                            name_color: 0xffff8afb,
                            role: ClubMemberRole::VicePresident,
                            trophies: 500
                        },
                        ClubMember {
                            tag: String::from("#CCCCCCCCCC"),
                            name: String::from("Member #2"),
                            name_color: 0xff1ba5f5,
                            role: ClubMemberRole::President,
                            trophies: 200
                        },
                        ClubMember {
                            tag: String::from("#VVVVVVVVV"),
                            name: String::from("Member #3"),
                            name_color: 0xffffff,
                            role: ClubMemberRole::Member,
                            trophies: 8500
                        },
                        ClubMember {
                            tag: String::from("#9999999999"),
                            name: String::from("Member #4"),
                            name_color: 0xff4ddba2,
                            role: ClubMemberRole::Member,
                            trophies: 20000
                        },
                        ClubMember {
                            tag: String::from("#UUUUUU888"),
                            name: String::from("Member #5"),
                            name_color: 0xff1ba5f5,
                            role: ClubMemberRole::Senior,
                            trophies: 4500
                        },
                        ClubMember {
                            tag: String::from("#JJJJJJJJJ"),
                            name: String::from("Member ██▬█"),
                            name_color: 0xff1ba5f5,
                            role: ClubMemberRole::Member,
                            trophies: 26300
                        }
                    ],
                    ..ClubMembers::default()
                }
            }
        );

        Ok(())
    }

    /// Tests for ClubMembers deserialization from API-provided JSON.
    #[test]
    fn club_members_deser() -> StdResult<(), Box<dyn ::std::error::Error>> {
        let cm_json_s = r##"{
  "items": [
    {
      "tag": "#PPP200JJJ",
      "name": "Member #1",
      "nameColor": "0xffff8afb",
      "role": "vicePresident",
      "trophies": 500
    },
    {
      "tag": "#CCCCCCCCCC",
      "name": "Member #2",
      "nameColor": "0xff1ba5f5",
      "role": "president",
      "trophies": 200
    },
    {
      "tag": "#VVVVVVVVV",
      "name": "Member #3",
      "nameColor": "0xffffff",
      "role": "member",
      "trophies": 8500
    },
    {
      "tag": "#9999999999",
      "name": "Member #4",
      "nameColor": "0xff4ddba2",
      "role": "member",
      "trophies": 20000
    },
    {
      "tag": "#UUUUUU888",
      "name": "Member #5",
      "nameColor": "0xff1ba5f5",
      "role": "senior",
      "trophies": 4500
    },
    {
      "tag": "#JJJJJJJJJ",
      "name": "Member ██▬█",
      "nameColor": "0xff1ba5f5",
      "role": "member",
      "trophies": 26300
    }
  ],
  "paging": {
    "cursors": {}
  }
}"##;
        let club_members: ClubMembers = serde_json::from_str::<ClubMembers>(cm_json_s)
            .map_err(BrawlError::Json)?;

        assert_eq!(
            club_members,
            ClubMembers {
                items: vec![
                    ClubMember {
                        tag: String::from("#PPP200JJJ"),
                        name: String::from("Member #1"),
                        name_color: 0xffff8afb,
                        role: ClubMemberRole::VicePresident,
                        trophies: 500
                    },
                    ClubMember {
                        tag: String::from("#CCCCCCCCCC"),
                        name: String::from("Member #2"),
                        name_color: 0xff1ba5f5,
                        role: ClubMemberRole::President,
                        trophies: 200
                    },
                    ClubMember {
                        tag: String::from("#VVVVVVVVV"),
                        name: String::from("Member #3"),
                        name_color: 0xffffff,
                        role: ClubMemberRole::Member,
                        trophies: 8500
                    },
                    ClubMember {
                        tag: String::from("#9999999999"),
                        name: String::from("Member #4"),
                        name_color: 0xff4ddba2,
                        role: ClubMemberRole::Member,
                        trophies: 20000
                    },
                    ClubMember {
                        tag: String::from("#UUUUUU888"),
                        name: String::from("Member #5"),
                        name_color: 0xff1ba5f5,
                        role: ClubMemberRole::Senior,
                        trophies: 4500
                    },
                    ClubMember {
                        tag: String::from("#JJJJJJJJJ"),
                        name: String::from("Member ██▬█"),
                        name_color: 0xff1ba5f5,
                        role: ClubMemberRole::Member,
                        trophies: 26300
                    }
                ],
                ..ClubMembers::default()
            }
        );

        Ok(())
    }
}

