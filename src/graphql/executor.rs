use actix::prelude::*;
use futures::future::Future;
use serde_json;
use juniper::http::graphiql::graphiql_source;
use juniper::http::GraphQLRequest;
use std;
use actix_web::{http, AsyncResponder, Error, HttpMessage, HttpRequest, HttpResponse};

use ::{AppState, DBPool};
use graphql::types::{create_schema, Schema};

#[derive(Serialize, Deserialize)]
pub struct GraphQLData(GraphQLRequest);

// Setup graphql result type
impl Message for GraphQLData {
    type Result = Result<String, Error>;
}

// Setup graphql executor with db pool and schema
pub struct GraphQLExecutor {
    pub schema: std::sync::Arc<Schema>,
    pub db_pool: DBPool,
}

// Setup Context for executor
impl Actor for GraphQLExecutor {
    type Context = SyncContext<Self>;
}

// Setup graphql query handler
impl Handler<GraphQLData> for GraphQLExecutor {
    type Result = Result<String, Error>;

    fn handle(&mut self, msg: GraphQLData, _: &mut Self::Context) -> Self::Result {
        let res = msg.0.execute(&self.schema, &self);
        let res_text = serde_json::to_string(&res)?;
        Ok(res_text)
    }
}

// Graphiql route
pub fn graphiql(_req: HttpRequest<AppState>) -> Result<HttpResponse, Error> {
    let html = graphiql_source("/graphql");
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html))
}

// Graphql endpoints route
pub fn graphql(req: HttpRequest<AppState>) -> Box<Future<Item = HttpResponse, Error = Error>> {
    let executor = req.state().executor.clone();
    req.json()
        .from_err()
        .and_then(move |val: GraphQLData| {
            executor.send(val).from_err().and_then(|res| match res {
                Ok(user) => Ok(HttpResponse::Ok()
                    .header(http::header::CONTENT_TYPE, "application/json")
                    .body(user)),
                Err(_) => Ok(HttpResponse::InternalServerError().into()),
            })
        })
        .responder()
}

pub fn create_executor(capacity: usize, pool: DBPool) -> Addr<Syn, GraphQLExecutor> {
    SyncArbiter::start(capacity, move || GraphQLExecutor {
        schema: std::sync::Arc::new(create_schema()),
        db_pool: pool.clone(),
    })
}
