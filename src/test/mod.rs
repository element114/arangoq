use crate::arango_response::ResponseExtra;
use serde::{Deserialize, Serialize};

mod arango_mock;

#[allow(unused_imports)] // used in test
pub use arango_mock::*;

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestResponse {
    #[serde(default = "Vec::new")]
    pub result: Vec<serde_json::Value>,

    #[serde(rename = "hasMore", default)]
    pub has_more: bool,
    #[serde(default)]
    pub cached: bool,
    #[serde(default)]
    pub extra: ResponseExtra,
    #[serde(default)]
    pub error: bool,
    #[serde(default)]
    pub code: u16,

    #[serde(rename = "errorMessage", skip_serializing_if = "String::is_empty", default)]
    pub error_message: String,
    #[serde(rename = "errorNum", skip_serializing, default)]
    pub error_num: u64,

    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub id: String,
}

impl TestResponse {
    #[must_use]
    pub fn new() -> Self {
        TestResponse::default()
    }

    #[must_use]
    pub fn with_results<T: Serialize>(data: &[T]) -> Self {
        let mut res = TestResponse::default();
        res.result = data.iter().map(|t| serde_json::to_value(t).unwrap()).collect();
        res.code = 201;
        res.extra.stats.execution_time = 0.000_365_495_681_762_695_3;
        res.extra.stats.peak_memory_usage = 2109;
        res
    }

    #[must_use]
    pub fn with_code(code: u16) -> Self {
        let mut res = TestResponse::default();
        res.code = code;
        res.extra.stats.execution_time = 0.000_365_495_681_762_695_3;
        res.extra.stats.peak_memory_usage = 2019;
        res
    }
}

impl<T: Serialize> From<crate::arango_response::ArangoResponse<T>> for TestResponse {
    fn from(ar: crate::arango_response::ArangoResponse<T>) -> Self {
        let result: Vec<serde_json::Value> =
            ar.result.iter().map(|t| serde_json::to_value(t).unwrap()).collect();
        TestResponse {
            result,
            has_more: ar.has_more,
            cached: ar.cached,
            extra: ar.extra,
            error: ar.error,
            code: ar.code,
            error_message: ar.error_message,
            error_num: ar.error_num,
            id: ar.id,
        }
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[derive(Serialize, Default)]
    struct TestData {
        name: String,
    }

    #[test]
    fn test_results() {
        let t = TestResponse::new();
        let r = serde_json::to_string(&t).unwrap();
        assert_eq!(
            r#"{"result":[],"hasMore":false,"cached":false,"extra":{"stats":{"writesExecuted":0,"writesIgnored":0,"scannedFull":0,"scannedIndex":0,"filtered":0,"httpRequests":0,"executionTime":0.0,"peakMemoryUsage":0},"warnings":[]},"error":false,"code":0}"#,
            r
        );

        // let t = TestResponse::with_results(vec!(TestData{ name: "John Doe".to_owned()}, TestData{ name: "Teszt Elek".to_owned()}));
        // let r = serde_json::to_string(&t).unwrap();
        // assert_eq!(
        //     r#"{"result":[{"name":"John Doe"},{"name":"Teszt Elek"}],"hasMore":false,"cached":false,"extra":{"stats":{"writesExecuted":0,"writesIgnored":0,"scannedFull":0,"scannedIndex":0,"filtered":0,"httpRequests":0,"executionTime":0.0003654956817626953,"peakMemoryUsage":2019},"warnings":[]},"error":false,"code":201}"#,
        //     r);

        let t = TestResponse::with_code(401);
        let r = serde_json::to_string(&t).unwrap();
        assert_eq!(
            r#"{"result":[],"hasMore":false,"cached":false,"extra":{"stats":{"writesExecuted":0,"writesIgnored":0,"scannedFull":0,"scannedIndex":0,"filtered":0,"httpRequests":0,"executionTime":0.0003654956817626953,"peakMemoryUsage":2019},"warnings":[]},"error":false,"code":401}"#,
            r
        );
    }
}
