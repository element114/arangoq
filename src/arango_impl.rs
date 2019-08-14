use super::*;

use maplit::*;

use std::collections::BTreeMap;

#[allow(dead_code)]
impl ArangoQuery {
    pub(crate) fn new(query: &str) -> Self {
        Self {
            query: String::from(query),
            ..Default::default()
        }
    }

    pub fn with_bind_vars(query: &str, bind_vars: BTreeMap<String, Value>) -> Self {
        Self {
            query: String::from(query),
            bind_vars,
        }
    }
}

impl Default for ArangoQuery {
    fn default() -> Self {
        Self {
            query: String::default(),
            bind_vars: BTreeMap::new(),
        }
    }
}

impl Collection {
    pub fn new(name: &str, collection_type: CollectionType) -> Self {
        Self {
            name: String::from(name),
            collection_type,
        }
    }
}

impl Insert for Collection {
    fn insert<Elem: Serialize>(&self, elem: &Elem) -> ArangoQuery {
        ArangoQuery::with_bind_vars(
            "INSERT @value INTO @@collection",
            btreemap![
                String::from("@collection") => Value::String(self.name.to_owned()),
                String::from("value") => serde_json::to_value(elem).unwrap(),
            ],
        )
    }
}

impl GetAll for Collection {
    fn get_all(&self) -> ArangoQuery {
        ArangoQuery::with_bind_vars(
            "FOR item in @@collection RETURN item",
            btreemap![String::from("@collection") => Value::String(self.name.to_owned())],
        )
    }
}

impl GetByKey for Collection {
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
    fn replace<Key: Serialize, Elem: Serialize>(&self, key: Key, elem: Elem) -> ArangoQuery {
        ArangoQuery::with_bind_vars(
            "REPLACE @key WITH @elem IN @@collection",
            btreemap![
                String::from("@collection") => Value::String(self.name.to_owned()),
                String::from("elem") => serde_json::to_value(&elem).unwrap(),
                String::from("key") => serde_json::to_value(&key).unwrap(),
            ],
        )
    }
}

impl Update for Collection {
    fn update<Key: Serialize, Update: Serialize>(&self, key: Key, update: Update) -> ArangoQuery {
        ArangoQuery::with_bind_vars(
            "UPDATE @key WITH @update IN @@collection",
            btreemap![
                String::from("@collection") => Value::String(self.name.to_owned()),
                String::from("key") => serde_json::to_value(&key).unwrap(),
                String::from("update") => serde_json::to_value(&update).unwrap(),
            ],
        )
    }
}

impl Remove for Collection {
    fn remove<Key: Serialize>(&self, key: Key) -> ArangoQuery {
        ArangoQuery::with_bind_vars(
            "REMOVE @key IN @@collection",
            btreemap![
                String::from("@collection") => Value::String(self.name.to_owned()),
                String::from("key") => serde_json::to_value(&key).unwrap()
            ],
        )
    }
}

impl Truncate for Collection {
    fn truncate(&self) -> ArangoQuery {
        ArangoQuery::with_bind_vars(
            "FOR item IN @@collection REMOVE item IN @@collection",
            btreemap![String::from("@collection") => Value::String( self.name.to_owned())],
        )
    }
}
