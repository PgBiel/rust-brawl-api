//! Models for all `/players/:tag/...` Brawl Stars API endpoints.
//! Included by the feature `"players"`; removing that feature will disable the usage of this module.

pub mod player;
pub use player::*;

pub mod battlelog;
pub use battlelog::*;
