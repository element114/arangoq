use actix::{Actor, System};
use actix_rt::spawn;
use arangoq::*;
use futures::Future;
use mockito;
use mockito::mock;
use serde::{Deserialize, Serialize};

#[test]
fn test_async_tooling() {
    std::env::set_var("RUST_LOG", "debug,hyper=info");
    let _res = env_logger::try_init();
    std::env::set_var("ARANGO_USER_NAME", "evt_write");
    std::env::set_var("ARANGO_PASSWORD", "notarealpw");

    let mock_resp = r#"{"result":[{"_key":"537130","id":8,"name":"NU","parent_id":"organizers/4242"}],"hasMore":false,"cached":false,"extra":{"stats":{"writesExecuted":0,"writesIgnored":0,"scannedFull":1,"scannedIndex":0,"filtered":0,"httpRequests":0,"executionTime":0.0012001991271972656,"peakMemoryUsage":8007},"warnings":[]},"error":false,"code":201}"#;
    let _m = mock("POST", "/")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(mock_resp)
        .expect(1)
        .create();

    // start system, it is a required step
    let _res = System::run(|| {
        // create new connection
        let connection =
            ArangoConnection::new(mockito::server_url(), reqwest::r#async::Client::new());
        // start new actor
        let addr = ArangoActorAsync { connection }.start();

        // create query
        let query = TestUser::query_builder("TestUsers").read().build();
        let dbq = DbQuery(query, std::marker::PhantomData::<TestUser>);

        // send message and get future for result
        let res = addr.send(dbq);

        // handle() returns tokio handle
        spawn(
            res.map(|res| {
                println!("RESULT: {:#?}", res);
                assert_eq!("NU", res.unwrap().result.first().unwrap().name);
                // stop system and exit
                System::current().stop();
            })
            .map_err(|_| ()),
        );
    });
}

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
    let res: Result<ArangoResponse<TestUser>, actix_web::Error> =
        test::block_on(query.exec(&aconn));
    debug!("{:#?}", res);
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
