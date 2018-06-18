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
extern crate colored;

use actix::prelude::*;
use actix_web::{http, middleware, server, App, HttpResponse, HttpRequest};

use colored::*;

mod database;
mod graphql;

use database::connection::{get_db_connection_pool, DBPool, DbExecutor};
use graphql::executor::GraphQLExecutor;

// App state for r2d2 pool and graphql executor
#[allow(dead_code)]
pub struct AppState {
    db: Addr<Syn, DbExecutor>,
    executor: Addr<Syn, GraphQLExecutor>,
}

/// Simple API index
fn index(_req: HttpRequest<AppState>) -> HttpResponse {
    HttpResponse::Ok().json("Swipe app graphql api")
}

// Main: activate logs, setup 3 routes and run
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
        .middleware(middleware::Logger::new("\nRemote '%{User-Agent}i' (ip: %a) request %b bytes in %D ms"))
        .resource("/graphql", |r| r.method(http::Method::POST).h(graphql::executor::graphql))
        .resource("/graphiql", |r| r.method(http::Method::GET).h(graphql::executor::graphiql))
        .resource("/", |r| r.method(http::Method::GET).with(index))
    }).bind("localhost:8000")
        .expect("Unable to bind to port")
        .shutdown_timeout(2)
        .start();

    println!("\n {} {}\n", " ".blink().green(), "Started http server: localhost:8000".bold().yellow());
    let _ = sys.run();
    println!("\n {} {}\n", "".blink().red(), "Server stopped".bold().yellow())
}
