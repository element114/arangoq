use actix::{Actor, System};
use actix_rt::spawn;
use arangoq::*;
use arangoq::*;
use futures::Future;
use lazy_static::*;
use log::debug;
use mockito;
use mockito::mock;
use proptest::prelude::*;
use proptest_derive::Arbitrary;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;

lazy_static! {
    static ref DATABASE: ArangoConnection = {
        std::env::set_var("RUST_LOG", "debug,hyper=info,tokio_reactor=info");
        let _res = env_logger::try_init();
        //set db password
        std::env::set_var("ARANGO_USER_NAME", "arangoq");
        std::env::set_var("ARANGO_PASSWORD", "arangoq");
        let db_host = "http://localhost:8529/".to_owned();
        let db_name = "evt_test".to_owned();
        // create new connection to local db
        ArangoConnection::new(
            db_host,
            db_name,
            reqwest::r#async::Client::new(),
        )
    };
}

/// These tests verify that generated query objects and responses work well
/// with a real arangodb instance.
/// Create a db called arangoq with user arangoq, and password arangoq
/// for these tests to work.

#[ignore]
#[test]
fn live_setup() {
    // setup logs
    std::env::set_var("RUST_LOG", "debug,hyper=info,tokio_reactor=info");
    let _res = env_logger::try_init();
    //set db password
    std::env::set_var("ARANGO_USER_NAME", "arangoq");
    std::env::set_var("ARANGO_PASSWORD", "arangoq");
    // create new connection to local db
    // let connection = ArangoConnection::new(
    //     "http://localhost:8529/_db/arangoq/_api/cursor".to_owned(),
    //     reqwest::r#async::Client::new(),
    // );

    create_collection("testdocs", arangoq::CollectionType::Document);
}

proptest! {
#![proptest_config(ProptestConfig::with_cases(1))]
#[ignore]
#[test]
fn test_live_queries(test_data: TestData) {
    // fn test_live_queries() {
    // let test_data = TestData::default();
    let _res = System::run(move || {
        // start actor
        let addr = ArangoActorAsync { connection: DATABASE.clone() }.start();

        // add test data with insert
        debug!("{:?}", test_data);

        // create an insert query
        let coll = Collection::new("testdocs", CollectionType::Document);
        let query = coll.insert(&test_data);
        let dbq = DbQuery(query, std::marker::PhantomData::<TestData>);
        // send message and get future for result
        let res = addr.send(dbq);
        // handle() returns tokio handle
        spawn(
            res.map(|res| {
                let ar = res.unwrap();
                assert!(!ar.error);
                let inserted_data = ar.result.first().unwrap();
                debug!("{:?}", inserted_data);
                assert!(!inserted_data._key.is_empty());
                // Last one alive, lock the door!
                // System::current().stop();
            })
            .map_err(|_| ()),
        );

        // create a replace query
        let coll = Collection::new("testdocs", CollectionType::Document);
        let query = coll.insert(&test_data);
        let dbq = DbQuery(query, std::marker::PhantomData::<TestData>);
        // send message and get future for result
        let res = addr.send(dbq);
        // handle() returns tokio handle
        spawn(
            res.map(|res| {
                let ar = res.unwrap();
                assert!(!ar.error);
                let inserted_data = ar.result.first().unwrap();
                debug!("{:?}", inserted_data);
                assert!(!inserted_data._key.is_empty());
                System::current().stop();
            })
            .map_err(|_| ()),
        );
    });
}
}

#[derive(Serialize, Deserialize, Debug, Default, Arbitrary, ArangoBuilder)]
pub struct TestData {
    #[proptest(value = "String::new()")]
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub(crate) _key: String,
    #[proptest(strategy = "any::<String>()")]
    name: String,
    // Error("i128 is not supported", line: 0, column: 0)
    // #[proptest(strategy = "any::<u128>()")]
    // u128data: u128,
    #[proptest(strategy = "any::<u64>()")]
    u64data: u64,
    #[proptest(strategy = "any::<u32>()")]
    u32data: u32,
    #[proptest(strategy = "any::<u16>()")]
    u16data: u16,
    #[proptest(strategy = "any::<u8>()")]
    u8data: u8,
    // Error("i128 is not supported", line: 0, column: 0)
    // #[proptest(strategy = "any::<i128>()")]
    // i128data: i128,
    #[proptest(strategy = "any::<i16>()")]
    i16data: i16,
    // TODO: figure out how to tag these
    // #[proptest(strategy = "proptest::option::of::<String>(String::default())")]
    // an_option: Option<String>,
    // #[proptest(strategy = "any::<Vec<String>>()")]
    // a_vec: Vec<String>,
    // #[proptest(strategy = "any::<HashMap<String, String>>()")]
    // a_map: HashMap<String, String>,
}

fn create_collection(local_name: &str, collection_type: arangoq::CollectionType) {
    let coll_url = "http://localhost:8529/_db/arangoq/_api/collection";

    let data = json!({
        "name": local_name,
        "type": collection_type as u8
    });
    debug!("{}", data.to_string());
    let client = reqwest::Client::new();
    let res = client
        .post(coll_url)
        .header("accept", "application/json")
        .header("content-type", "application/json")
        .basic_auth(
            std::env::var("ARANGO_USER_NAME").unwrap_or_default(),
            std::env::var("ARANGO_PASSWORD").ok(),
        )
        .json(&data)
        .send();
    debug!("{:#?}", res);
}
