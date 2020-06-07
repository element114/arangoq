use super::{
    btreemap, ArangoConnection, ArangoQuery, ArangoResponse, Collection, CollectionType,
    CursorExtractor, GetAll, GetByKey, GetByKeys, Insert, Remove, Replace, Truncate, Update,
};
use core::future::Future;
use futures::future::TryFutureExt;
use serde::Serialize;
// use maplit::*;
use serde::de::DeserializeOwned;
use serde_json::value::Value;
use std::collections::BTreeMap;

#[allow(dead_code)]
impl ArangoQuery {
    #[must_use]
    pub(crate) fn new(query: &str) -> Self {
        Self { query: String::from(query), ..Self::default() }
    }

    #[must_use]
    /// Same as raw, but with &str
    pub fn with_bind_vars(query: &str, bind_vars: BTreeMap<String, Value>) -> Self {
        Self { query: String::from(query), bind_vars, ..Self::default() }
    }

    #[must_use]
    /// ```ignore
    /// let mut bind_vars = std::collections::BTreeMap::new();
    /// bind_vars.insert(
    ///     "@conactcoll".to_owned(),
    ///     serde_json::Value::String(conactcoll.clone()),
    /// );
    /// bind_vars.insert("email".to_owned(), serde_json::Value::String("aaa@bbb.ccc"));
    /// let raw_query = "FOR c IN @@conactcoll FOR b IN @@balancecoll FILTER c.email == @email RETURN b";
    /// let query = ArangoQuery::raw(raw_query.to_owned(), bind_vars);
    /// match query.try_exec::<CreditBalance>(&conn).await {
    ///    Ok(ar) => ar.result,
    ///    Err(_) => vec![],
    ///}
    /// ```
    pub fn raw(query: String, bind_vars: BTreeMap<String, Value>) -> Self {
        ArangoQuery { query, bind_vars, ..Self::default() }
    }

    #[must_use]
    /// Returns results in batches of size `batch_size`
    /// ```ignore
    /// let query = ArangoQuery::raw_batched(raw_query.to_owned(), bind_vars, 100);
    /// ```
    pub fn raw_batched(
        query: String,
        bind_vars: BTreeMap<String, Value>,
        batch_size: usize,
    ) -> Self {
        ArangoQuery { query, bind_vars, batch_size: Some(batch_size) }
    }

    #[must_use]
    /// Converts an existing query to `batched` of size `batch_size`
    pub fn into_batched(self, batch_size: usize) -> Self {
        Self { query: self.query, bind_vars: self.bind_vars, batch_size: Some(batch_size) }
    }

    /// Executes this query using the provided `ArangoConnection`.
    /// Returns `ArangoResponse`
    /// # Errors
    ///
    /// Returns `reqwest::Error`
    /// Note: `reqwest::Error` is temporarily exposed and may change in the future.
    pub fn try_exec<T: Serialize + DeserializeOwned>(
        &self,
        dbc: &ArangoConnection,
    ) -> impl Future<Output = Result<ArangoResponse<T>, reqwest::Error>> {
        let nm = format!("{:?}", self);
        dbc.client
            .post(dbc.cursor().as_str())
            .header("content-type", "application/json")
            .json(self)
            .basic_auth(
                // TODO add this to ArangoConnection as well
                std::env::var("ARANGO_USER_NAME").unwrap_or_default(),
                std::env::var("ARANGO_PASSWORD").ok(),
            )
            .send()
            .and_then(reqwest::Response::json)
            .map_err(move |err| {
                log::debug!("Error during db request: {} Query: {:?}", err, nm);
                err
            })
    }
}

impl CursorExtractor {
    /// TODO: document this
    pub fn next<T: Serialize + DeserializeOwned>(
        &self,
        dbc: &ArangoConnection,
    ) -> impl Future<Output = Result<ArangoResponse<T>, reqwest::Error>> {
        let nm = format!("{:?}", self);
        dbc.client
            .put(&format!["{}/{}", dbc.cursor().as_str(), self.0])
            .basic_auth(
                // TODO add this to ArangoConnection as well
                std::env::var("ARANGO_USER_NAME").unwrap_or_default(),
                std::env::var("ARANGO_PASSWORD").ok(),
            )
            .send()
            .and_then(reqwest::Response::json)
            .map_err(move |err| {
                log::debug!("Error during db request: {} Query: {:?}", err, nm);
                err
            })
    }
}

impl Collection {
    #[must_use]
    /// ```ignore
    /// let coll = Collection::new(coll.as_str(), CollectionType::Document);
    /// ```
    pub fn new(name: &str, collection_type: CollectionType) -> Self {
        Self {
            id: String::new(),
            status: 0,
            is_system: false,
            globally_unique_id: String::from(name),
            name: String::from(name),
            collection_type,
        }
    }
}

impl Insert for Collection {
    /// ```ignore
    /// let query = coll.insert(&data);
    /// ```
    fn insert<Elem: Serialize>(&self, elem: &Elem) -> ArangoQuery {
        ArangoQuery::with_bind_vars(
            "INSERT @value INTO @@collection RETURN NEW",
            btreemap![
                String::from("@collection") => Value::String(self.name.to_owned()),
                String::from("value") => serde_json::to_value(elem).unwrap(),
            ],
        )
    }
}

impl GetAll for Collection {
    /// ```ignore
    /// let query = coll.get_all();
    /// ```
    fn get_all(&self) -> ArangoQuery {
        ArangoQuery::with_bind_vars(
            "FOR item in @@collection RETURN item",
            btreemap![String::from("@collection") => Value::String(self.name.to_owned())],
        )
    }
}

impl GetByKey for Collection {
    /// ```ignore
    /// let query = coll.get_by_key(key);
    /// ```
    fn get_by_key<Key: Serialize>(&self, key: Key) -> ArangoQuery {
        ArangoQuery::with_bind_vars(
            "RETURN DOCUMENT(@@collection, @key)",
            btreemap![
                String::from("@collection") => Value::String(self.name.to_owned()),
                String::from("key") => serde_json::to_value(&key).unwrap()
            ],
        )
    }
}

impl GetByKeys for Collection {
    /// ```ignore
    /// let query = coll.get_by_keys(&["key1","key2"]);
    /// ```
    fn get_by_keys<Key: Serialize>(&self, keys: &[Key]) -> ArangoQuery {
        ArangoQuery::with_bind_vars(
            "RETURN DOCUMENT(@@collection, @keys)",
            btreemap![
                String::from("@collection") => Value::String(self.name.to_owned()),
                String::from("keys") => serde_json::to_value(&keys).unwrap()
            ],
        )
    }
}

impl Replace for Collection {
    /// ```ignore
    /// let query = coll.replace("Paul", &TestUser::new("John Lennon"));
    /// ```
    fn replace<Key: Serialize, Elem: Serialize>(&self, key: Key, elem: Elem) -> ArangoQuery {
        ArangoQuery::with_bind_vars(
            "REPLACE @key WITH @elem IN @@collection RETURN NEW",
            btreemap![
                String::from("@collection") => Value::String(self.name.to_owned()),
                String::from("elem") => serde_json::to_value(&elem).unwrap(),
                String::from("key") => serde_json::to_value(&key).unwrap(),
            ],
        )
    }

    /// ```ignore
    /// let query = coll.replace_with_id("Paul", &Instrument { instrument: String::from("bass") });
    /// ```
    fn replace_with_id<Id: Serialize, Replace: Serialize>(
        &self,
        id: Id,
        replace: Replace,
    ) -> ArangoQuery {
        ArangoQuery::with_bind_vars(
            "LET doc = DOCUMENT(@id) REPLACE doc WITH @replace IN @@collection RETURN NEW",
            btreemap![
                String::from("@collection") => Value::String(self.name.to_owned()),
                String::from("id") => serde_json::to_value(&id).unwrap(),
                String::from("replace") => serde_json::to_value(&replace).unwrap(),
            ],
        )
    }
}

impl Update for Collection {
    /// ```ignore
    /// let query = coll.update("Paul", &Instrument { instrument: String::from("bass") });
    /// ```
    fn update<Key: Serialize, Update: Serialize>(&self, key: Key, update: Update) -> ArangoQuery {
        ArangoQuery::with_bind_vars(
            "UPDATE @key WITH @update IN @@collection RETURN NEW",
            btreemap![
                String::from("@collection") => Value::String(self.name.to_owned()),
                String::from("key") => serde_json::to_value(&key).unwrap(),
                String::from("update") => serde_json::to_value(&update).unwrap(),
            ],
        )
    }

    /// ```ignore
    /// let query = coll.update_with_id("Paul", &Instrument { instrument: String::from("bass") });
    /// ```
    fn update_with_id<Id: Serialize, Update: Serialize>(
        &self,
        id: Id,
        update: Update,
    ) -> ArangoQuery {
        ArangoQuery::with_bind_vars(
            "LET doc = DOCUMENT(@id) UPDATE doc WITH @update IN @@collection RETURN NEW",
            btreemap![
                String::from("@collection") => Value::String(self.name.to_owned()),
                String::from("id") => serde_json::to_value(&id).unwrap(),
                String::from("update") => serde_json::to_value(&update).unwrap(),
            ],
        )
    }
}

impl Remove for Collection {
    /// ```ignore
    /// let query = coll.remove("Paul");
    /// ```
    fn remove<Key: Serialize>(&self, key: Key) -> ArangoQuery {
        ArangoQuery::with_bind_vars(
            "REMOVE @key IN @@collection RETURN OLD",
            btreemap![
                String::from("@collection") => Value::String(self.name.to_owned()),
                String::from("key") => serde_json::to_value(&key).unwrap()
            ],
        )
    }
}

impl Truncate for Collection {
    /// ```ignore
    /// let query = coll.truncate();
    /// ```
    fn truncate(&self) -> ArangoQuery {
        ArangoQuery::with_bind_vars(
            "FOR item IN @@collection REMOVE item IN @@collection",
            btreemap![String::from("@collection") => Value::String( self.name.to_owned())],
        )
    }
}
