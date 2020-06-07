use crate::arango_api::{ArangoQuery, ExecuteArangoQuery};

use std::collections::HashMap;

#[allow(dead_code)] // used in test
pub struct ArangoMock {
    requests_and_responses: HashMap<String, String>,
}

impl ArangoMock {
    #[must_use]
    ///
    /// ```
    /// use maplit::hashmap;
    /// use arangoq::test::{ArangoMock, TestResponse};
    /// use arangoq::arango_api::{Collection, CollectionType, GetByKey, ExecuteArangoQuery};
    ///
    /// let t = TestResponse::new();
    /// let test_response_json = serde_json::to_string(&t).unwrap();
    ///
    /// let query = || Collection::new("Characters", CollectionType::Document).get_by_key("13221");
    /// let query_json = || serde_json::to_string(&query()).unwrap();
    /// let test_mock = ArangoMock::new(hashmap![query_json() => test_response_json.clone()]);
    /// assert_eq!(test_response_json, test_mock.execute_query(query()));
    /// ```
    #[allow(dead_code)] // used in test
    pub fn new(requests_and_responses: HashMap<String, String>) -> Self {
        Self { requests_and_responses }
    }
}

impl ExecuteArangoQuery for ArangoMock {
    type Output = String;

    fn execute_query(&self, query: ArangoQuery) -> <Self as ExecuteArangoQuery>::Output {
        self.requests_and_responses
            .get(&serde_json::to_string(&query).unwrap())
            .map(ToOwned::to_owned)
            .unwrap_or_default()
    }
}
