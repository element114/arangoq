use super::*;

#[derive(Debug, Serialize, PartialEq)]
pub struct ArangoQuery {
    pub(crate) query: String,

    #[serde(skip_serializing_if = "BTreeMap::is_empty", rename = "bindVars")]
    pub(crate) bind_vars: BTreeMap<String, Value>,
}

pub trait ExecuteArangoQuery {
    type Output;

    fn execute_query(&self, query: ArangoQuery) -> Self::Output;
}

pub enum CollectionType {
    Document = 2,
    Edge = 3,
}

#[allow(dead_code)]
pub struct Collection {
    pub(crate) name: String,
    pub(crate) collection_type: CollectionType,
}

// Create

pub trait Insert {
    fn insert<Elem: Serialize>(&self, elem: &Elem) -> ArangoQuery;
}

// Read

pub trait GetAll {
    fn get_all(&self) -> ArangoQuery;
}

pub trait GetByKey {
    fn get_by_key<Key: Serialize>(&self, key: Key) -> ArangoQuery;
}

pub trait GetByKeys {
    fn get_by_keys<Key: Serialize>(&self, keys: &[Key]) -> ArangoQuery;
}

// Update

pub trait Replace {
    fn replace<Key: Serialize, Elem: Serialize>(&self, key: Key, elem: Elem) -> ArangoQuery;
}

pub trait Update {
    fn update<Key: Serialize, Update: Serialize>(&self, key: Key, update: Update) -> ArangoQuery;
}

// Delete

pub trait Remove {
    fn remove<Key: Serialize>(&self, key: Key) -> ArangoQuery;
}

pub trait Truncate {
    fn truncate(&self) -> ArangoQuery;
}

pub enum QueryType {
    Create,
    Read,
    Update,
    Delete,
}

pub trait BuilderTag {}

pub trait Buildable: BuilderTag {}

pub trait Conditionable: BuilderTag {}

pub trait LogicallyOperatable: BuilderTag {}

pub trait Filterable: BuilderTag {}

pub trait Limitable: BuilderTag {}

pub trait UpdateWith: BuilderTag {}

pub struct EmptyBuilder;

pub struct CreateQuery;

pub struct ReadQuery;

pub struct UpdateQuery;

pub struct DeleteQuery;

pub struct Conditional;

pub struct Filtering;

pub struct LogicalOperator;

pub struct UpdateField;

impl BuilderTag for EmptyBuilder {}

impl BuilderTag for CreateQuery {}

impl BuilderTag for ReadQuery {}

impl BuilderTag for UpdateQuery {}

impl BuilderTag for DeleteQuery {}

impl BuilderTag for Conditional {}

impl BuilderTag for Filtering {}

impl BuilderTag for LogicalOperator {}

impl BuilderTag for UpdateField {}

impl Buildable for ReadQuery {}

impl Buildable for DeleteQuery {}

impl Buildable for CreateQuery {}

impl Buildable for UpdateField {}

impl Buildable for Conditional {}

impl Conditionable for LogicalOperator {}

impl Conditionable for Filtering {}

impl Filterable for ReadQuery {}

impl Filterable for UpdateQuery {}

impl Filterable for DeleteQuery {}

impl Filterable for Conditional {}

impl Limitable for ReadQuery {}

impl Limitable for Conditional {}

impl LogicallyOperatable for Conditional {}

impl UpdateWith for Conditional {}

impl UpdateWith for UpdateQuery {}
