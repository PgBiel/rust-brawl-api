use std::result::Result as StdResult;
use std::error::Error as StdError;
use std::time::SystemTime;
use serde::Deserialize;
use serde_json::{Error as SerdeError, Value as JsonValue};
use url::ParseError as UrlError;
use reqwest::blocking::Response;
use reqwest::{
    Error as ReqwestError, StatusCode, Response as AResponse,
    header::{InvalidHeaderValue, HeaderMap}
};
use std::fmt::{Formatter, Display};
use crate::util::JsonMap;
use crate::traits::Initializable;

/// Represents a `brawl-api` Result type.
pub type Result<T> = StdResult<T, Error>;

/// Represents all possible errors while using methods from this lib (`brawl-api`).
#[derive(Debug, PartialEq)]
pub enum Error {
    /// Represents an error occurred while using `serde_json` for serializing/deserializing JSON
    /// data. (A `serde_json` crate error)
    Json(SerdeError),

    /// Represents an error indicating a malformed URL.
    Url(UrlError),

    /// Represents an error indicating that an invalid Authorization header was specified.
    /// This error can be caused by the user of this lib when an invalid auth key is given.
    /// (Note that invalid auth key is covered by the Status error, but if it is given in an
    /// invalid format - which is impossible to be a valid key -, this error may appear.)
    /// Contains the invalid header value inside (as the `.0` field).
    Authorization(InvalidHeaderValue),

    /// Represents an error occurred while requesting to the API or while receiving its data.
    /// (A `reqwest` crate error)
    Request(ReqwestError),

    /// Represents an API ratelimit.
    Ratelimited {
        /// Maximum amount of requests per minute allowed. None indicates this was not given.
        limit: Option<usize>,

        /// Amount remaining (this should normally be 0). None indicates this was not given
        remaining: Option<usize>,

        /// Stringified timestamp (seconds) at which the ratelimit block will be lifted, or None
        /// for not ratelimited. This is only an Option in case a change is needed, considering
        /// that this will always be a `Some(String)` if this specific error is raised.
        time_until_reset: Option<String>,
    },

    /// Represents a JSON decoding error, with a description and the offending value.
    Decode(&'static str, JsonValue),

    /// Represents an arbitrary status code error received from the API.
    /// E.g. 400, 403, 404, 429, 500, 503
    ///
    /// - Field `.0` is the status code object;
    /// - Field `.1` is an optional instance of [`APIError`], if it may be parsed like so;
    /// - Field `.2` is the raw error response as parsed json, if it had that format.
    /// (If field .2 is None, that implies field .1 is None, since an APIError comes from a JSON
    /// object).
    ///
    /// [`APIError`]: ./error/struct.APIError.html
    Status(StatusCode, Option<APIError>, Option<JsonValue>),

    #[doc(hidden)]
    _AntiExhaustive,
}

/// Represents an error given by the API, with its specifications.
#[derive(Deserialize)]
pub struct APIError {
    /// The reason for the error.
    pub reason: String,

    /// Optionally, a human-readable message for the error.
    pub message: Option<String>,

    /// Optionally, a specific type of this error.
    pub err_type: Option<String>,

    /// Optionally, any extra details about this error.
    pub detail: Option<JsonMap>,
}

impl Initializable for APIError {
    fn new() -> APIError {
        APIError {
            reason: String::from(""),
            message: None,
            err_type: None,
            detail: None,
        }
    }
}

impl From<SerdeError> for Error {
    fn from(err: SerdeError) -> Error {
        Error::Json(err)
    }
}

impl From<ReqwestError> for Error {
    fn from(err: ReqwestError) -> Error {
        Error::Request(err)
    }
}

impl From<UrlError> for Error {
    fn from(err: UrlError) -> Error {
        Error::Url(err)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Error::Json(ref e) => e.fmt(f),
            Error::Request(ref e) => e.fmt(f),
            _ => f.write_str(self.description()),
            // _ => f.write_str(self.description())
        }
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Json(ref e) => e.description(),

            Error::Authorization(_) => "Auth key was provided in an invalid format for a header.",

            Error::Url(_) => "Invalid URL was given/built.",

            Error::Request(ref e) => e.description(),

            Error::Decode(msg, _) => msg,

            Error::Status(ref status, _, _) => status.canonical_reason().unwrap_or(
                "Unknown HTTP status code error received"
            ),

            Error::_AntiExhaustive => unreachable!("May not use the '_AntiExhaustive' variant."),
        }
    }

    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match *self {
            Error::Json(ref e) => Some(e),
            Error::Url(ref e) => Some(e),
            Error::Request(ref e) => Some(e),
            _ => None,
        }
    }
}

impl Error {
    #[doc(hidden)]
    pub(crate) fn from_response(response: Response, value: Option<JsonValue>) -> Error {
        let status = response.status();
        let value: Option<JsonValue> = match value {
            Some(val) => Some(val),
            None => ::serde_json::from_reader(response).ok()
        };

        let headers: HeaderMap = response.headers();
        let reset_header = headers.get("x-ratelimit-reset");
        if let Some(reset_header) = reset_header {  // ratelimited
            let reset_header = reset_header.to_str();
            if let Ok(reset) = reset_header {
                return Error::Ratelimited {
                    limit: match headers.get("x-ratelimit-limit") {
                        Some(lim_header) => lim_header.to_str().unwrap_or(
                            "Not a number, to provoke a None"
                        ).parse().ok(),
                        None => None,
                    },

                    remaining: match headers.get("x-ratelimit-remaining") {
                        Some(rem_header) => rem_header.to_str().unwrap_or(
                            "Not a number, to provoke a None"
                        ).parse().ok(),
                        None => None,
                    },

                    time_until_reset: Some(String::from(reset)),
                }
            }
        }

        Error::Status(status, value)
    }
}
