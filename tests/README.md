# tests directory

This directory contains integration tests for the `brawl_api` crate. Inside this directory, there must be a `test_config.json` file with the API auth key (following the format below) in order to test fetching.

## Test Config File Format 

```json
{
  "key": "api auth key here",
  "tags": {
    "player": "#PLAYER_TAG_TO_USE_WHEN_TESTING_PLAYER_FETCHING",
    "club": "#CLUB_TAG_TO_USE_WHEN_TESTING_CLUB_FETCHING"
  }
}
```

