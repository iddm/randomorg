use model::*;
use params::*;
use methods::Method;

const API_JSON_RPC_VERSION: &'static str = "2.0";


/// Empty request - has nothing but api key inside.
/// Used in `getUsage` method.
pub type EmptyRequest = Request<ApiKeyParams>;
impl EmptyRequest {
    pub fn new(method: Method, api_key: ApiKey) -> EmptyRequest {
        EmptyRequest {
            json_rpc: API_JSON_RPC_VERSION.to_owned(),
            method: method,
            params: ApiKeyParams { api_key: api_key },
            id: RequestId(1),
        }
    }
}

/// A request for `generateIntegers` method.
pub type GenerateIntegersRequest = Request<GenerateIntegersParams>;
impl GenerateIntegersRequest {
    pub fn new(api_key: ApiKey,
               min: i32,
               max: i32,
               limit: u16,
               replacement: bool)
        -> GenerateIntegersRequest {
        GenerateIntegersRequest {
            json_rpc: API_JSON_RPC_VERSION.to_owned(),
            method: Method::GenerateIntegers,
            params: GenerateIntegersParams {
                api_key: api_key,
                min: min,
                max: max,
                limit: limit,
                replacement: replacement,
            },
            id: RequestId(1),
        }
    }
}


/// A request for `generateDecimalFractions` method.
pub type GenerateDecimalFractionsRequest = Request<GenerateDecimalFractionsParams>;
impl GenerateDecimalFractionsRequest {
    pub fn new(api_key: ApiKey,
               limit: u16,
               decimal_places: u8)
        -> GenerateDecimalFractionsRequest {
        GenerateDecimalFractionsRequest {
            json_rpc: API_JSON_RPC_VERSION.to_owned(),
            method: Method::GenerateDecimalFractions,
            params: GenerateDecimalFractionsParams {
                api_key: api_key,
                limit: limit,
                decimal_places: decimal_places,
            },
            id: RequestId(1),
        }
    }
}


/// A request for `generateGaussians` method.
pub type GenerateGaussiansRequest = Request<GenerateGaussiansParams>;
impl GenerateGaussiansRequest {
    pub fn new(api_key: ApiKey,
               limit: u16,
               mean: i32,
               standard_deviation: i32,
               significant_digits: u8)
        -> GenerateGaussiansRequest {
        GenerateGaussiansRequest {
            json_rpc: API_JSON_RPC_VERSION.to_owned(),
            method: Method::GenerateGaussians,
            params: GenerateGaussiansParams {
                api_key: api_key,
                limit: limit,
                mean: mean,
                standard_deviation: standard_deviation,
                significant_digits: significant_digits,
            },
            id: RequestId(1),
        }
    }
}


/// A request for `generateStrings` method.
pub type GenerateStringsRequest = Request<GenerateStringsParams>;
impl GenerateStringsRequest {
    pub fn new(api_key: ApiKey, limit: u16, length: u8, characters: AllowedCharacters)
        -> GenerateStringsRequest {
        GenerateStringsRequest {
            json_rpc: API_JSON_RPC_VERSION.to_owned(),
            method: Method::GenerateStrings,
            params: GenerateStringsParams {
                api_key: api_key,
                limit: limit,
                length: length,
                characters: characters.0.iter().collect::<String>(),
            },
            id: RequestId(1),
        }
    }
}


/// A request for `generateUUIDs` method.
pub type GenerateUUIDsRequest = Request<GenerateUUIDsParams>;
impl GenerateUUIDsRequest {
    pub fn new(api_key: ApiKey, limit: u16)
        -> GenerateUUIDsRequest {
        GenerateUUIDsRequest {
            json_rpc: API_JSON_RPC_VERSION.to_owned(),
            method: Method::GenerateUUIDs,
            params: GenerateUUIDsParams {
                api_key: api_key,
                limit: limit,
            },
            id: RequestId(1),
        }
    }
}


/// A request for `generateBlobs` method.
pub type GenerateBlobsRequest = Request<GenerateBlobsParams>;
impl GenerateBlobsRequest {
    pub fn new(api_key: ApiKey, limit: u16, size: u32)
        -> GenerateBlobsRequest {
        GenerateBlobsRequest {
            json_rpc: API_JSON_RPC_VERSION.to_owned(),
            method: Method::GenerateBlobs,
            params: GenerateBlobsParams {
                api_key: api_key,
                limit: limit,
                size: size,
            },
            id: RequestId(1),
        }
    }
}
