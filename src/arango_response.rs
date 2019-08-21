use super::*;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ArangoResponse<T> {
    pub result: Vec<T>,

    #[serde(rename = "hasMore")]
    pub has_more: bool,

    pub cached: bool,

    pub extra: ArangoResponseExtra,

    pub error: bool,

    pub code: u16,

    #[serde(
        rename = "errorMessage",
        skip_serializing_if = "String::is_empty",
        default
    )]
    pub error_message: String,
    #[serde(rename = "errorNum", skip_serializing, default)]
    pub error_num: u64,
}

#[allow(clippy::too_many_arguments)]
impl<T> ArangoResponse<T> {
    pub fn new(
        result: Vec<T>,
        has_more: bool,
        cached: bool,
        extra: ArangoResponseExtra,
        error: bool,
        code: u16,
        error_message: String,
        error_num: u64,
    ) -> Self {
        Self {
            result,
            has_more,
            cached,
            extra,
            error,
            code,
            error_message,
            error_num,
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ArangoResponseExtra {
    pub stats: ArangoStats,
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

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ArangoStats {
    #[serde(rename = "writesExecuted")]
    pub writes_executed: usize,

    #[serde(rename = "writesIgnored")]
    pub writes_ignored: usize,

    #[serde(rename = "scannedFull")]
    pub scanned_full: usize,

    #[serde(rename = "scannedIndex")]
    pub scanned_index: usize,

    pub filtered: usize,

    #[serde(rename = "httpRequests")]
    pub http_requests: usize,

    #[serde(rename = "executionTime")]
    pub execution_time: f64,

    #[serde(rename = "peakMemoryUsage")]
    pub peak_memory_usage: usize,
}
