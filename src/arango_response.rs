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
        full_count: usize,
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
                full_count,
                peak_memory_usage,
            },
            warnings,
        }
    }
}

/// <https://www.arangodb.com/docs/stable/aql/execution-and-performance-query-statistics.html>
#[derive(Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ArangoStats {
    /// The total number of data-modification operations successfully executed.
    #[serde(rename = "writesExecuted", default)]
    pub writes_executed: usize,

    /// The total number of data-modification operations that were unsuccessful, but have been ignored because of query option `ignoreErrors`.
    #[serde(rename = "writesIgnored", default)]
    pub writes_ignored: usize,

    /// The total number of documents iterated over when scanning a collection without an index.
    #[serde(rename = "scannedFull", default)]
    pub scanned_full: usize,

    /// The total number of documents iterated over when scanning a collection using an index.
    #[serde(rename = "scannedIndex", default)]
    pub scanned_index: usize,

    /// The total number of documents that were removed after executing a filter condition.
    #[serde(default)]
    pub filtered: usize,

    #[serde(rename = "httpRequests", default)]
    pub http_requests: usize,

    #[serde(rename = "executionTime", default)]
    pub execution_time: f64,

    /// The total number of documents that matched the search condition.
    #[serde(rename = "fullCount", default)]
    pub full_count: usize,

    /// The maximum memory usage of the query while it was running.
    #[serde(rename = "peakMemoryUsage", default)]
    pub peak_memory_usage: usize,
}
