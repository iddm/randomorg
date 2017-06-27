#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum Method {
    /// This method generates true random integers within a user-defined range.
    #[serde(rename = "generateIntegers")]
    GenerateIntegers,
    /// This method generates true random decimal fractions from a uniform distribution across
    /// the [0,1] interval with a user-defined number of decimal places.
    #[serde(rename = "generateDecimalFractions")]
    GenerateDecimalFractions,
    /// This method generates true random numbers from a Gaussian distribution (also known as a
    /// normal distribution). The form uses a Box-Muller Transform to generate the Gaussian
    /// distribution from uniformly distributed numbers.
    #[serde(rename = "generateGaussians")]
    GenerateGaussians,
    /// This method generates true random strings.
    #[serde(rename = "generateStrings")]
    GenerateStrings,
    /// This method generates version 4 true random Universally Unique IDentifiers (UUIDs) in
    /// accordance with section 4.4 of RFC 4122.
    #[serde(rename = "generateUUIDs")]
    GenerateUUIDs,
    /// This method generates Binary Large OBjects (BLOBs) containing true random data.
    #[serde(rename = "generateBlobs")]
    GenerateBlobs,
    /// This method returns information related to the the usage of a given API key.
    #[serde(rename = "getUsage")]
    GetUsage,
}
