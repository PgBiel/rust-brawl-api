use std::result::Result as StdResult;
use std::error::Error as StdError;
use serde_json::{Error as SerdeError, Value as JsonValue, Map as JsonMap};
use reqwest::{Error as ReqwestError, StatusCode, Response};
use std::fmt::{Formatter, Display};

/// Represents a `brawl-api` Result type.
pub type Result<T> = StdResult<T, Error>;

/// Represents all possible errors while using methods from this lib (`brawl-api`).
#[derive(Debug, PartialEq)]
pub enum Error {
    /// Represents an error occurred while using `serde_json` for serializing/deserializing JSON
    /// data. (A `serde_json` crate error)
    Json(SerdeError),

    /// Represents an error occurred while requesting to the API or while receiving its data.
    /// (A `reqwest` crate error)
    Request(ReqwestError),

    /// Represents a JSON decoding error, with a description and the offending value/
    Decode(&'static str, JsonValue),

    /// Represents an arbitrary status code error received from the API.
    /// E.g. 400, 403, 404, 429, 500, 503
    ///
    /// - Field .0 is the status code object;
    /// - Field .1 is an optional instance of [`APIError`], if it may be parsed like so;
    /// - Field .2 is the raw error response as parsed json, if it had that format.
    /// (If field .2 is None, that implies field .1 is None, since an APIError comes from a JSON
    /// object).
    ///
    /// [`APIError`]: ./error/struct.APIError.html
    Status(StatusCode, Option<APIError>, Option<JsonValue>),
}

/// Represents an error given by the API, with its specifications.
pub struct APIError {
    /// The reason for the error.
    pub reason: String,

    /// Optionally, a human-readable message for the error.
    pub message: Option<String>,

    /// Optionally, a specific type of this error.
    pub err_type: Option<String>,

    /// Optionally, any extra details about this error.
    pub detail: Option<JsonMap<String, JsonValue>>
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
            Error::Request(ref e) => e.description(),

            Error::Status(ref status, _, _) => status.canonical_reason().unwrap_or(
                "Unknown HTTP status code error received"
            ),

            Error::Decode(msg, _) => msg,
        }
    }

    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match *self {
            Error::Json(ref e) => Some(e),
            Error::Request(ref e) => Some(e),
            _ => None,
        }
    }
}

impl Error {
    #[doc(hidden)]
    pub fn from_response(response: Response) -> Error {
        let status = response.status();
        let value = ::serde_json::from_reader(response).ok();
        if status == ::hyper::status::StatusCode::TooManyRequests {
            if let Some(JsonValue::Object(ref map)) = value {
                if let Some(delay) = map.get("retry_after").and_then(|v| v.as_u64()) {
                    return Error::RateLimited(delay)
                }
            }
        }
        Error::Status(status, value)
    }
}
