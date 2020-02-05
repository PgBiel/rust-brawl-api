# rust-brawl-api
A Rust implementation of the Brawl Stars API (https://developer.brawlstars.com/).

## Usage

1. Use `cargo` (add as dependency on your `Cargo.toml`).
2. Generate an API key in the Brawl Stars developer website.
3. Code!

For example:

```rust
use brawl_api::prelude::*;

fn main() -> Result<(), Box<dyn ::std::error::Error>> {
    let client = Client("MY AUTH TOKEN");
    let player = Player::fetch(&client, "#PLAYER_TAG_TO_FETCH")?;
    // now data for player with the given tag is available.
    // see all models in the documentation.
}
```

## License

Licensed under the MIT license (see the LICENSE file).

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you shall be licensed as MIT, without any additional terms or conditions.