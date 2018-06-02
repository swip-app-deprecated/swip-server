use juniper::Context as GraphQlContext;
use database::*;

pub struct Context {
    pub database: Connection
}

impl GraphQlContext for Context {}
