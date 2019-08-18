use crate::{ArangoQuery, ArangoResponse, ExecuteArangoQuery};
use futures::Future;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::marker::PhantomData;

#[cfg(feature = "actixclient")]
use actix_web::client::Client;
#[cfg(feature = "actixclient")]
use actix_web::{dev::Body, http::header, Error};

pub struct ArangoConnection<'a, T> {
    pub host: &'a str,
    pub client: Client,
    pub phantom: PhantomData<T>,
}

#[cfg(feature = "actixclient")]
impl From<ArangoQuery> for Body {
    fn from(item: ArangoQuery) -> Self {
        let b = serde_json::to_vec(&item).unwrap();
        Body::from_slice(&b)
    }
}

impl<T: Serialize + DeserializeOwned> ExecuteArangoQuery for ArangoConnection<'_,T> {
    type Output = Result<ArangoResponse<T>, Error>;

    fn execute_query(&self, query: ArangoQuery) -> Self::Output {
        #[cfg(feature = "actixclient")]
        {
            self.client
                .post(format!("{}/_api/cursor", self.host))
                .header(header::CONTENT_TYPE, "application/json")
                .send_body(query)
                .map_err(Error::from)
                .and_then(|mut response| response.json().map_err(Error::from).wait())
                .wait()
        }
    }
}

// TODO: give this a better name
pub fn _get_from_collection(
    // collection: &Collection,
    coll_name: &str,
    client: &Client,
    host: &str,
) -> impl Future<Item = serde_json::Value, Error = Error> {
    #[cfg(feature = "actixclient")]
    {
        let req = client.get(format!("{}/_api/collection/{}", host, coll_name));

        req.send()
            .map_err(Error::from)
            .and_then(|mut response| response.json().map_err(Error::from).wait())
    }
}

pub fn _db_query(
    client: &Client,
    host: &str,
    query: ArangoQuery,
) -> impl Future<Item = serde_json::Value, Error = Error> {
    #[cfg(feature = "actixclient")]
    {
        let req = client
            .post(format!("{}/_api/cursor", host))
            .header(header::CONTENT_TYPE, "application/json");

        req.send_body(query)
            .map_err(Error::from)
            .and_then(|mut response| response.json().map_err(Error::from).wait())
    }
}

// #[derive(Debug, Serialize, Deserialize)]
// pub struct DbResponse {
//     pub code: i64,
//     pub error: bool,
//     pub result: Vec<Result>,
// }

// #[derive(Debug, Serialize, Deserialize)]
// pub struct Result {
//     #[serde(rename = "globallyUniqueId")]
//     pub globally_unique_id: String,
//     pub id: String,
//     #[serde(rename = "isSystem")]
//     pub is_system: bool,
//     pub name: String,
//     pub status: i64,
//     #[serde(rename = "type")]
//     pub result_type: i64,
// }

// pub trait Queriable<D> {
//     type Result;

//     fn query(collection: Collection, client: &Client, host: &str, queriable: D) -> Self::Result;
// }

// impl<D: Remove> Queriable<D> for ArangoQuery {
//     type Result = Result<serde_json::Value,Error>;
//     fn query(collection: Collection, client: &Client, host: &str, queriable: D) -> Self::Result {
//         db_cmd(collection, client, host).wait()
//     }
// }

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
    _key: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    _id: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    _rev: String,
    #[serde(rename = "_oldRev", skip_serializing_if = "String::is_empty", default)]
    _old_rev: String,

    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

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
