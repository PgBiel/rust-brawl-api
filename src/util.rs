use std::result::Result as StdResult;
use serde::{Deserialize, de::DeserializeOwned};
use serde_json::{Map as SerdeJsonMap, Value};
use crate::error::{Result, Error};
use crate::http::Client;
use crate::http::routes::Route;
use reqwest::{Error as ReqwestError, StatusCode};
use reqwest::blocking::{
    Response,
};

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

/// Fetches a deserializable struct/enum/... from some route.
pub(crate) fn fetch_route<T>(client: &Client, route: &Route) -> Result<T>
    where T: DeserializeOwned {
    let mut request_b = client.build_endpoint_get(&*route.to_url_str())?;
    let response: StdResult<Response, ReqwestError> = request_b.send();
    let response = response.map_err(Error::Request)?;

    let status: StatusCode = response.status();
    if status.is_success() {
        return response.json::<T>().map_err(Error::Json);
    } else {
        return Err(Error::from_response(response, None));
    }
}
