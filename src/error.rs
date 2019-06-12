use reqwest::Error as ReqwestError;
use serde_json::Error as JsonError;
use std::error::Error as StdError;
use std::fmt::Display;
use std::io::Error as IoError;

/// Random.org API `Result` alias type.
pub type Result<T> = ::std::result::Result<T, Error>;

/// Random.org error code.
#[derive(Debug, Clone, Deserialize)]
pub struct ErrorCode(pub u64);

/// Random.org response error definition.
///
/// If an error occurred, this member contains a service-specific error object with details
/// about the error. If no error occurred, this member is not included in the response.
#[derive(Debug, Clone, Deserialize)]
pub struct ResponseError {
    /// A numeric error code that uniquely identifies the error type.
    pub code: ErrorCode,
    /// A string containing a human-readable error message in English suitable for printing in a
    /// log file or as part of an error message to be displayed to a user.
    pub message: String,
    /*
    /// Any values that the client needs to construct its own error message, for example in a
    /// different language than English.
    // pub data:
     */
}

/// Random.org API crate error type.
#[derive(Debug)]
pub enum Error {
    /// A `reqwest` crate error
    Reqwest(ReqwestError),
    /// A `serde_json` crate error
    Json(JsonError),
    /// A `std::io` module error
    Io(IoError),
    /// A error common toornament service error
    RandomOrg(::reqwest::StatusCode, ResponseError),
    /// A generic non-success response from the REST API
    Status(::reqwest::StatusCode, String),
    /// A rest-api error
    Rest(&'static str),
}

impl From<::reqwest::Response> for Error {
    fn from(mut response: ::reqwest::Response) -> Error {
        use std::io::Read;

        #[derive(Deserialize)]
        pub struct ErrorObject {
            error: ResponseError,
        }

        let status = response.status();
        let mut body = String::new();
        let _ = response.read_to_string(&mut body);
        if !status.is_success() {
            if let Ok(e) = ::serde_json::from_str::<ErrorObject>(&body) {
                return Error::RandomOrg(status, e.error);
            }
        }
        Error::Status(status, body)
    }
}

impl From<IoError> for Error {
    fn from(err: IoError) -> Error {
        Error::Io(err)
    }
}

impl From<ReqwestError> for Error {
    fn from(err: ReqwestError) -> Error {
        Error::Reqwest(err)
    }
}

impl From<JsonError> for Error {
    fn from(err: JsonError) -> Error {
        Error::Json(err)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            Error::Reqwest(ref inner) => inner.fmt(f),
            Error::Json(ref inner) => inner.fmt(f),
            Error::Io(ref inner) => inner.fmt(f),
            _ => f.write_str(self.description()),
        }
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Reqwest(ref inner) => inner.description(),
            Error::Json(ref inner) => inner.description(),
            Error::Io(ref inner) => inner.description(),
            Error::Rest(msg) => msg,
            Error::RandomOrg(status, _) | Error::Status(status, _) => status
                .canonical_reason()
                .unwrap_or("Unknown bad HTTP status"),
        }
    }

    fn cause(&self) -> Option<&StdError> {
        match *self {
            Error::Reqwest(ref inner) => Some(inner),
            Error::Json(ref inner) => Some(inner),
            Error::Io(ref inner) => Some(inner),
            _ => None,
        }
    }
}
