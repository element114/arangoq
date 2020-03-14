use super::*;

#[derive(Debug, Default, Serialize, PartialEq)]
pub struct ArangoQuery {
    pub(crate) query: String,

    #[serde(skip_serializing_if = "BTreeMap::is_empty", rename = "bindVars")]
    pub(crate) bind_vars: BTreeMap<String, Value>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "batchSize")]
    pub(crate) batch_size: Option<usize>,
}

pub struct CursorExtractor(pub String);

pub trait ExecuteArangoQuery {
    type Output;

    fn execute_query(&self, query: ArangoQuery) -> Self::Output;
}

// pub trait ExecuteArangoQueryFut {
//     fn execute_query<T>(&self, query: ArangoQuery) -> Box<Future<Item = ArangoResponse<T>, Error = Error> + Send>;
// }

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(untagged)]
pub enum CollectionType {
    Document = 2,
    Edge = 3,
}
impl Default for CollectionType {
    fn default() -> Self {
        CollectionType::Document
    }
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
pub struct Collection {
    #[serde(rename = "type", default)]
    pub(crate) collection_type: CollectionType,

    pub id: String,
    pub name: String,
    pub status: u8,
    #[serde(rename = "isSystem", default)]
    pub is_system: bool,
    #[serde(rename = "globallyUniqueId", default)]
    pub globally_unique_id: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
pub struct Edge {
    pub(crate) _from: String,
    pub(crate) _to: String,

    #[serde(flatten)]
    pub(crate) mandatory: CollectionMandatory,
}

impl Edge {
    pub fn new(_from: &str, _to: &str) -> Self {
        Self { _from: _from.to_owned(), _to: _to.to_owned(), ..Default::default() }
    }
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

pub enum SortingDirection {
    Asc,
    Desc,
}

pub trait BuilderTag {}

pub trait Buildable: BuilderTag {}

pub trait Conditionable: BuilderTag {}

pub trait LogicallyOperatable: BuilderTag {}

pub trait Filterable: BuilderTag {}

pub trait Limitable: BuilderTag {}

pub trait UpdateWith: BuilderTag {}

pub trait Sortable: BuilderTag {}

pub struct EmptyBuilder;

pub struct CreateQuery;

pub struct ReadQuery;

pub struct UpdateQuery;

pub struct DeleteQuery;

pub struct Conditional;

pub struct Filtering;

pub struct LogicalOperator;

pub struct UpdateField;

pub struct Sorting;

impl BuilderTag for EmptyBuilder {}

impl BuilderTag for CreateQuery {}

impl BuilderTag for ReadQuery {}

impl BuilderTag for UpdateQuery {}

impl BuilderTag for DeleteQuery {}

impl BuilderTag for Conditional {}

impl BuilderTag for Filtering {}

impl BuilderTag for LogicalOperator {}

impl BuilderTag for UpdateField {}

impl BuilderTag for Sorting {}

impl Buildable for ReadQuery {}

impl Buildable for DeleteQuery {}

impl Buildable for CreateQuery {}

impl Buildable for UpdateField {}

impl Buildable for Conditional {}

impl Buildable for Sorting {}

impl Conditionable for LogicalOperator {}

impl Conditionable for Filtering {}

impl Filterable for ReadQuery {}

impl Filterable for UpdateQuery {}

impl Filterable for DeleteQuery {}

impl Filterable for Conditional {}

impl Filterable for Sorting {}

impl Limitable for ReadQuery {}

impl Limitable for Conditional {}

impl Limitable for Sorting {}

impl LogicallyOperatable for Conditional {}

impl UpdateWith for Conditional {}

impl UpdateWith for UpdateQuery {}

impl Sortable for ReadQuery {}

impl Sortable for Filtering {}
