//! Contains the `Route` enum, responsible for listing the available API endpoints and parsing
//! the given values into a valid URL.

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

    /// Route for the `/rankings/:country_code/players?limit=x` endpoint (shows the top `x` players
    /// with most trophies in said country code).
    ///
    /// The limit can be up to 200. Specifying higher than that simply works the same way as
    /// specifying 200, thus returning up to 200 entries.
    PlayerRankings {
        /// The two-letter country code whose leaderboard should be fetched (e.g. BR for Brazil,
        /// ZW for Zimbabwe...), or `"global"` for the global leaderboard.
        country_code: String,

        /// The limit of rankings to get (i.e., to get the top `limit` players, sorted by trophies).
        limit: u8,
    },

    /// Route for the `/rankings/:country_code/clubs?limit=x` endpoint.
    ///
    /// The limit can be up to 200. Specifying higher than that simply works the same way as
    /// specifying 200, thus returning up to 200 entries.
    ClubRankings {
        /// The two-letter country code whose leaderboard should be fetched (e.g. BR for Brazil,
        /// ZW for Zimbabwe...), or `"global"` for the global leaderboard.
        country_code: String,

        /// The limit of rankings to get (i.e., to get the top `limit` clubs, sorted by trophies).
        limit: u8,
    },

    /// Route for the `/rankings/:country_code/brawlers/:brawler_id?limit=x` endpoint.
    ///
    /// The limit can be up to 200. Specifying higher than that simply works the same way as
    /// specifying 200, thus returning up to 200 entries.
    BrawlerRankings {
        /// The two-letter country code whose leaderboard should be fetched (e.g. BR for Brazil,
        /// ZW for Zimbabwe...), or `"global"` for the global leaderboard.
        country_code: String,

        /// The ID of the brawler whose rankings should be fetched. To obtain this,
        /// use the `/brawlers/` endpoint.
        brawler_id: usize,

        /// The limit of rankings to get (i.e., to get the top `limit` players, sorted by trophies
        /// on this specific brawler).
        limit: u8,
    },

    /// Route for the `/brawlers/` endpoint, which returns data for all brawlers in the game.
    Brawlers,

    /// Route for the `/brawlers/:id` endpoint, which returns data for a specific brawler, given
    /// that brawler's ID.
    Brawler(usize),
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
            Route::Player(ref s) => format!("{}{}", b_api_concat!("players/"), s),

            Route::PlayerBattlelogs(ref s) => format!(
                "{}{}/battlelog", b_api_concat!("players/"), s
            ),

            Route::Club(ref s) => format!("{}{}", b_api_concat!("clubs/"), s),

            Route::ClubMembers(ref s) => format!(
                "{}{}/members", b_api_concat!("clubs/"), s
            ),

            Route::PlayerRankings {
                ref country_code,
                limit
            } => format!(
                "{}{}/players?limit={}", b_api_concat!("rankings/"), country_code, limit
            ),

            Route::ClubRankings {
                ref country_code,
                limit
            } => format!(
                "{}{}/clubs?limit={}", b_api_concat!("rankings/"), country_code, limit
            ),

            Route::BrawlerRankings {
                ref country_code,
                brawler_id,
                limit
            } => format!(
                "{}{}/brawlers/{}?limit={}",
                b_api_concat!("rankings/"), country_code, brawler_id, limit
            ),

            Route::Brawlers => String::from(b_api_concat!("brawlers/")),

            Route::Brawler(id) => format!(
                "{}/{}",
                b_api_concat!("brawlers"),
                id,
            )
        }
    }
}