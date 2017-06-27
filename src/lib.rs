//! A random.org api bindings.
//! 
//! The randomness comes from atmospheric noise, which for many purposes is better than the
//! pseudo-random number algorithms typically used in computer programs. To use the library you
//! must have an api key which you may get [from here](https://api.random.org/api-keys/beta).
//! 
//! # Usage
//! 
//! ```rust,no_run
//! extern crate randomorg;
//! 
//! fn main() {
//!     use randomorg::Random;
//!     let r = Random::new("API KEY HERE").unwrap();
//!     println!("Result: {:?}", r.generate_integers(-100, 100, 15, true));
//!     let random_data = r.request_integers().min(0).max(100).limit(5).collect::<Vec<i32>>();
//!     println!("Random integers: {:?}", random_data);
//! }
//! ```

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate reqwest;
extern crate chrono;

use reqwest::header::ContentType;

mod date_de;
mod model;
mod methods;
mod error;
mod params;
mod requests;
mod results;
mod request_builders;
pub mod version;

use methods::Method;
use requests::{
    EmptyRequest,
    GenerateIntegersRequest,
    GenerateDecimalFractionsRequest,
    GenerateGaussiansRequest,
    GenerateStringsRequest,
    GenerateUUIDsRequest,
    GenerateBlobsRequest,
};
pub use request_builders::{
    RequestIntegers,
    RequestDecimalFractions,
    RequestGaussians,
};
pub use model::{
    ApiKey,
    ApiKeyStatus,
    AllowedCharacters,
    Request,
    RequestId,
    Response,
};
pub use results:: {
    GetUsageResult,
    RandomResult,
    RandomData,
    GenerateIntegersResult,
    GenerateDecimalFractionsResult,
    GenerateGaussiansResult,
    GenerateStringsResult,
    GenerateUUIDsResult,
    GenerateBlobsResult,
};
pub use error::{ Error, ErrorCode, ResponseError, Result };


const API_INVOKE: &'static str = "https://api.random.org/json-rpc/2/invoke";

fn retry<F: Fn() -> reqwest::RequestBuilder>(f: F)
    -> Result<reqwest::Response> {
    let f2 = || check_status(f().send());
    // retry on a ConnectionAborted, which occurs if it's been a while since the last request
    match f2() {
        Err(_) => f2(),
        other => other
    }
}

fn check_status(response: reqwest::Result<reqwest::Response>) -> Result<reqwest::Response> {
    let response = response?;
    if !response.status().is_success() {
        return Err(Error::from(response))
    }
    Ok(response)
}

/// Macro only for internal use with the `Random` object (relies on it's fields)
macro_rules! request {
    ($api:ident, $body:expr) => {
        {
            retry(|| $api.client.post(API_INVOKE)
                                .body($body)
                                .header(ContentType::json()))
        }
    };
}


/// A random.org api client.
pub struct Random {
    client: reqwest::Client,
    api_key: ApiKey,
}

impl Random {
    /// Creates new random.org client.
    /// 
    /// # Usage
    /// 
    /// ```rust,no_run
    /// extern crate randomorg;
    /// 
    /// fn main() {
    ///     use randomorg::Random;
    ///     let r = Random::new("API KEY HERE").unwrap();
    /// }
    /// ```
    pub fn new<S: Into<String>>(api_key: S) -> Result<Random> {
        Ok(Random {
            client: reqwest::Client::new()?,
            api_key: ApiKey(api_key.into()),
        })
    }

    /// Create a request object for generating random integers
    /// 
    /// # Usage
    ///
    /// ```rust,no_run
    /// extern crate randomorg;
    /// 
    /// fn main() {
    ///     use randomorg::Random;
    ///     let r = Random::new("API KEY HERE").unwrap();
    ///     let random_data = r.request_integers().min(0).max(100).limit(5).collect::<Vec<i32>>();
    ///     println!("Random integers: {:?}", random_data);
    /// }
    /// ```
    pub fn request_integers(&self) -> RequestIntegers {
        RequestIntegers::new(self)
    }

    /// Create a request object for generating random decimal fractions
    /// 
    /// # Usage
    ///
    /// ```rust,no_run
    /// extern crate randomorg;
    /// 
    /// fn main() {
    ///     use randomorg::Random;
    ///     let r = Random::new("API KEY HERE").unwrap();
    ///     let random_data = r.request_decimal_fractions().limit(5)
    ///                                                    .decimal_places(4)
    ///                                                    .collect::<Vec<f32>>();
    ///     println!("Random decimal fractions: {:?}", random_data);
    /// }
    /// ```
    pub fn request_decimal_fractions(&self) -> RequestDecimalFractions {
        RequestDecimalFractions::new(self)
    }

    /// Create a request object for generating random gaussians
    /// 
    /// # Usage
    ///
    /// ```rust,no_run
    /// extern crate randomorg;
    /// 
    /// fn main() {
    ///     use randomorg::Random;
    ///     let r = Random::new("API KEY HERE").unwrap();
    ///     let random_data = r.request_gaussians().limit(5)
    ///                                            .collect::<Vec<f32>>();
    ///     println!("Random gaussians: {:?}", random_data);
    /// }
    /// ```
    pub fn request_gaussians(&self) -> RequestGaussians {
        RequestGaussians::new(self)
    }

    /// This method generates true random integers within a user-defined range.
    /// 
    /// * [Official documentation](https://api.random.org/json-rpc/2/basic#generateIntegers)
    /// 
    /// # Usage
    /// 
    /// ```rust,no_run
    /// extern crate randomorg;
    /// 
    /// fn main() {
    ///     use randomorg::Random;
    ///     let r = Random::new("API KEY HERE").unwrap();
    ///     println!("Result: {:?}", r.generate_integers(-100, 100, 15, true));
    /// }
    /// ```
    /// 
    /// # Constraints
    /// * `min` must be within [-1e9; 1e9] range
    /// * `max` must be within [-1e9; 1e9] range
    /// * `limit` must be within [1; 1e4] range
    pub fn generate_integers(&self, min: i32, max: i32, limit: u16, replacement: bool)
        -> Result<Response<GenerateIntegersResult>> {
        let request = GenerateIntegersRequest::new(self.api_key.clone(),
                                                   min,
                                                   max,
                                                   limit,
                                                   replacement);
        let body = serde_json::to_string(&request)?;
        Ok(request!(self, body.as_str())?.json()?)
    }

    /// This method generates true random decimal fractions from a uniform distribution across
    /// the [0,1] interval with a user-defined number of decimal places.
    /// 
    /// * [Official documentation](https://api.random.org/json-rpc/2/basic#generateDecimalFractions)
    /// 
    /// # Usage
    /// 
    /// ```rust,no_run
    /// extern crate randomorg;
    /// 
    /// fn main() {
    ///     use randomorg::Random;
    ///     let r = Random::new("API KEY HERE").unwrap();
    ///     println!("Result: {:?}", r.generate_decimal_fractions(10, 8));
    /// }
    /// ```
    /// 
    /// # Constraints
    /// * `limit` must be within [1; 1e4] range
    /// * `decimal_places` must be within [1; 20] range
    pub fn generate_decimal_fractions(&self, limit: u16, decimal_places: u8)
        -> Result<Response<GenerateDecimalFractionsResult>> {
        let request = GenerateDecimalFractionsRequest::new(self.api_key.clone(),
                                                           limit,
                                                           decimal_places);
        let body = serde_json::to_string(&request)?;
        Ok(request!(self, body.as_str())?.json()?)
    }

    /// This method generates true random numbers from a Gaussian distribution (also known as a
    /// normal distribution). The form uses a Box-Muller Transform to generate the Gaussian
    /// distribution from uniformly distributed numbers.
    /// 
    /// * [Official documentation](https://api.random.org/json-rpc/2/basic#generateGaussians)
    /// 
    /// # Usage
    /// 
    /// ```rust,no_run
    /// extern crate randomorg;
    /// 
    /// fn main() {
    ///     use randomorg::Random;
    ///     let r = Random::new("API KEY HERE").unwrap();
    ///     println!("Result: {:?}", r.generate_gaussians(2000, 1100, 100, 4));
    /// }
    /// ```
    /// 
    /// # Constraints
    /// * `limit` must be within [1; 1e4] range
    /// * `mean` must be within [-1e6; 1e6] range
    /// * `standard_deviation` must be within [-1e6; 1e6] range
    /// * `significant_digits` must be within [2; 20] range
    pub fn generate_gaussians(&self,
                              limit: u16,
                              mean: i32,
                              standard_deviation: i32,
                              significant_digits: u8)
        -> Result<Response<GenerateGaussiansResult>> {
        let request = GenerateGaussiansRequest::new(self.api_key.clone(),
                                                    limit,
                                                    mean,
                                                    standard_deviation,
                                                    significant_digits);
        let body = serde_json::to_string(&request)?;
        Ok(request!(self, body.as_str())?.json()?)
    }

    /// This method generates true random strings.
    /// 
    /// * [Official documentation](https://api.random.org/json-rpc/2/basic#generateStrings)
    /// 
    /// # Usage
    /// 
    /// ```rust,no_run
    /// extern crate randomorg;
    /// 
    /// fn main() {
    ///     use randomorg::{ Random, AllowedCharacters };
    ///     use std::collections::BTreeSet;
    ///     let chars = "abcde".chars().collect::<BTreeSet<char>>();
    ///     let r = Random::new("API KEY HERE").unwrap();
    ///     println!("Result: {:?}", r.generate_strings(5, 10, AllowedCharacters(chars)));
    /// }
    /// ```
    /// 
    /// # Constraints
    /// * `limit` must be within [1; 1e4] range
    /// * `length` must be within [1; 20] range
    /// * `characters` must contain maximum 80 characters.
    pub fn generate_strings(&self, limit: u16, length: u8, characters: AllowedCharacters)
        -> Result<Response<GenerateStringsResult>> {
        let request = GenerateStringsRequest::new(self.api_key.clone(),
                                                  limit,
                                                  length,
                                                  characters);
        let body = serde_json::to_string(&request)?;
        Ok(request!(self, body.as_str())?.json()?)
    }

    /// This method generates version 4 true random Universally Unique IDentifiers (UUIDs) in
    /// accordance with section 4.4 of RFC 4122.
    /// 
    /// * [Official documentation](https://api.random.org/json-rpc/2/basic#generateUUIDs)
    /// 
    /// # Usage
    /// 
    /// ```rust,no_run
    /// extern crate randomorg;
    /// 
    /// fn main() {
    ///     use randomorg::Random;
    ///     let r = Random::new("API KEY HERE").unwrap();
    ///     println!("Result: {:?}", r.generate_uuids(5));
    /// }
    /// ```
    /// 
    /// # Constraints
    /// * `limit` must be within [1; 1e3] range
    pub fn generate_uuids(&self, limit: u16) -> Result<Response<GenerateUUIDsResult>> {
        let request = GenerateUUIDsRequest::new(self.api_key.clone(), limit);
        let body = serde_json::to_string(&request)?;
        Ok(request!(self, body.as_str())?.json()?)
    }

    /// This method generates Binary Large OBjects (BLOBs) containing true random data.
    /// 
    /// * [Official documentation](https://api.random.org/json-rpc/2/basic#generateBlobs)
    /// 
    /// # Usage
    /// 
    /// ```rust,no_run
    /// extern crate randomorg;
    /// 
    /// fn main() {
    ///     use randomorg::Random;
    ///     let r = Random::new("API KEY HERE").unwrap();
    ///     println!("Result: {:?}", r.generate_blobs(5, 16));
    /// }
    /// ```
    /// 
    /// # Constraints
    /// * `limit` must be within [1; 100] range
    /// * `size` must be within [1, 1048576] range
    pub fn generate_blobs(&self, limit: u16, size: u32) -> Result<Response<GenerateBlobsResult>> {
        let request = GenerateBlobsRequest::new(self.api_key.clone(), limit, size);
        let body = serde_json::to_string(&request)?;
        Ok(request!(self, body.as_str())?.json()?)
    }

    /// Returns information related to the usage of a given API key.
    /// 
    /// * [Official documentation](https://api.random.org/json-rpc/2/basic#getUsage)
    /// 
    /// # Usage
    /// 
    /// ```rust,no_run
    /// extern crate randomorg;
    /// 
    /// fn main() {
    ///     use randomorg::Random;
    ///     let r = Random::new("API KEY HERE").unwrap();
    ///     println!("Result: {:?}", r.get_usage());
    /// }
    /// ```
    pub fn get_usage(&self) -> Result<Response<GetUsageResult>> {
        let request = EmptyRequest::new(Method::GetUsage, self.api_key.clone());
        let body = serde_json::to_string(&request)?;
        Ok(request!(self, body.as_str())?.json()?)
    }
}


#[cfg(test)]
mod tests {
    fn assert_sync_and_send<T: Sync + Send>() {}

    #[test]
    fn test_sync_and_send() {
        assert_sync_and_send::<::Random>();
    }
}
