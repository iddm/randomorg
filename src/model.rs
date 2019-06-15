use crate::methods::Method;
use serde::*;

use std::collections::BTreeSet;

/// An unique random.org request id
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct RequestId(pub u64);

/// A random.org api key
#[derive(Debug, Clone, Serialize)]
pub struct ApiKey(pub String);

/// A random.org api key status
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ApiKeyStatus {
    /// The key has been revoked.
    Stopped,
    /// The key has been paused, means it is not working, but not revoked.
    Paused,
    /// The key is valid by all means.
    Running,
}

/// Allowed characters
#[derive(Debug, Clone, Serialize)]
pub struct AllowedCharacters(pub BTreeSet<char>);

/// A random.org request holder
#[derive(Debug, Clone, Serialize)]
pub struct Request<Params: Serialize> {
    /// A json rpc version
    #[serde(rename = "jsonrpc")]
    pub json_rpc: String,
    /// A name of the method to be invoked
    pub method: Method,
    /// A structured value containing the parameters that will be supplied to the method.
    pub params: Params,
    /// A request identifier that allows the client to match responses to request. The service will
    /// return this unchanged in its response.
    pub id: RequestId,
}

/// A random.org response holder
#[derive(Debug, Clone, Deserialize)]
pub struct Response<ResponseResult> {
    /// A json rpc version
    #[serde(rename = "jsonrpc")]
    pub json_rpc: String,
    /// If no error occurred, this member contains the response from the service, typically random
    /// values and other associated data. If an error occurred, this member is not included in the
    /// response.
    pub result: ResponseResult,
    /// A request identifier that allows the client to match responses to request. The service
    /// will return this unchanged in its response.
    pub id: RequestId,
}
