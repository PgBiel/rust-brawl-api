use crate::constants::API_URI;

macro_rules! b_api_concat {
    ($($s:expr),*) => {
        concat!("https://api.brawlstars.com/v1/", $($s,)*)
    }
}
