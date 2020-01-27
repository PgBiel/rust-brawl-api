//! Models for all `/rankings/:country_code/...` Brawl Stars API endpoints.
//! Included by the feature `"rankings"`; removing that feature will disable the usage of this module.

pub mod players;
pub use players::*;

pub mod clubs;
pub use clubs::*;

pub mod brawlers;
pub use brawlers::*;
