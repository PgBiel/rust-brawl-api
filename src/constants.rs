pub const API_URI: &str = "https://api.brawlstars.com/v1/";

pub const USER_AGENT: &str = concat!(
    "Rust (brawl-api crate, ", env!("CARGO_PKG_VERSION"),
    " - https://github.com/PgBiel/rust-brawl-api)"
);
