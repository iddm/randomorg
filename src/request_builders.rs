use {
    AllowedCharacters, GenerateBlobsResult, GenerateDecimalFractionsResult,
    GenerateGaussiansResult, GenerateIntegersResult, GenerateStringsResult, GenerateUUIDsResult,
    Random, Response, Result,
};

macro_rules! builder {
    ($field:ident, $field_type:ty) => {
        /// A builder method
        pub fn $field(mut self, $field: $field_type) -> Self {
            self.$field = $field;
            self
        }
    };
}

/// A lazy integers request (builder)
pub struct RequestIntegers<'a> {
    client: &'a Random,
    min: i32,
    max: i32,
    limit: u16,
    replacement: bool,
}

impl<'a> RequestIntegers<'a> {
    /// Creates a lazy integers request (builder)
    pub fn new(client: &'a Random) -> RequestIntegers {
        RequestIntegers {
            client,
            min: 0i32,
            max: 100i32,
            limit: 10,
            replacement: true,
        }
    }

    builder!(min, i32);
    builder!(max, i32);
    builder!(limit, u16);
    builder!(replacement, bool);
}

/// Terminators
impl<'a> RequestIntegers<'a> {
    /// Collect the random integers (performs the request)
    pub fn collect<T: From<Response<GenerateIntegersResult>>>(self) -> Result<T> {
        Ok(T::from(self.client.generate_integers(
            self.min,
            self.max,
            self.limit,
            self.replacement,
        )?))
    }
}

/// A lazy decimal fractions request (builder)
pub struct RequestDecimalFractions<'a> {
    client: &'a Random,
    limit: u16,
    decimal_places: u8,
}

impl<'a> RequestDecimalFractions<'a> {
    /// Creates a lazy decimal fractions request (builder)
    pub fn new(client: &'a Random) -> RequestDecimalFractions {
        RequestDecimalFractions {
            client,
            limit: 10u16,
            decimal_places: 4u8,
        }
    }

    builder!(limit, u16);
    builder!(decimal_places, u8);
}

/// Terminators
impl<'a> RequestDecimalFractions<'a> {
    /// Collect the random decimal fractions (performs the request)
    pub fn collect<T: From<Response<GenerateDecimalFractionsResult>>>(self) -> Result<T> {
        Ok(T::from(self.client.generate_decimal_fractions(
            self.limit,
            self.decimal_places,
        )?))
    }
}

/// A lazy gaussians request (builder)
pub struct RequestGaussians<'a> {
    client: &'a Random,
    limit: u16,
    mean: i32,
    standard_deviation: i32,
    significant_digits: u8,
}

impl<'a> RequestGaussians<'a> {
    /// Creates a lazy gaussians request (builder)
    pub fn new(client: &'a Random) -> RequestGaussians {
        RequestGaussians {
            client,
            limit: 10u16,
            mean: 0i32,
            standard_deviation: 0i32,
            significant_digits: 0u8,
        }
    }

    builder!(limit, u16);
    builder!(mean, i32);
    builder!(standard_deviation, i32);
    builder!(significant_digits, u8);
}

/// Terminators
impl<'a> RequestGaussians<'a> {
    /// Collect the random gaussians (performs the request)
    pub fn collect<T: From<Response<GenerateGaussiansResult>>>(self) -> Result<T> {
        Ok(T::from(self.client.generate_gaussians(
            self.limit,
            self.mean,
            self.standard_deviation,
            self.significant_digits,
        )?))
    }
}

/// A lazy strings request (builder)
pub struct RequestStrings<'a> {
    client: &'a Random,
    limit: u16,
    length: u8,
    characters: AllowedCharacters,
}

impl<'a> RequestStrings<'a> {
    /// Creates a lazy strings request (builder)
    pub fn new(client: &'a Random) -> RequestStrings {
        use std::collections::BTreeSet;

        RequestStrings {
            client,
            limit: 10u16,
            length: 0u8,
            characters: AllowedCharacters("0123456789abcdef".chars().collect::<BTreeSet<char>>()),
        }
    }

    builder!(limit, u16);
    builder!(length, u8);
    builder!(characters, AllowedCharacters);
}

/// Terminators
impl<'a> RequestStrings<'a> {
    /// Collect the random strings (performs the request)
    pub fn collect<T: From<Response<GenerateStringsResult>>>(self) -> Result<T> {
        Ok(T::from(self.client.generate_strings(
            self.limit,
            self.length,
            self.characters,
        )?))
    }
}

/// A lazy UUIDs request (builder)
pub struct RequestUUIDs<'a> {
    client: &'a Random,
    limit: u16,
}

impl<'a> RequestUUIDs<'a> {
    /// Creates a lazy UUIDs request (builder)
    pub fn new(client: &'a Random) -> RequestUUIDs {
        RequestUUIDs {
            client,
            limit: 10u16,
        }
    }

    builder!(limit, u16);
}

/// Terminators
impl<'a> RequestUUIDs<'a> {
    /// Collect the random UUIDs (performs the request)
    pub fn collect<T: From<Response<GenerateUUIDsResult>>>(self) -> Result<T> {
        Ok(T::from(self.client.generate_uuids(self.limit)?))
    }
}

/// A lazy blobs request (builder)
pub struct RequestBlobs<'a> {
    client: &'a Random,
    limit: u16,
    size: u32,
}

impl<'a> RequestBlobs<'a> {
    /// Creates a lazy blobs request (builder)
    pub fn new(client: &'a Random) -> RequestBlobs {
        RequestBlobs {
            client,
            limit: 10u16,
            size: 128u32,
        }
    }

    builder!(limit, u16);
    builder!(size, u32);
}

/// Terminators
impl<'a> RequestBlobs<'a> {
    /// Collect the random blobs (performs the request)
    pub fn collect<T: From<Response<GenerateBlobsResult>>>(self) -> Result<T> {
        Ok(T::from(self.client.generate_blobs(self.limit, self.size)?))
    }
}
