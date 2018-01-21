use rocket::{self, State};
use rocket::response::content;
use juniper_rocket;
use schema::model::{Mutation, Query};
use schema::Schema;
use database;

pub fn start() {
    rocket::ignite()
        .manage(database::init_pool())
        .manage(Schema::new(Query {}, Mutation {}))
        .mount(
            "/",
            routes![graphql, get_graphql_handler, post_graphql_handler],
        )
        .launch();
}

#[get("/")]
fn graphql() -> content::Html<String> {
    juniper_rocket::graphiql_source("/graphql")
}

#[get("/graphql?<request>")]
fn get_graphql_handler(
    context: database::Connection,
    request: juniper_rocket::GraphQLRequest,
    schema: State<Schema>,
) -> juniper_rocket::GraphQLResponse {
    request.execute(&schema, &context)
}

#[post("/graphql", data = "<request>")]
fn post_graphql_handler(
    context: database::Connection,
    request: juniper_rocket::GraphQLRequest,
    schema: State<Schema>,
) -> juniper_rocket::GraphQLResponse {
    request.execute(&schema, &context)
}
