use crate::constants::API_URI;

macro_rules! b_api_concat {
    ($($s:expr),*) => {
        concat!(brawl_api::constants::API_URI, $($s,)*)
    }
}
