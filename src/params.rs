use crate::model::ApiKey;

/// A random.org api key params
#[derive(Debug, Clone, serde::Serialize)]
pub struct ApiKeyParams {
    /// An api key
    #[serde(rename = "apiKey")]
    pub api_key: ApiKey,
}

/// A random.org `generateIntegers` method params
#[derive(Debug, Clone, serde::Serialize)]
pub struct GenerateIntegersParams {
    /// An api key
    #[serde(rename = "apiKey")]
    pub api_key: ApiKey,
    /// The lower boundary for the range from which the random numbers will be picked. Must be
    /// within the [-1e9,1e9] range.
    pub min: i32,
    /// The upper boundary for the range from which the random numbers will be picked. Must be
    /// within the [-1e9,1e9] range.
    pub max: i32,
    #[serde(rename = "n")]
    /// How many random integers you need. Must be within the [1,1e4] range.
    pub limit: u16,
    /// Specifies whether the random numbers should be picked with replacement. The default (true)
    /// will cause the numbers to be picked with replacement, i.e., the resulting numbers may
    /// contain duplicate values (like a series of dice rolls). If you want the numbers picked to
    /// be unique (like raffle tickets drawn from a container), set this value to false.
    pub replacement: bool,
    /*
    /// Specifies the base that will be used to display the numbers. Values allowed are 2, 8, 10
    /// and 16. This affects the JSON types and formatting of the resulting data as discussed
    /// below.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base: Option<u8>,
    */
}

/// A random.org `generateDecimalFractions` method params
#[derive(Debug, Clone, serde::Serialize)]
pub struct GenerateDecimalFractionsParams {
    /// An api key
    #[serde(rename = "apiKey")]
    pub api_key: ApiKey,
    #[serde(rename = "n")]
    /// How many random decimal fractions you need. Must be within the [1,1e4] range.
    pub limit: u16,
    /// The number of decimal places to use. Must be within the [1,20] range.
    #[serde(rename = "decimalPlaces")]
    pub decimal_places: u8,
}

/// A random.org `generateGaussians` method params
#[derive(Debug, Clone, serde::Serialize)]
pub struct GenerateGaussiansParams {
    /// An api key
    #[serde(rename = "apiKey")]
    pub api_key: ApiKey,
    /// How many random numbers you need. Must be within the [1,1e4] range.
    #[serde(rename = "n")]
    pub limit: u16,
    /// The distribution's mean. Must be within the [-1e6,1e6] range.
    pub mean: i32,
    /// The distribution's standard deviation. Must be within the [-1e6,1e6] range.
    #[serde(rename = "standardDeviation")]
    pub standard_deviation: i32,
    /// The number of significant digits to use. Must be within the [2,20] range.
    #[serde(rename = "significantDigits")]
    pub significant_digits: u8,
}

/// A random.org `generateStrings` method params
#[derive(Debug, Clone, serde::Serialize)]
pub struct GenerateStringsParams {
    /// An api key
    #[serde(rename = "apiKey")]
    pub api_key: ApiKey,
    /// How many random strings you need. Must be within the [1,1e4] range.
    #[serde(rename = "n")]
    pub limit: u16,
    /// The length of each string. Must be within the [1,20] range. All strings will be of the
    /// same length
    pub length: u8,
    /// A string that contains the set of characters that are allowed to occur in the random
    /// strings. The maximum number of characters is 80.
    pub characters: String,
}

/// A random.org `generateUUIDs` method params
#[derive(Debug, Clone, serde::Serialize)]
pub struct GenerateUUIDsParams {
    /// An api key
    #[serde(rename = "apiKey")]
    pub api_key: ApiKey,
    /// How many random UUIDs you need. Must be within the [1,1e3] range.
    #[serde(rename = "n")]
    pub limit: u16,
}

/// A random.org `generateBlobs` method params
#[derive(Debug, Clone, serde::Serialize)]
pub struct GenerateBlobsParams {
    /// An api key
    #[serde(rename = "apiKey")]
    pub api_key: ApiKey,
    /// How many random blobs you need. Must be within the [1,100] range.
    #[serde(rename = "n")]
    pub limit: u16,
    /// The size of each blob, measured in bits. Must be within the [1,1048576] range and must be
    /// divisible by 8.
    pub size: u32,
}
