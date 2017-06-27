use chrono;
use model::ApiKeyStatus;
use date_de;


/// A random.org response of `getUsage` method.
#[derive(Debug, Clone, Deserialize)]
pub struct GetUsageResult {
    /// A string indicating the API key's current status, which may be stopped, paused or running.
    /// An API key must be running for it to be able to serve requests.
    pub status: ApiKeyStatus,
    /// A time at which the API key was created.
    #[serde(rename = "creationTime")]
    #[serde(deserialize_with = "date_de::deserialize")]
    pub creation_time: chrono::NaiveDateTime,
    /// An integer containing the (estimated) number of remaining true random bits available to
    /// the client.
    #[serde(rename = "bitsLeft")]
    pub bits_left: u64,
    /// An integer containing the (estimated) number of remaining API requests available
    /// to the client.
    #[serde(rename = "requestsLeft")]
    pub requests_left: u64,
    /// An integer containing the number of bits used by this API key since it was created.
    #[serde(rename = "totalBits")]
    pub total_bits: u64,
    /// An integer containing the number of requests used by this API key since it was created.
    #[serde(rename = "totalRequests")]
    pub total_requests: u64,
}

/// A random.org's produced random data (from `generateIntegers` method)
#[derive(Debug, Clone, Deserialize)]
pub struct RandomData<T> {
    /// An array containing the sequence of numbers requested.
    pub data: Vec<T>,
    /// A time at which the request was completed.
    #[serde(rename = "completionTime")]
    #[serde(deserialize_with = "date_de::deserialize")]
    pub completion_time: chrono::NaiveDateTime,
    // pub completion_time: chrono::DateTime<chrono::offset::Utc>,
}

/// A random.org response with random data.
#[derive(Debug, Clone, Deserialize)]
pub struct RandomResult<RandomDataType> {
    /// This object encapsulates the random numbers and associated data. It contains the following
    /// properties.
    pub random: RandomData<RandomDataType>,
    /// An integer containing the number of true random bits used to complete this request.
    #[serde(rename = "bitsUsed")]
    pub bits_used: u64,
    /// An integer containing the (estimated) number of remaining true random bits available to
    /// the client.
    #[serde(rename = "bitsLeft")]
    pub bits_left: u64,
    /// An integer containing the (estimated) number of remaining API requests available
    /// to the client.
    #[serde(rename = "requestsLeft")]
    pub requests_left: u64,
    /// An integer containing the recommended number of milliseconds that the client should delay
    /// before issuing another request.
    #[serde(rename = "advisoryDelay")]
    pub advisory_delay: u64,
}

/// A random.org response of `generateIntegers` method.
pub type GenerateIntegersResult = RandomResult<i32>;

/// A random.org response of `GenerateDecimalFractions` method.
pub type GenerateDecimalFractionsResult = RandomResult<f32>;

/// A random.org response of `GenerateGaussians` method.
pub type GenerateGaussiansResult = RandomResult<f32>;

/// A random.org response of `GenerateStrings` method.
pub type GenerateStringsResult = RandomResult<String>;

/// A random.org response of `GenerateUUIDs` method.
pub type GenerateUUIDsResult = RandomResult<String>;

/// A random.org response of `GenerateBlobs` method.
pub type GenerateBlobsResult = RandomResult<String>;


#[cfg(test)]
mod parse_tests {
    use ::serde_json;

    #[test]
    fn test_get_usage_response_parse() {
        use ::{ Response, GetUsageResult, ApiKeyStatus, RequestId, };
        use chrono::{ Datelike, Timelike };

        let s = r#"
        {
            "jsonrpc": "2.0",
            "result": {
                "status": "running",
                "creationTime": "2017-06-22 13:32:16Z",
                "bitsLeft": 250000,
                "requestsLeft": 1000,
                "totalBits": 0,
                "totalRequests": 0
            },
            "id": 1
        }
        "#;

        let u: Response<GetUsageResult> = serde_json::from_str(s).unwrap();

        assert_eq!(u.json_rpc, "2.0");
        assert_eq!(u.id, RequestId(1));
        assert_eq!(u.result.status, ApiKeyStatus::Running);
        assert_eq!(u.result.creation_time.year(), 2017i32);
        assert_eq!(u.result.creation_time.month(), 06u32);
        assert_eq!(u.result.creation_time.day(), 22u32);
        assert_eq!(u.result.creation_time.hour(), 13u32);
        assert_eq!(u.result.creation_time.minute(), 32u32);
        assert_eq!(u.result.creation_time.second(), 16u32);
        assert_eq!(u.result.bits_left, 250000);
        assert_eq!(u.result.requests_left, 1000);
        assert_eq!(u.result.total_bits, 0);
        assert_eq!(u.result.total_requests, 0);
    }

    #[test]
    fn test_generate_integers_response_parse() {
        use ::{ Response, GenerateIntegersResult, RequestId, };
        use chrono::{ Datelike, Timelike };

        let s = r#"
        {
            "jsonrpc": "2.0",
            "result": {
                "random": {
                    "data": [
                        1, 5, 4, 6, 6, 4
                    ],
                    "completionTime": "2011-10-10 13:19:12Z"
                },
                "bitsUsed": 16,
                "bitsLeft": 199984,
                "requestsLeft": 9999,
                "advisoryDelay": 0
            },
            "id": 42
        }
        "#;

        let u: Response<GenerateIntegersResult> = serde_json::from_str(s).unwrap();

        assert_eq!(u.json_rpc, "2.0");
        assert_eq!(u.id, RequestId(42));
        assert_eq!(u.result.random.data, vec![1, 5, 4, 6, 6, 4]);
        assert_eq!(u.result.random.completion_time.year(), 2011i32);
        assert_eq!(u.result.random.completion_time.month(), 10u32);
        assert_eq!(u.result.random.completion_time.day(), 10u32);
        assert_eq!(u.result.random.completion_time.hour(), 13u32);
        assert_eq!(u.result.random.completion_time.minute(), 19u32);
        assert_eq!(u.result.random.completion_time.second(), 12u32);
        assert_eq!(u.result.bits_used, 16);
        assert_eq!(u.result.bits_left, 199984);
        assert_eq!(u.result.requests_left, 9999);
        assert_eq!(u.result.advisory_delay, 0);
    }

    #[test]
    fn test_generate_decimal_fractions_response_parse() {
        use ::{ Response, GenerateDecimalFractionsResult, RequestId, };
        use chrono::{ Datelike, Timelike };

        let s = r#"
        {
            "jsonrpc": "2.0",
            "result": {
                "random": {
                    "data": [
                        0.0753205, 0.59823072, 0.46109946, 0.28453638, 0.92390558,
                        0.53087566, 0.48139983, 0.06829921, 0.1878, 0.10107864
                    ],
                    "completionTime": "2011-10-10 13:19:12Z"
                },
                "bitsUsed": 16,
                "bitsLeft": 199984,
                "requestsLeft": 9999,
                "advisoryDelay": 0
            },
            "id": 42
        }
        "#;

        let u: Response<GenerateDecimalFractionsResult> = serde_json::from_str(s).unwrap();

        assert_eq!(u.json_rpc, "2.0");
        assert_eq!(u.id, RequestId(42));
        assert_eq!(u.result.random.data, vec![0.0753205, 0.59823072, 0.46109946, 0.28453638,
                                              0.92390558, 0.53087566, 0.48139983, 0.06829921,
                                              0.1878, 0.10107864]);
        assert_eq!(u.result.random.completion_time.year(), 2011i32);
        assert_eq!(u.result.random.completion_time.month(), 10u32);
        assert_eq!(u.result.random.completion_time.day(), 10u32);
        assert_eq!(u.result.random.completion_time.hour(), 13u32);
        assert_eq!(u.result.random.completion_time.minute(), 19u32);
        assert_eq!(u.result.random.completion_time.second(), 12u32);
        assert_eq!(u.result.bits_used, 16);
        assert_eq!(u.result.bits_left, 199984);
        assert_eq!(u.result.requests_left, 9999);
        assert_eq!(u.result.advisory_delay, 0);
    }

    #[test]
    fn test_generate_gaussians_response_parse() {
        use ::{ Response, GenerateGaussiansResult, RequestId, };
        use chrono::{ Datelike, Timelike };

        let s = r#"
        {
            "jsonrpc": "2.0",
            "result": {
                "random": {
                    "data": [
                        0.4025041, -1.4918831, 0.64733849, 0.5222242
                    ],
                    "completionTime": "2011-10-10 13:19:12Z"
                },
                "bitsUsed": 16,
                "bitsLeft": 199984,
                "requestsLeft": 9999,
                "advisoryDelay": 0
            },
            "id": 42
        }
        "#;

        let u: Response<GenerateGaussiansResult> = serde_json::from_str(s).unwrap();

        assert_eq!(u.json_rpc, "2.0");
        assert_eq!(u.id, RequestId(42));
        assert_eq!(u.result.random.data, vec![0.4025041, -1.4918831, 0.64733849, 0.5222242]);
        assert_eq!(u.result.random.completion_time.year(), 2011i32);
        assert_eq!(u.result.random.completion_time.month(), 10u32);
        assert_eq!(u.result.random.completion_time.day(), 10u32);
        assert_eq!(u.result.random.completion_time.hour(), 13u32);
        assert_eq!(u.result.random.completion_time.minute(), 19u32);
        assert_eq!(u.result.random.completion_time.second(), 12u32);
        assert_eq!(u.result.bits_used, 16);
        assert_eq!(u.result.bits_left, 199984);
        assert_eq!(u.result.requests_left, 9999);
        assert_eq!(u.result.advisory_delay, 0);
    }

    #[test]
    fn test_generate_strings_response_parse() {
        use ::{ Response, GenerateStringsResult, RequestId, };
        use chrono::{ Datelike, Timelike };

        let s = r#"
        {
            "jsonrpc": "2.0",
            "result": {
                "random": {
                    "data": [
                        "grvhglvahj", "hjrmosjwed", "nivjyqptyy", "lhogeshsmi",
                        "syilbgsytb", "birvcmgdrz", "wgclyynpcq", "eujwnhgonh"
                    ],
                    "completionTime": "2011-10-10 13:19:12Z"
                },
                "bitsUsed": 16,
                "bitsLeft": 199984,
                "requestsLeft": 9999,
                "advisoryDelay": 0
            },
            "id": 42
        }
        "#;

        let u: Response<GenerateStringsResult> = serde_json::from_str(s).unwrap();

        assert_eq!(u.json_rpc, "2.0");
        assert_eq!(u.id, RequestId(42));
        assert_eq!(u.result.random.data, vec!["grvhglvahj", "hjrmosjwed", "nivjyqptyy",
                                              "lhogeshsmi", "syilbgsytb", "birvcmgdrz",
                                              "wgclyynpcq", "eujwnhgonh"]);
        assert_eq!(u.result.random.completion_time.year(), 2011i32);
        assert_eq!(u.result.random.completion_time.month(), 10u32);
        assert_eq!(u.result.random.completion_time.day(), 10u32);
        assert_eq!(u.result.random.completion_time.hour(), 13u32);
        assert_eq!(u.result.random.completion_time.minute(), 19u32);
        assert_eq!(u.result.random.completion_time.second(), 12u32);
        assert_eq!(u.result.bits_used, 16);
        assert_eq!(u.result.bits_left, 199984);
        assert_eq!(u.result.requests_left, 9999);
        assert_eq!(u.result.advisory_delay, 0);
    }

    #[test]
    fn test_generate_uuids_response_parse() {
        use ::{ Response, GenerateUUIDsResult, RequestId, };
        use chrono::{ Datelike, Timelike };

        let s = r#"
        {
            "jsonrpc": "2.0",
            "result": {
                "random": {
                    "data": [
                        "47849fd4-b790-492e-8b93-c601a91b662d"
                    ],
                    "completionTime": "2011-10-10 13:19:12Z"
                },
                "bitsUsed": 16,
                "bitsLeft": 199984,
                "requestsLeft": 9999,
                "advisoryDelay": 0
            },
            "id": 42
        }
        "#;

        let u: Response<GenerateUUIDsResult> = serde_json::from_str(s).unwrap();

        assert_eq!(u.json_rpc, "2.0");
        assert_eq!(u.id, RequestId(42));
        assert_eq!(u.result.random.data, vec!["47849fd4-b790-492e-8b93-c601a91b662d"]);
        assert_eq!(u.result.random.completion_time.year(), 2011i32);
        assert_eq!(u.result.random.completion_time.month(), 10u32);
        assert_eq!(u.result.random.completion_time.day(), 10u32);
        assert_eq!(u.result.random.completion_time.hour(), 13u32);
        assert_eq!(u.result.random.completion_time.minute(), 19u32);
        assert_eq!(u.result.random.completion_time.second(), 12u32);
        assert_eq!(u.result.bits_used, 16);
        assert_eq!(u.result.bits_left, 199984);
        assert_eq!(u.result.requests_left, 9999);
        assert_eq!(u.result.advisory_delay, 0);
    }

    #[test]
    fn test_generate_blobs_response_parse() {
        use ::{ Response, GenerateBlobsResult, RequestId, };
        use chrono::{ Datelike, Timelike };

        let s = r#"
        {
            "jsonrpc": "2.0",
            "result": {
                "random": {
                    "data": [
                        "aNB8L3hY3kWYXgTUQxGVB5njMe2e0l3LCjkDCN1u12kPBPrsDcWMLTCDlB60kRhAlGbvPqoBHhjg6ZbOM4LfD3T9/wfhvnqJ1FTraamW2IAUnyKxz27fgcPw1So6ToIBL0fGQLpMQDF2/nEmNmFRNa9s6sQ+400IGA+ZeaOAgjE="
            
                    ],
                    "completionTime": "2011-10-10 13:19:12Z"
                },
                "bitsUsed": 16,
                "bitsLeft": 199984,
                "requestsLeft": 9999,
                "advisoryDelay": 0
            },
            "id": 42
        }
        "#;

        let u: Response<GenerateBlobsResult> = serde_json::from_str(s).unwrap();

        assert_eq!(u.json_rpc, "2.0");
        assert_eq!(u.id, RequestId(42));
        assert_eq!(u.result.random.data, vec!["aNB8L3hY3kWYXgTUQxGVB5njMe2e0l3LCjkDCN1u12kPBPrsDcWMLTCDlB60kRhAlGbvPqoBHhjg6ZbOM4LfD3T9/wfhvnqJ1FTraamW2IAUnyKxz27fgcPw1So6ToIBL0fGQLpMQDF2/nEmNmFRNa9s6sQ+400IGA+ZeaOAgjE="]);
        assert_eq!(u.result.random.completion_time.year(), 2011i32);
        assert_eq!(u.result.random.completion_time.month(), 10u32);
        assert_eq!(u.result.random.completion_time.day(), 10u32);
        assert_eq!(u.result.random.completion_time.hour(), 13u32);
        assert_eq!(u.result.random.completion_time.minute(), 19u32);
        assert_eq!(u.result.random.completion_time.second(), 12u32);
        assert_eq!(u.result.bits_used, 16);
        assert_eq!(u.result.bits_left, 199984);
        assert_eq!(u.result.requests_left, 9999);
        assert_eq!(u.result.advisory_delay, 0);
    }

    #[test]
    fn test_incorrect_and_correct_dates_parse() {
        use ::{ Response, GenerateBlobsResult, RequestId, };
        use chrono::{ Datelike, Timelike };

        let correct = r#"
        {
            "jsonrpc": "2.0",
            "result": {
                "random": {
                    "data": [
                        "aNB8L3hY3kWYXgTUQxGVB5njMe2e0l3LCjkDCN1u12kPBPrsDcWMLTCDlB60kRhAlGbvPqoBHhjg6ZbOM4LfD3T9/wfhvnqJ1FTraamW2IAUnyKxz27fgcPw1So6ToIBL0fGQLpMQDF2/nEmNmFRNa9s6sQ+400IGA+ZeaOAgjE="
            
                    ],
                    "completionTime": "2011-10-10 13:19:12Z"
                },
                "bitsUsed": 16,
                "bitsLeft": 199984,
                "requestsLeft": 9999,
                "advisoryDelay": 0
            },
            "id": 42
        }
        "#;

        let incorrect = r#"
        {
            "jsonrpc": "2.0",
            "result": {
                "random": {
                    "data": [
                        "aNB8L3hY3kWYXgTUQxGVB5njMe2e0l3LCjkDCN1u12kPBPrsDcWMLTCDlB60kRhAlGbvPqoBHhjg6ZbOM4LfD3T9/wfhvnqJ1FTraamW2IAUnyKxz27fgcPw1So6ToIBL0fGQLpMQDF2/nEmNmFRNa9s6sQ+400IGA+ZeaOAgjE="
            
                    ],
                    "completionTime": "2011-10-10 13:19:12Z"
                },
                "bitsUsed": 16,
                "bitsLeft": 199984,
                "requestsLeft": 9999,
                "advisoryDelay": 0
            },
            "id": 42
        }
        "#;

        for s in vec![correct, incorrect] {
            let u: Response<GenerateBlobsResult> = serde_json::from_str(s).unwrap();

            assert_eq!(u.json_rpc, "2.0");
            assert_eq!(u.id, RequestId(42));
            assert_eq!(u.result.random.data, vec!["aNB8L3hY3kWYXgTUQxGVB5njMe2e0l3LCjkDCN1u12kPBPrsDcWMLTCDlB60kRhAlGbvPqoBHhjg6ZbOM4LfD3T9/wfhvnqJ1FTraamW2IAUnyKxz27fgcPw1So6ToIBL0fGQLpMQDF2/nEmNmFRNa9s6sQ+400IGA+ZeaOAgjE="]);
            assert_eq!(u.result.random.completion_time.year(), 2011i32);
            assert_eq!(u.result.random.completion_time.month(), 10u32);
            assert_eq!(u.result.random.completion_time.day(), 10u32);
            assert_eq!(u.result.random.completion_time.hour(), 13u32);
            assert_eq!(u.result.random.completion_time.minute(), 19u32);
            assert_eq!(u.result.random.completion_time.second(), 12u32);
            assert_eq!(u.result.bits_used, 16);
            assert_eq!(u.result.bits_left, 199984);
            assert_eq!(u.result.requests_left, 9999);
            assert_eq!(u.result.advisory_delay, 0);
        }
    }
}
