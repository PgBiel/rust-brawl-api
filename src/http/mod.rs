//! Module related to the internals of fetching and requesting to the Brawl Stars API.

pub mod request;

pub mod client;
pub use client::Client;

pub mod routes;
pub use routes::Route;
