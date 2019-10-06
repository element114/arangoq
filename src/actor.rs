use crate::{ArangoConnection, ArangoConnectionInternal, ArangoQuery, ArangoResponse};
use actix::*;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::env;

/// This is a sync actor to be able to use reqwest sync client in any actix based project.
/// Usage:
///
/// use arangoq::{ArangoActor, ArangoConnection};
/// use reqwest::Client;
/// use actix::prelude::*;
/// use actix_web::{App, HttpServer};
/// let sys = actix::System::new("auth-proxy");
/// let connection = ArangoConnection {
///    host: std::sync::Arc::new(format!("{}/_db/{}/_api/cursor", db_conn, db_name)),
///    client: std::sync::Arc::new(Client::new()),
/// };
/// let addr = SyncArbiter::start(2, move || ArangoActor {connection: connection.clone()});
/// HttpServer::new(move || {
///     App::new()
///         .data(addr.clone())
/// });
/// sys.run();
///
pub struct ArangoActor {
    pub connection: ArangoConnection,
}

impl Actor for ArangoActor {
    type Context = actix::SyncContext<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        println!("I am alive!");
    }
}

pub struct DbQuery<T>(pub ArangoQuery, pub std::marker::PhantomData<T>);

impl<T: 'static> Message for DbQuery<T> {
    type Result = Result<ArangoResponse<T>, reqwest::Error>;
}

impl<T: 'static + Serialize + DeserializeOwned + std::fmt::Debug> Handler<DbQuery<T>>
    for ArangoActor
{
    type Result = Result<ArangoResponse<T>, reqwest::Error>;

    fn handle(&mut self, msg: DbQuery<T>, _ctx: &mut SyncContext<Self>) -> Self::Result {
        let query = msg.0;
        let dbc = &self.connection;
        // query.exec(&dbc)
        // .map_err(|_| "Error occured during db request".to_owned())
        let conn: ArangoConnectionInternal<T> = dbc.clone().into();
        conn.conn
            .client
            .post(format!("{}", conn.conn.host).as_str())
            .header("content-type", "application/json")
            .json(&query)
            .basic_auth(
                env::var("ARANGO_USER_NAME").unwrap_or_default(),
                env::var("ARANGO_PASSWORD").ok(),
            )
            .send()
            .and_then(|mut r| {
                // let res: serde_json::Value = r.json().unwrap();
                // println!("{}", res);
                r.json()
            })
        // .map_err(Error::from)
        // .map_err(|e| e.to_string())
    }
}
