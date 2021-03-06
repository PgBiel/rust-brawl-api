//! Contains the `Client` class, responsible for API authentication.

use reqwest::blocking::{
    Client as ReqClient, ClientBuilder as ReqClientBuilder,
    RequestBuilder
};

#[cfg(feature = "async")]
use reqwest::{
    Client as AReqClient, ClientBuilder as AReqClientBuilder,
    RequestBuilder as ARequestBuilder
};

use crate::constants::USER_AGENT as BRAWL_USER_AGENT;
use crate::http::request::Request;
use crate::error::Result;

#[derive(Debug, Clone)]
pub struct Client {
    pub auth_key: String,
    pub(crate) inner: ReqClient,

    #[cfg(feature = "async")]
    pub(crate) a_inner: AReqClient,
}

/// Represents an HTTP client which holds the user's API auth key, and is required on every fetch
/// method for authentication. This is usually the starting point for using this library.
///
/// See the [`Client::new`] method to start.
///
/// [`Client::new`]: #method.new
impl Client {
    /// Creates a new Client with a given API auth key.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use brawl_api::Client;
    ///
    /// let my_client = Client::new("my auth key");
    /// ```
    pub fn new(auth_key: &str) -> Client {
        let inner_b: ReqClientBuilder = ReqClient::builder().user_agent(BRAWL_USER_AGENT);

        #[cfg(feature = "async")]
        let a_inner_b: AReqClientBuilder = AReqClient::builder().user_agent(BRAWL_USER_AGENT);

        Client {
            auth_key: String::from(auth_key),
            inner: inner_b.build().unwrap(),

            #[cfg(feature = "async")]
            a_inner: a_inner_b.build().unwrap(),
        }
    }

    /// (For sync usage) Provides an immutable reference to the [`inner`] field.
    ///
    /// [`inner`]: #structfield.inner
    pub fn inner(&self) -> &ReqClient { &self.inner }

    /// (For sync usage) Provides a mutable reference to the [`inner`] field.
    ///
    /// [`inner`]: #structfield.inner
    pub fn inner_mut(&mut self) -> &mut ReqClient { &mut self.inner }

    /// (For async usage) Provides an immutable reference to the [`a_inner`] field.
    ///
    /// [`a_inner`]: #structfield.a_inner
    #[cfg(feature = "async")]
    pub fn a_inner(&self) -> &AReqClient { &self.a_inner }

    /// (For async usage) Provides a mutable reference to the [`a_inner`] field.
    ///
    /// [`a_inner`]: #structfield.a_inner
    #[cfg(feature = "async")]
    pub fn a_inner_mut(&mut self) -> &mut AReqClient { &mut self.a_inner }

    /// Creates a Request instance for one specific endpoint and returns it.
    pub fn endpoint_request(&self, endpoint: &str) -> Request<'_> {
        let mut req = Request::<'_>::default();
        req.endpoint = String::from(endpoint);
        req
    }

    /// (For sync usage) Creates a Request instance for one specific endpoint and calls
    /// [`Request::build`] on the newly-made instance, returning a (blocking) `RequestBuilder`.
    /// (GET)
    pub(crate) fn build_endpoint_get(&self, endpoint: &str) -> Result<RequestBuilder> {
        self.endpoint_request(endpoint).build(&self)
    }

    /// (For async usage) Creates a Request instance for one specific endpoint and calls
    /// [`Request::build`] on the newly-made instance, returning a (non-blocking) `RequestBuilder`.
    /// (GET)
    #[cfg(feature = "async")]
    pub(crate) fn a_build_endpoint_get(&self, endpoint: &str) -> Result<ARequestBuilder> {
        self.endpoint_request(endpoint).a_build(&self)
    }
}
