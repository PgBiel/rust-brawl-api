//! Tests related to fetching the API `/brawlers/` endpoint.
#[cfg(test)]
mod common;

#[cfg(test)]
mod tests {
    use brawl_api::prelude::*;
    use super::common;

    const TEST_BRAWLER_ID: usize = Brawlers::Shelly as usize;

    /// Checks if brawler list etching does not error.
    #[test]
    fn brawlers_fetch() {
        let client = common::create_test_client();

        BrawlerList::fetch(&client).unwrap();
    }

    /// Checks if brawler data fetching does not error.
    #[test]
    fn brawler_fetch() {
        let client = common::create_test_client();

        Brawler::fetch(&client, TEST_BRAWLER_ID).unwrap();
    }
}