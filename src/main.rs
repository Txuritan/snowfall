mod backend;
mod config;
mod database;
mod schema;

use std::sync::Arc;

use futures::future::Future;

use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

use actix_web::{
    actix::*, http, middleware, server, App, AsyncResponder, Error, FutureResponse, HttpResponse,
    Json, State,
};
use juniper::http::GraphQLRequest;

use crate::{
    config::Config,
    database::Database,
    schema::{create_schema, Schema},
};

lazy_static! {
    pub static ref CONFIG: Config = Config::load();
}

struct SnowfallState {
    graphql: Addr<GraphQLExecutor>,
}

#[derive(Serialize, Deserialize)]
pub struct GraphQLData(pub GraphQLRequest);

impl Message for GraphQLData {
    type Result = Result<String, Error>;
}

pub struct GraphQLExecutor {
    context: Database,
    schema: Arc<Schema>,
}

impl GraphQLExecutor {
    pub fn new(context: Database, schema: Arc<Schema>) -> GraphQLExecutor {
        GraphQLExecutor { context, schema }
    }
}

impl Actor for GraphQLExecutor {
    type Context = SyncContext<Self>;
}

impl Handler<GraphQLData> for GraphQLExecutor {
    type Result = Result<String, Error>;

    fn handle(&mut self, msg: GraphQLData, _: &mut Self::Context) -> Self::Result {
        let res = msg.0.execute(&self.schema, &self.context);
        let res_text = serde_json::to_string(&res)?;

        Ok(res_text)
    }
}

fn main() {
    ::std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    let sys = System::new("snowfall");

    let database = Database::new();

    let schema = Arc::new(create_schema());
    let schema_addr = SyncArbiter::start(3, move || {
        GraphQLExecutor::new(database.clone(), schema.clone())
    });

    let addr = format!("{}:{}", CONFIG.server.host, CONFIG.server.port);

    server::new(move || {
        App::with_state(SnowfallState {
            graphql: schema_addr.clone(),
        })
        .middleware(middleware::Logger::default())
        .resource("/graphql", |r| r.method(http::Method::POST).with(graphql))
    })
    .bind(addr)
    .unwrap()
    .start();

    let _ = sys.run();
}

fn graphql(
    (state, data): (State<SnowfallState>, Json<GraphQLData>),
) -> FutureResponse<HttpResponse> {
    state
        .graphql
        .send(data.0)
        .from_err()
        .and_then(|res| match res {
            Ok(data) => Ok(HttpResponse::Ok()
                .content_type("application/json")
                .body(data)),
            Err(_) => Ok(HttpResponse::InternalServerError().into()),
        })
        .responder()
}
