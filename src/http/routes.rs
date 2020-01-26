use crate::b_api_concat;


/// An enum representing the possible Brawl API routes.
#[non_exhaustive]
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
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

    /// Route for the `/clubs/:tag/members` endpoint.
    /// (`tag` must begin with a `#` (`%23`) for correct results.)
    ///
    /// This fetches a club's members.
    ClubMembers(String),

    /// Route for the `/rankings/:country_code/players?limit=x` endpoint.
    ///
    /// The limit can be up to 200. Specifying higher than that simply works the same way as
    /// specifying 200, thus returning up to 200 entries.
    PlayerRankings {
        country_code: String,
        limit: u8,
    },

    /// Route for the `/rankings/:country_code/clubs?limit=x` endpoint.
    ///
    /// The limit can be up to 200. Specifying higher than that simply works the same way as
    /// specifying 200, thus returning up to 200 entries.
    ClubRankings {
        country_code: String,
        limit: u8,
    },
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
                "{}{}/battlelog", b_api_concat!("/players/"), s
            ),

            Route::Club(ref s) => format!("{}{}", b_api_concat!("/clubs/"), s),

            Route::ClubMembers(ref s) => format!(
                "{}{}/members", b_api_concat!("/clubs/"), s
            ),

            Route::PlayerRankings {
                ref country_code,
                limit
            } => format!(
                "{}{}/players?limit={}", b_api_concat!("/rankings/"), country_code, limit
            ),

            Route::ClubRankings {
                ref country_code,
                limit
            } => format!(
                "{}{}/clubs?limit={}", b_api_concat!("/rankings/"), country_code, limit
            ),
        }
    }
}