//! Tests related to fetching the API `/players/` endpoint.
#[cfg(test)]
mod common;

#[cfg(test)]
mod tests {
    use brawl_api::prelude::*;
    use super::common;

    fn get_tag_client() -> (String, Client) {
        let config = common::open_test_config_panic();
        (config.tags.player, Client::new(&config.key))
    }

    /// Checks if player fetching does not error and returns the correct player.
    #[test]
    fn player_fetch() {
        let (tag, client) = get_tag_client();

        let player = Player::fetch(&client, &*tag).unwrap();

        assert_eq!(player.tag, tag);

    }

    /// Checks if a battlelog fetching does not error.
    #[test]
    fn battlelog_fetch() {
        let (tag, client) = get_tag_client();

        BattleLog::fetch(&client, &*tag).unwrap();
    }
}