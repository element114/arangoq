use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ArangoResponse<T> {
    #[serde(default = "Vec::new")]
    pub result: Vec<T>,

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

#[cfg(test)]
#[allow(clippy::too_many_arguments)]
impl<T> ArangoResponse<T> {
    pub(crate) fn new(
        result: Vec<T>,
        has_more: bool,
        cached: bool,
        extra: ResponseExtra,
        error: bool,
        code: u16,
        error_message: String,
        error_num: u64,
        id: String,
    ) -> Self {
        Self { result, has_more, cached, extra, error, code, error_message, error_num, id }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResponseExtra {
    #[serde(default)]
    pub stats: ArangoStats,
    #[serde(default)]
    pub warnings: Vec<String>,
}

#[allow(clippy::too_many_arguments)]
impl ResponseExtra {
    #[must_use]
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
