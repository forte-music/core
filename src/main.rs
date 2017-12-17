#![feature(plugin)]
#![plugin(rocket_codegen)]

#[macro_use] extern crate juniper;
extern crate juniper_rocket;
extern crate rocket;
extern crate redis;

mod schema;
mod database;

use rocket::response::content;
use rocket::State;
use schema::model::{Query, Mutation};
use schema::Schema;

#[get("/")]
fn graphql() -> content::Html<String> {
    juniper_rocket::graphiql_source("/graphql")
}

#[get("/graphql?<request>")]
fn get_graphql_handler(
    context: database::Connection,
    request: juniper_rocket::GraphQLRequest,
    schema: State<Schema>
) -> juniper_rocket::GraphQLResponse {
    request.execute(&schema, &context)
}

#[post("/graphql", data = "<request>")]
fn post_graphql_handler(
    context: database::Connection,
    request: juniper_rocket::GraphQLRequest,
    schema: State<Schema>
) -> juniper_rocket::GraphQLResponse {
    request.execute(&schema, &context)
}

fn main() {
    rocket::ignite()
        .manage(database::init_pool())
        .manage(Schema::new(Query {}, Mutation {}))
        .mount("/", routes![graphql, get_graphql_handler, post_graphql_handler])
        .launch();
}
