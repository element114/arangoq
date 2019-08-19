extern crate arangoq;
use arangoq::*;

use serde::{Deserialize, Serialize};
use serde_json::value::Value;
// use std::marker::PhantomData;
use maplit::*;
use std::collections::BTreeMap;

#[derive(Serialize, Deserialize, ArangoBuilder, Debug)]
pub struct TestUser {
    name: String,
}

#[cfg(feature = "actixclient")]
#[ignore]
#[test]
fn test_arango_connection_actix() {
    // use actix::prelude::*;
    use actix_web::client::Client;
    use actix_web::test;

    std::env::set_var("RUST_BACKTRACE", "1");
    let aconn = ArangoConnection::new("http://127.0.0.1:8529".to_owned(), Client::default());
    let collection_name = "Users";
    let query = TestUser::query_builder(collection_name).read().build();
    // let sys = System::new("db_example");
    // let res = Arbiter::spawn(
    //     futures::future::ok(query.exec(&aconn)).block_on()
    // );
    let res: Result<ArangoResponse<TestUser>, actix_web::Error> = test::block_on(query.exec(&aconn));
    println!("{:#?}", res);
    // sys.run();
}

#[cfg(feature = "actixclient")]
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
