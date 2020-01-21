use crate::constants::API_URI;

/// Concats string(s) to the main API URI.
#[macro_export]
macro_rules! b_api_concat {
    ($($s:expr),*) => {
        concat!("https://api.brawlstars.com/v1/", $($s,)*)
    }
}
