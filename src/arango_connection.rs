use crate::{ArangoQuery, ArangoResponse};
use futures::Future;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::marker::PhantomData;
use std::sync::Arc;

#[cfg(feature = "actixclient")]
use actix_web::client::Client;
#[cfg(feature = "actixclient")]
use actix_web::{dev::Body, http::header, Error};

#[cfg(feature = "reqwestasyc")]
use reqwest::r#async::{Body, Client};
#[cfg(all(feature = "reqwestactor", not(feature = "reqwestasyc")))]
use reqwest::{Body, Client};
#[cfg(not(feature = "actixclient"))]
use reqwest::Error;

#[cfg(feature = "actixclient")]
impl From<ArangoQuery> for Body {
    fn from(item: ArangoQuery) -> Self {
        let b = serde_json::to_vec(&item).unwrap();
        Body::from_slice(&b)
    }
}

#[cfg(any(feature = "reqwestasyc", feature = "reqwestactor"))]
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
    pub client: Arc<Client>,
    // pub phantom: PhantomData<T>,
}
impl ArangoConnection {
    pub fn new(host: String, client: Client) -> Self {
        ArangoConnection {
            host: Arc::new(host),
            client: Arc::new(client),
            // phantom: PhantomData::<T>,
        }
    }
}

pub struct ArangoConnectionInternal<T> {
    pub conn: ArangoConnection,
    phantom: PhantomData<T>,
}

// impl<T: Serialize + DeserializeOwned> ExecuteArangoQuery for ArangoConnectionInternal<T> {
    // type Output = Result<ArangoResponse<T>, Error>;
impl<T: Serialize + DeserializeOwned> ArangoConnectionInternal<T> {
    pub fn execute_query(&self, query: ArangoQuery) -> impl Future<Item = ArangoResponse<T>, Error = Error> {
        #[cfg(feature = "actixclient")]
        {
            self.conn
                .client
                .post(format!("{}/_api/cursor", self.conn.host))
                .header(header::CONTENT_TYPE, "application/json")
                .send_body(query)
                .and_then(|mut response| response.json())
                .map_err(Error::from)
        }
        #[cfg(feature = "reqwestasyc")]
        {
            self.conn
                .client
                .post(format!("{}/_api/cursor", self.conn.host).as_str())
                .header("content-type", "application/json")
                .json(&query)
                .send()
                .and_then(|mut response| response.json())
                .map_err(Error::from)
        }
        #[cfg(all(feature = "reqwestactor", not(feature = "reqwestasyc")))]
        {
            futures::future::result(
            self.conn
                .client
                .post(format!("{}/_api/cursor", self.conn.host).as_str())
                .header("content-type", "application/json")
                .json(&query)
                .send()
                .and_then(|mut r| r.json())
            )
            // if let Ok(res) = res {
            //     if let Ok(j) = res.clone().json() {
            //         return futures::future::ok(j);
            //     }
            // }
            // return futures::future::err(res.unwrap_err());
        }
    }
}

impl<T> From<ArangoConnection> for ArangoConnectionInternal<T> {
    fn from(conn: ArangoConnection) -> Self {
        ArangoConnectionInternal {
            conn: conn.clone(),
            phantom: PhantomData::<T>,
        }
    }
}

// // TODO: give this a better name
// pub fn _get_from_collection(
//     // collection: &Collection,
//     coll_name: &str,
//     client: &Client,
//     host: &str,
// ) -> impl Future<Item = serde_json::Value, Error = Error> {
//     #[cfg(feature = "actixclient")]
//     {
//         client
//             .get(format!("{}/_api/collection/{}", host, coll_name))
//             .send()
//             .and_then(|mut response| response.json())
//             .map_err(Error::from)
//     }
//     #[cfg(feature = "reqwestasyc")]
//     {
//         client
//             .get(format!("{}/_api/collection/{}", host, coll_name).as_str())
//             .send()
//             .and_then(|mut response| response.json())
//             .map_err(Error::from)
//     }
// }

// pub fn _db_query(
//     client: &Client,
//     host: &str,
//     query: ArangoQuery,
// ) -> impl Future<Item = serde_json::Value, Error = Error> {
//     #[cfg(feature = "actixclient")]
//     {
//         let req = client
//             .post(format!("{}/_api/cursor", host))
//             .header(header::CONTENT_TYPE, "application/json");

//         req.send_body(query)
//             .and_then(|mut response| response.json())
//             .map_err(Error::from)
//     }
//     #[cfg(feature = "reqwestasyc")]
//     {
//         client
//             .post(format!("{}/_api/cursor", host).as_str())
//             .header("content-type", "application/json")
//             .body(query)
//             .send()
//             .and_then(|mut response| response.json())
//             .map_err(Error::from)
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
