use super::*;
use std::error::Error;
use std::fmt;

#[derive(Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ArangoResponse<T> {
    #[serde(default = "Vec::new")]
    pub result: Vec<T>,

    #[serde(rename = "hasMore", default)]
    pub has_more: bool,
    #[serde(default)]
    pub cached: bool,
    #[serde(default)]
    pub extra: ArangoResponseExtra,
    #[serde(rename = "error", default)]
    pub is_error: bool,
    #[serde(default)]
    pub code: u16,

    #[serde(flatten)]
    pub error: ArangoError,
}

#[allow(clippy::too_many_arguments)]
impl<T> ArangoResponse<T> {
    pub fn new(
        result: Vec<T>,
        has_more: bool,
        cached: bool,
        extra: ArangoResponseExtra,
        is_error: bool,
        code: u16,
        error: ArangoError,
    ) -> Self {
        Self { result, has_more, cached, extra, is_error, code, error }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ArangoResponseExtra {
    #[serde(default)]
    pub stats: ArangoStats,
    #[serde(default)]
    pub warnings: Vec<String>,
}

#[allow(clippy::too_many_arguments)]
impl ArangoResponseExtra {
    pub fn new(
        writes_executed: usize,
        writes_ignored: usize,
        scanned_full: usize,
        scanned_index: usize,
        filtered: usize,
        http_requests: usize,
        execution_time: f64,
        peak_memory_usage: usize,
        warnings: Vec<String>,
    ) -> Self {
        Self {
            stats: ArangoStats {
                writes_executed,
                writes_ignored,
                scanned_full,
                scanned_index,
                filtered,
                http_requests,
                execution_time,
                peak_memory_usage,
            },
            warnings,
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ArangoStats {
    #[serde(rename = "writesExecuted", default)]
    pub writes_executed: usize,

    #[serde(rename = "writesIgnored", default)]
    pub writes_ignored: usize,

    #[serde(rename = "scannedFull", default)]
    pub scanned_full: usize,

    #[serde(rename = "scannedIndex", default)]
    pub scanned_index: usize,
    #[serde(default)]
    pub filtered: usize,

    #[serde(rename = "httpRequests", default)]
    pub http_requests: usize,

    #[serde(rename = "executionTime", default)]
    pub execution_time: f64,

    #[serde(rename = "peakMemoryUsage", default)]
    pub peak_memory_usage: usize,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ArangoError {
    #[serde(rename = "errorMessage", skip_serializing_if = "String::is_empty", default)]
    pub error_message: String,
    #[serde(rename = "errorNum", default)]
    pub error_num: u64,
}
impl fmt::Display for ArangoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.error_num, self.error_message)
    }
}
impl Error for ArangoError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

impl From<reqwest::Error> for ArangoError {
    fn from(error: reqwest::Error) -> Self {
        ArangoError {
            error_message: error.to_string(),
            error_num: error.status().unwrap_or_default().as_u16() as u64,
        }
    }
}
