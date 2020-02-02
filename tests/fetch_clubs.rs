//! Tests related to fetching the API `/clubs/` endpoint.
#[cfg(test)]
mod common;

#[cfg(test)]
mod tests {
    use brawl_api::prelude::*;
    use super::common;

    fn get_tag_client() -> (String, Client) {
        let config = common::open_test_config_panic();
        (config.tags.club, Client::new(&config.key))
    }

    /// Checks if club fetching does not error and returns the correct club.
    #[test]
    fn club_fetch() {
        let (tag, client) = get_tag_client();

        let club = Club::fetch(&client, &*tag).unwrap();

        assert_eq!(club.tag, tag);

        assert_eq!(club.members.tag, club.tag);
    }

    /// Checks if club member fetching does not error.
    #[test]
    fn club_members_fetch() {
        let (tag, client) = get_tag_client();

        let members = ClubMembers::fetch(&client, &*tag).unwrap();

        assert_eq!(members.tag, tag);
    }
}