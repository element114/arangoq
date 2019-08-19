use super::*;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ArangoResponse<T> {
    result: Vec<T>,

    #[serde(rename = "hasMore")]
    has_more: bool,

    cached: bool,

    extra: ArangoResponseExtra,

    error: bool,

    code: u16,
}

impl<T> ArangoResponse<T> {
    pub fn new(
        result: Vec<T>,
        has_more: bool,
        cached: bool,
        extra: ArangoResponseExtra,
        error: bool,
        code: u16,
    ) -> Self {
        Self {
            result,
            has_more,
            cached,
            extra,
            error,
            code,
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ArangoResponseExtra {
    stats: ArangoStats,
    warnings: Vec<String>,
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
    writes_executed: usize,

    #[serde(rename = "writesIgnored")]
    writes_ignored: usize,

    #[serde(rename = "scannedFull")]
    scanned_full: usize,

    #[serde(rename = "scannedIndex")]
    scanned_index: usize,

    filtered: usize,

    #[serde(rename = "httpRequests")]
    http_requests: usize,

    #[serde(rename = "executionTime")]
    execution_time: f64,

    #[serde(rename = "peakMemoryUsage")]
    peak_memory_usage: usize,
}
