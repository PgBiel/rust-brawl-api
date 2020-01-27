#![doc(hidden)]
/// Concats string(s) to the main API URI.
///
/// # Examples
///
/// ```rust
/// use brawl_api::b_api_concat;
///
/// assert_eq!(
///     b_api_concat!("players/"),
///     "https://api.brawlstars.com/v1/players/"
/// )
/// ```
#[macro_export]
macro_rules! b_api_concat {
    ($($s:expr),*) => {
        concat!("https://api.brawlstars.com/v1/", $($s,)*)
    }
}

/// Constructs any Map<Key, Value> type, based on an initializer expression.
///
/// # Examples
/// ```rust,ignore
/// let custom_map = map_build!{
///     MyMap::new();
///     "key" => "val",
///     "other_key" => value,
///     key => "val",
/// }
///
/// // Expands to
///
/// let custom_map = MyMap::new();
/// custom_map.insert("key", "val");
/// custom_map.insert("other_key", value);
/// custom_map.insert(key, "val");
/// ```
#[macro_export]
macro_rules! map_build {
    ($initializer:expr; $($key:expr => $val:expr),*) => {
        {
            let m = $initializer;
            $(
                m.insert($key, $val);
            )*
            m
        }
    }
}


