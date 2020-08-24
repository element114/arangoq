#[cfg(feature = "actors")]
use actix::{Actor, System};
#[cfg(feature = "actors")]
use actix_rt::{spawn, test};
use arangoq::*;
#[cfg(feature = "actors")]
use futures::future::FutureExt;
use lazy_static::*;
use proptest::prelude::*;
use proptest_derive::Arbitrary;
use serde::{Deserialize, Serialize};
#[macro_use] extern crate log;

lazy_static! {
    static ref DATABASE: ArangoConnection = {
        std::env::set_var("RUST_LOG", "debug,hyper=info,tokio_reactor=info");
        let _res = env_logger::try_init();
        //set db password
        std::env::set_var("ARANGO_USER_NAME", "test_dev");
        std::env::set_var("ARANGO_PASSWORD", "test_dev_pw");
        let db_host = "http://localhost:8529/".to_owned();
        let db_name = "test_dev".to_owned();
        // create new connection to local db
        ArangoConnection::new(
            db_host,
            db_name,
            reqwest::Client::new(),
        )
    };
}

#[cfg(feature = "actors")]
proptest! {
#![proptest_config(ProptestConfig::with_cases(1))]
// #[ignore]
#[test]
/// These tests verify that generated query objects and responses work well
/// with a real arangodb instance.
/// Create a db called arangoq with user arangoq, and password arangoq
/// for these tests to work.
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
                let ar = res.unwrap().unwrap();
                assert!(!ar.error);
                let inserted_data = ar.result.first().unwrap();
                debug!("{:?}", inserted_data);
                assert!(!inserted_data._key.is_empty());
                // Last one alive, lock the door!
                // System::current().stop();
            }),
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
                let ar = res.unwrap().unwrap();
                assert!(!ar.error);
                let inserted_data = ar.result.first().unwrap();
                debug!("{:?}", inserted_data);
                assert!(!inserted_data._key.is_empty());
                System::current().stop();
            }),
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
