use reqwest::blocking::{
    Client as ReqClient, ClientBuilder as ReqClientBuilder,
    RequestBuilder
};
use reqwest::{
    Client as AReqClient, ClientBuilder as AReqClientBuilder,
    RequestBuilder as ARequestBuilder
};
use crate::constants::USER_AGENT as BRAWL_USER_AGENT;
use crate::http::request::Request;
use crate::error::Result;

pub struct Client {
    pub auth_key: String,
    inner: ReqClient,

    #[cfg(feature = "async")]
    a_inner: AReqClient,
}

impl Client {
    fn new(auth_key: &str) -> Client {
        let mut inner_b: ReqClientBuilder = ReqClient::builder();
        let mut a_inner_b: AReqClientBuilder = AReqClient::builder();

        inner_b.user_agent(BRAWL_USER_AGENT);
        a_inner_b.user_agent(BRAWL_USER_AGENT);

        Client {
            auth_key: String::from(auth_key),
            inner: inner_b.build().unwrap(),
            a_inner: a_inner_b.build().unwrap(),
        }
    }

    fn inner(&self) -> &ReqClient { &self.inner }
    fn a_inner(&self) -> &AReqClient { &self.a_inner }
    
    /// Creates a Request instance for one specific endpoint and returns it.
    fn endpoint_request(&self, endpoint: &str) -> Request<'_> {
        Request::<'_>::new()
    }

    /// (Sync) Creates a Request instance for one specific endpoint and calls [`Request::build`]
    /// on the newly-made instance, returning a (blocking) `RequestBuilder`.
    fn build_endpoint_request(&self, endpoint: &str) -> Result<RequestBuilder> {
        self.endpoint_request(endpoint).build(&self)
    }

    /// (Async) Creates a Request instance for one specific endpoint and calls [`Request::build`]
    /// on the newly-made instance, returning a (non-blocking) `RequestBuilder`.
    #[cfg(feature = "async")]
    fn a_build_endpoint_request(&self, endpoint: &str) -> Result<ARequestBuilder> {
        self.endpoint_request(endpoint).a_build(&self)
    }
}
