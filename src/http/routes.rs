use crate::b_api_concat;
use url::Url;

/// An enum representing the possible Brawl API routes.
#[derive(Debug, Clone)]
pub enum Route {
    /// Route for the `/players/:tag` endpoint. (`tag` must begin with a `#` (`%23`) for correct
    /// results.)
    ///
    /// This fetches a player's data.
    Player(String),

    /// Route for the `/players/:tag/battlelog` endpoint. (`tag` must begin with a `#` (`%23`) for
    /// correct results.)
    ///
    /// This fetches the player's recently-played battles.
    PlayerBattlelogs(String),

    /// Route for the `/clubs/:tag` endpoint. (`tag` must begin with a `#` (`%23`) for correct
    /// results.)
    ///
    /// This fetches a club's data.
    Club(String),

    ///  Route for the `/clubs/:tag/members` endpoint.
    /// (`tag` must begin with a `#` (`%23`) for correct results.)
    ///
    /// This fetches a club's members.
    ClubMembers(String),

    #[doc(hidden)]
    _AntiExhaustive,
}

impl Route {

    /// Evaluates the `Route` instance into a full URL path string.
    ///
    /// # Examples
    /// ```rs
    /// use brawl_api::Route;
    /// assert_eq!(Route::Player("tag"), "https://api.brawlstars.com/v1/players/tag")
    /// assert_eq!(
    ///     Route::PlayerBattlelogs("tag"), "https://api.brawlstars.com/v1/players/tag/battlelogs"
    /// )
    /// assert_eq!(Route::Club("tag"), "https://api.brawlstars.com/v1/clubs/tag")
    /// assert_eq!(Route::ClubMembers("tag"), "https://api.brawlstars.com/v1/clubs/tag/members")
    /// ```
    pub fn to_url_str(&self) -> String {
        match self {
            Route::Player(ref s) => format!("{}{}", b_api_concat!("/players/"), s),

            Route::PlayerBattlelogs(ref s) => format!(
                "{}{}{}", b_api_concat!("/players/"), s, "/battlelog"
            ),

            Route::Club(ref s) => format!("{}{}", b_api_concat!("/clubs/"), s),

            Route::ClubMembers(ref s) => format!(
                "{}{}{}", b_api_concat!("/clubs/"), s, "/members"
            ),

            _AntiExhaustive => unreachable!("May not use the '_AntiExhaustive' variant."),
        }
    }
}