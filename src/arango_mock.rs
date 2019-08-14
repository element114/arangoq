use super::*;

use std::collections::HashMap;

#[allow(dead_code)] // used in test
pub struct ArangoMock {
    requests_and_responses: HashMap<String, String>,
}

impl ArangoMock {
    #[allow(dead_code)] // used in test
    pub fn new(requests_and_responses: HashMap<String, String>) -> Self {
        Self {
            requests_and_responses,
        }
    }
}

impl ExecuteArangoQuery for ArangoMock {
    type Output = String;

    fn execute_query(&self, query: ArangoQuery) -> <Self as ExecuteArangoQuery>::Output {
        self.requests_and_responses
            .get(&serde_json::to_string(&query).unwrap())
            .map(|s| s.to_owned())
            .unwrap_or_default()
    }
}
