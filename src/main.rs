#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
#[macro_use] extern crate rocket_contrib;
extern crate serde;
#[macro_use] extern crate serde_json;
#[macro_use] extern crate serde_derive;
extern crate rustc_serialize;
#[macro_use] extern crate juniper;
#[macro_use] extern crate log;
extern crate log4rs;
#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_codegen;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate dotenv;
extern crate chrono;
extern crate uuid;

pub mod database;
pub mod graphql;

use dotenv::dotenv;
use rocket::http::Cookies;
use database::*;
use graphql::context::*;
use std::collections::BTreeMap;

fn main() {
    dotenv().ok();

    rocket::ignite()
        .manage(database::init_pool())
        .mount("/", routes![graphql])
        .launch();
}

#[post("/graphql", data = "<params>")]
fn graphql(
    _cookies: &Cookies,
    params: graphql::request::Request,
    database: Connection
) -> String {


    let query = &(params.query[..]);
    let operation_name_raw = match params.operation_name.clone() {
        Some(value) => value,
        _ => "".to_string()
    };
    let operation_name_raw = &operation_name_raw[..];
    let operation_name = if params.operation_name == None {
        None
    } else {
        Some(operation_name_raw)
    };
    let schema = &graphql::schema();
    let variables = &params.variables;
    let context = &Context {
        database: database
    };


    let result = juniper::execute(
        query,
        operation_name,
        schema,
        variables,
        context
    );

    match result {
        Ok((result, errors)) => {
            let mut map = BTreeMap::new();
            map.insert("data".to_owned(), json!(result));

            if !errors.is_empty() {
                map.insert("errors".to_owned(), json!(errors));
            }

            let json = json!(map);
            json.to_string()
        },
        Err(err) => {
            let mut map = BTreeMap::new();
            map.insert("errors".to_owned(), json!(err));
            let json = json!(map);
            json.to_string()
        }
    }
}
