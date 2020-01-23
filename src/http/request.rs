use reqwest::blocking::{
    RequestBuilder
};

#[cfg(feature = "async")]
use reqwest::RequestBuilder as ARequestBuilder;

use reqwest::{
    header::{
        HeaderMap,
        USER_AGENT, AUTHORIZATION, CONTENT_TYPE, CONTENT_LENGTH,
        HeaderValue,
    },
    Url,
    Method,
};
use crate::error::{Result, Error};
use crate::traits::Initializable;
use crate::http::Client;
use crate::constants::USER_AGENT as B_API_USER_AGENT;
use crate::map_build;

/// A struct representing a request to some endpoint.
#[derive(Debug, Clone, PartialEq)]
pub struct Request<'a> {

    /// The body of the request. (Note that this is rarely, if ever, used in this lib.)
    pub body: Option<&'a [u8]>,

    /// The headers of the request.
    pub headers: Option<HeaderMap>,

    /// The endpoint (e.g. /players/%23sometag).
    pub endpoint: String,

    /// The method (GET/POST/...). Defaults to GET
    pub method: Method,
}

impl<'a> Initializable for Request<'a> {
    fn new() -> Request<'a> {
        Request {
            body: None,
            headers: None,
            endpoint: String::from(""),
            method: Method::GET,
        }
    }
}

#[doc(hidden)]
enum _BuildRes {
    SyncBuilder(RequestBuilder),

    #[cfg(feature = "async")]
    AsyncBuilder(ARequestBuilder),

    _AntiExhaustive,
}

use _BuildRes::SyncBuilder;

#[cfg(feature = "async")]
use _BuildRes::AsyncBuilder;

// (Credits to Serenity lib for the useful HTTP bases)
impl<'a> Request<'a> {
    fn _generic_build(&'a self, client: &Client, is_sync: bool) -> Result<_BuildRes> {
        let Request {
            body,
            headers: ref r_headers,
            endpoint: ref r_endpoint,
            method: ref method,
        } = *self;

        let mut builder: _BuildRes = if is_sync {
            SyncBuilder(
                client.inner.request(
                    method,
                    Url::parse(r_endpoint).map_err(Error::Url)?,
                )
            )
        } else if cfg!(feature = "async") {
            #[cfg(feature = "async")]
            AsyncBuilder(
                client.a_inner.request(
                    method,
                    Url::parse(r_endpoint).map_err(Error::Url)?,
                )
            )
        } else {
            panic!("Async feature not enabled; can't build request.");
        };

        if let Some(ref bytes) = body {  // body was provided
            let b_vec = Vec::from(*bytes);
            builder = match builder {
                SyncBuilder(ref inner) => SyncBuilder(inner.body(b_vec)),

                #[cfg(feature = "async")]
                AsyncBuilder(ref a_inner) => AsyncBuilder(a_inner.body(b_vec)),

                _ => unreachable!(),
            }
        }

        let key = &client.auth_key;

        let key = if key.starts_with("Bearer ") {
            key
        } else {
            format!("Bearer {}", key)
        };

        let mut headers = HeaderMap::with_capacity(3);
        headers.insert(USER_AGENT, HeaderValue::from_static(&B_API_USER_AGENT));
        headers.insert(AUTHORIZATION,
                       HeaderValue::from_str(&key).map_err(Error::Authorization)?);
        headers.insert(CONTENT_TYPE, HeaderValue::from_static(&"application/json"));
        headers.insert(CONTENT_LENGTH, HeaderValue::from_static(&"0"));

        if let Some(ref r_headers) = r_headers {
            headers.extend(r_headers.clone());
        }

        builder = match builder {
            SyncBuilder(ref inner) => SyncBuilder(inner.headers(headers)),

            #[cfg(feature = "async")]
            AsyncBuilder(ref a_inner) => AsyncBuilder(a_inner.headers(headers)),

            _ => unreachable!(),
        };

        Ok(builder)
    }

    /// (For sync usage) Creates a (blocking) RequestBuilder (`reqwest` crate) instance.
    pub fn build(&'a self, client: &Client) -> Result<RequestBuilder> {
        match self._generic_build(client, true) {
            Ok(SyncBuilder(builder)) => Ok(builder),
            Err(e) => Err(e),
            _ => unreachable!()
        }
    }

    /// (For async usage) Creates a (non-blocking) RequestBuilder (`reqwest` crate) instance.
    #[cfg(feature = "async")]
    pub fn a_build(&'a self, client: &Client) -> Result<ARequestBuilder> {
        match self._generic_build(client, false) {
            Ok(AsyncBuilder(builder)) => Ok(builder),
            Err(e) => Err(e),
            _ => unreachable!()
        }
    }
}
