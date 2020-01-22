use serde::Deserializer;
use serde_json::{Map as SerdeJsonMap, Value};

pub(crate) fn auto_hashtag(tag: &str) -> String {
    let mut new_tag = String::from(tag.clone());
    if tag.starts_with("#") {
        new_tag = new_tag.replacen("#", "%23", 1);
    } else if cfg!(feature = "auto-hashtag") {  // automtically add the hashtag
        new_tag = format!("%23{}", new_tag);
    }
    new_tag
}

pub(crate) type JsonMap = SerdeJsonMap<String, Value>;
