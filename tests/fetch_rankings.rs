//! Tests related to fetching the API `/rankings/` endpoint.
#[cfg(test)]
mod common;

#[cfg(test)]
mod tests {
    use brawl_api::prelude::*;
    use super::common;

    const TEST_RANK_REGION: &str = "global";
    const TEST_BRAWLER_ID: usize = Brawlers::Shelly as usize;

    /// Checks if player ranking fetching does not error.
    #[test]
    fn rankings_players_fetch() {
        let client = common::create_test_client();

        PlayerLeaderboard::fetch(&client, TEST_RANK_REGION, 5).unwrap();
    }

    /// Checks if club ranking fetching does not error.
    #[test]
    fn rankings_clubs_fetch() {
        let client = common::create_test_client();

        ClubLeaderboard::fetch(&client, TEST_RANK_REGION, 5).unwrap();
    }

    /// Checks if brawler ranking fetching does not error.
    #[test]
    fn rankings_brawlers_fetch() {
        let client = common::create_test_client();

        BrawlerLeaderboard::fetch(&client, TEST_RANK_REGION, TEST_BRAWLER_ID, 5)
            .unwrap();
    }
}