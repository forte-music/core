#![feature(plugin)]
#![plugin(rocket_codegen)]

#[macro_use] extern crate juniper;
extern crate juniper_rocket;
extern crate rocket;
extern crate redis;

mod schema;
mod db;

use rocket::response::content;
use rocket::State;
use schema::resolvers::{Query, Mutation};
use schema::Schema;

#[get("/")]
fn graphql() -> content::Html<String> {
    juniper_rocket::graphiql_source("/graphql")
}

#[get("/graphql?<request>")]
fn get_graphql_handler(
    context: db::Connection,
    request: juniper_rocket::GraphQLRequest,
    schema: State<Schema>
) -> juniper_rocket::GraphQLResponse {
    request.execute(&schema, &context)
}

#[post("/graphql", data = "<request>")]
fn post_graphql_handler(
    context: db::Connection,
    request: juniper_rocket::GraphQLRequest,
    schema: State<Schema>
) -> juniper_rocket::GraphQLResponse {
    request.execute(&schema, &context)
}

fn main() {
    rocket::ignite()
        .manage(db::init_pool())
        .manage(Schema::new(Query {}, Mutation {}))
        .mount("/", routes![graphql, get_graphql_handler, post_graphql_handler])
        .launch();
}
