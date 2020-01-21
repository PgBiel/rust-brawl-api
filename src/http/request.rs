use reqwest::blocking::{
    RequestBuilder
};
use reqwest::RequestBuilder as ARequestBuilder;
use reqwest::{
    header::{
        HeaderMap
    }
};
use crate::error::Result;
use crate::traits::Initializable;
use crate::http::Client;

/// A struct representing a request to some endpoint.
#[derive(Debug, Clone, PartialEq)]
pub struct Request<'a> {

    /// The body of the request. (Note that this is rarely, if ever, used in this lib.)
    pub body: Option<&'a [u8]>,

    /// The headers of the request.
    pub headers: Option<HeaderMap>,

    /// The endpoint (e.g. /players/%23sometag).
    pub endpoint: String,
}

impl<'a> Initializable for Request<'a> {
    fn new() -> Request<'a> {
        Request {
            body: None,
            headers: None,
            endpoint: String::from(""),
        }
    }
}

impl<'a> Request<'a> {
    pub fn build(&'a self, client: &Client) -> Result<RequestBuilder> {
        let Request {
            body,
            headers: ref r_headers,
            endpoint: ref r_endpoint,
        } = *self;


    }

    pub fn a_build(&'a self, client: &Client) -> Result<ARequestBuilder> {
        let Request {
            body,
            headers: ref r_headers,
            endpoint: ref r_endpoint,
        } = *self;


    }
}
