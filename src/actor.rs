use crate::{ArangoConnection, ArangoQuery, ArangoResponse};
use actix::prelude::*;
use log::debug;
use serde::de::DeserializeOwned;
use serde::Serialize;

pub struct DbQuery<T>(pub ArangoQuery, pub std::marker::PhantomData<T>);

impl<T: 'static> Message for DbQuery<T> {
    type Result = Result<ArangoResponse<T>, reqwest::Error>;
}

/// This is an actix async actor using reqwest async client.
/// Usage:
///
/// ```ignore
/// use arangoq::{ArangoActor, ArangoConnection};
/// use reqwest::r#async::Client;
/// use actix::prelude::*;
/// use actix_web::{App, HttpServer};
/// let sys = actix::System::new("auth-proxy");
/// let connection = ArangoConnection {
///    host: std::sync::Arc::new(format!("{}/_db/{}/_api/cursor", db_conn, db_name)),
///    client: std::sync::Arc::new(Client::new()),
/// };
/// let addr = ArangoActor {connection: connection.clone()}.start();
/// HttpServer::new(move || {
///     App::new()
///         .data(addr.clone())
/// });
/// sys.run();
/// ```
pub struct ArangoActorAsync {
    pub connection: ArangoConnection,
}

impl Actor for ArangoActorAsync {
    type Context = actix::Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        debug!("ArangoActorAsync alive.");
    }
}

impl<T: 'static + Serialize + DeserializeOwned + std::fmt::Debug + Send> Handler<DbQuery<T>>
    for ArangoActorAsync
{
    type Result = ResponseFuture<Result<ArangoResponse<T>, reqwest::Error>>;

    fn handle(&mut self, msg: DbQuery<T>, _ctx: &mut Context<Self>) -> Self::Result {
        let query = msg.0;
        let dbc = &self.connection;
        Box::pin(query.try_exec::<T>(dbc))
    }
}

impl Message for ArangoQuery {
    type Result = Result<ArangoResponse<serde_json::Value>, reqwest::Error>;
}
impl Handler<ArangoQuery> for ArangoActorAsync {
    type Result = ResponseFuture<Result<ArangoResponse<serde_json::Value>, reqwest::Error>>;

    fn handle(&mut self, query: ArangoQuery, _ctx: &mut Context<Self>) -> Self::Result {
        let dbc = &self.connection;
        Box::pin(query.try_exec::<serde_json::Value>(dbc))
    }
}
