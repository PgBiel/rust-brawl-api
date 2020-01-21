pub const API_URI: &'static str = "https://api.brawlstars.com/v1/";

pub const USER_AGENT: &'static str = concat!(
    "Rust (brawl-api crate, ", env!("CARGO_PKG_VERSION"),
    " - https://github.com/PgBiel/rust-brawl-api)"
);
