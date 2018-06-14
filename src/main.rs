extern crate actix;
extern crate actix_web;
extern crate chrono;
#[macro_use]
extern crate diesel;
extern crate r2d2_diesel;
extern crate dotenv;
extern crate env_logger;
extern crate futures;
#[macro_use]
extern crate juniper;
extern crate num_cpus;
extern crate r2d2;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate uuid;

use actix::prelude::*;
use actix_web::{http, middleware, server, App, AsyncResponder, FutureResponse, HttpResponse, Path,
                State};

use futures::future::Future;

mod database;
mod graphql;

use database::connection::{get_db_connection_pool, CreateUser, DBPool, DbExecutor};
use graphql::executor::GraphQLExecutor;

pub struct AppState {
    db: Addr<Syn, DbExecutor>,
    executor: Addr<Syn, GraphQLExecutor>,
}

/// Async request handler
fn index(name: Path<String>, state: State<AppState>) -> FutureResponse<HttpResponse> {
    // send async `CreateUser` message to a `DbExecutor`
    state
        .db
        .send(CreateUser {
            name: name.into_inner(),
        })
        .from_err()
        .and_then(|res| match res {
            Ok(user) => Ok(HttpResponse::Ok().json(user)),
            Err(_) => Ok(HttpResponse::InternalServerError().into()),
        })
        .responder()
}

fn main() {
    ::std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    dotenv::dotenv().ok();
    let sys = actix::System::new("swipe-server");
    let capacity = (num_cpus::get() / 2) as usize;

    server::new(move || {
        App::with_state(
            AppState{
                db: database::connection::get_db_address(capacity),
                executor: graphql::executor::create_executor(capacity, get_db_connection_pool())
        })
        // enable logger
        .middleware(middleware::Logger::default())
            .resource("/graphql", |r| r.method(http::Method::POST).h(graphql::executor::graphql))
            .resource("/graphiql", |r| r.method(http::Method::GET).h(graphql::executor::graphiql))
            .resource("/get/{name}", |r| r.method(http::Method::GET).with2(index))
    }).bind("localhost:8000")
        .expect("Unable to bind to port")
        .shutdown_timeout(2)
        .start();

    println!("Started http server: localhost:8000");
    let _ = sys.run();
}
