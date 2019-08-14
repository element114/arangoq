extern crate arangoq;
use arangoq::*;

use serde::{Deserialize, Serialize};
use serde_json::value::Value;
use std::marker::PhantomData;
use std::collections::BTreeMap;
use maplit::*;
use futures::{lazy};

#[derive(Serialize, Deserialize, ArangoBuilder)]
pub struct TestUser {
    name: String,
}

#[test]
fn test_arango_connection_actix() {
    use actix_web::client::Client;
    use actix_web::test;
    use actix::*;

    // std::env::set_var("RUST_BACKTRACE", "1");
    let host = "http://localhost:8529";
    let client = Client::default();
    let conn = ArangoConnection {
        host,
        client,
        phantom: PhantomData::<TestUser>,
    };
    let collection_name = "TestUsers";
    let query = TestUser::query_builder(collection_name)
        .read()
        .filter()
        .name_eq(&"John Lennon".to_owned())
        .build();
    // Arbiter::new().exec_fn(conn.execute_query(query));
    // assert!(res.is_ok());
}

#[ignore]
#[test]
fn test_db_get_from_collection() {
    use actix_web::client::Client;
    use actix_web::test;
    let coll = "test_coll";
    let res = test::block_fn(move || {
        _get_from_collection(&coll, &Client::default(), "http://localhost:8529")
    });
    let resp = test::block_on(res).unwrap();
    // let resp_json: serde_json::Value = test::block_on(resp.json()).unwrap();

    assert!(resp["result"][1]["globallyUniqueId"].eq("_statistics"));
    let _coll_resp: ArangoResponse<serde_json::Value> = serde_json::from_value(resp).unwrap();
}
