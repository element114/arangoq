use crate::ArangoQuery;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;

use reqwest::{Body, Client};

impl From<ArangoQuery> for Body {
    fn from(item: ArangoQuery) -> Self {
        let b = serde_json::to_vec(&item).unwrap();
        b.into()
    }
}

/// Check https://www.arangodb.com/docs/stable/http/database.html
#[derive(Clone)]
pub struct ArangoConnection {
    pub host: Arc<String>,
    pub database: Arc<String>,
    pub client: Arc<Client>,
    // pub phantom: PhantomData<T>,
    pub context: Arc<Context>,
}
impl ArangoConnection {
    pub fn new(host: String, database: String, client: Client) -> Self {
        Self::with_context(host, database, client, Context::default())
    }
    pub fn with_context(host: String, database: String, client: Client, context: Context) -> Self {
        ArangoConnection {
            host: Arc::new(host),
            database: Arc::new(database),
            client: Arc::new(client),
            // phantom: PhantomData::<T>,
            context: Arc::new(context),
        }
    }
    pub fn cursor(&self) -> String {
        format!("{}/_db/{}/_api/cursor", self.host, self.database)
    }
    pub fn collection(&self) -> String {
        format!("{}/_db/{}/_api/collection", self.host, self.database)
    }
}

/// This struct contains all the props the db might include on top of user defined ones.
///
/// The _extra_ HashMap handles the case when a new property is defined in the collection,
/// but the rust sturct is not yet updated to handle that.
/// This is mandatory to be able to replace running services granularly, instead of full halt.
/// Avoids a panic in the old code by deserializing to _extra_.
///
/// During document create, _key,_id,_rev,_oldRev should be striped.
/// This is done by skip_serializing_if = "String::is_empty" if these are left empty.
///
/// For update like operations and get, _ker or _id is required,
/// in that case do not leave them empty or else these might be removed.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
pub struct CollectionMandatory {
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub(crate) _key: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub(crate) _id: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub(crate) _rev: String,
    #[serde(rename = "_oldRev", skip_serializing_if = "String::is_empty", default)]
    pub(crate) _old_rev: String,

    #[serde(flatten)]
    pub(crate) extra: HashMap<String, Value>,
}

impl CollectionMandatory {
    pub fn with_key(_key: &str) -> Self {
        Self { _key: _key.to_owned(), ..Default::default() }
    }

    pub fn id(&self) -> &str {
        &self._id
    }

    pub fn key(&self) -> &str {
        &self._key
    }
}

#[derive(Debug, PartialEq, Clone, Default)]
pub struct Context {
    pub app_prefix: &'static str,
}
impl Context {
    /// app_prefix is used to store collections of the same name for different apps using the same db
    /// This function returns the final collection name
    pub fn collection_name(&self, local_name: &str) -> String {
        format!("{}_{}", self.app_prefix, local_name)
    }
}
