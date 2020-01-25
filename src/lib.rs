pub(crate) mod util;

pub mod serde;

pub mod traits;

pub mod constants;

pub mod http;

mod macros;

pub mod model;

#[cfg(feature = "players")]
pub use model::players::{Player, PlayerClub, PlayerBrawlerStat, StarPower};

#[cfg(feature = "clubs")]
pub use model::clubs::{Club, ClubMember, ClubMemberRole};

pub mod error;

//#[cfg(test)]
//mod tests {
//    #[test]
//    fn it_works() {
//        assert_eq!(2 + 2, 4);
//    }
//}
