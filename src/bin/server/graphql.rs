use juniper::graphiql::graphiql_source;
use juniper::http::GraphQLRequest;

use forte_core::context;
use forte_core::context::GraphQLContext;
use forte_core::models::Schema;

use actix::prelude::*;

use actix_web::AsyncResponder;
use actix_web::FutureResponse;
use actix_web::HttpRequest;
use actix_web::HttpResponse;
use actix_web::Json;
use actix_web::State;

use serde_json;
use std::sync::Arc;

mod errors {
    error_chain! {
        foreign_links {
            R2d2(::r2d2::Error);
            SerdeJson(::serde_json::Error);
        }
    }
}

use futures::Future;

pub struct AppState {
    pub executor: Addr<Syn, GraphQLExecutor>,
    pub connection_pool: context::Pool,
}

impl AppState {
    pub fn new(executor: Addr<Syn, GraphQLExecutor>, connection_pool: context::Pool) -> AppState {
        AppState {
            executor,
            connection_pool,
        }
    }
}

struct ResolveMessage {
    request: GraphQLRequest,
    connection_pool: context::Pool,
}

impl Message for ResolveMessage {
    type Result = Result<(bool, String), errors::Error>;
}

pub struct GraphQLExecutor {
    schema: Arc<Schema>,
}

impl GraphQLExecutor {
    pub fn new(schema: Arc<Schema>) -> GraphQLExecutor {
        GraphQLExecutor { schema }
    }
}

impl Actor for GraphQLExecutor {
    type Context = SyncContext<Self>;
}

impl Handler<ResolveMessage> for GraphQLExecutor {
    type Result = Result<(bool, String), errors::Error>;

    fn handle(&mut self, request: ResolveMessage, _ctx: &mut Self::Context) -> Self::Result {
        let context = GraphQLContext::new(request.connection_pool.get()?);
        let response = request.request.execute(&self.schema, &context);
        let text = serde_json::to_string(&response)?;

        Ok((response.is_ok(), text))
    }
}

pub fn graphql(
    state: State<AppState>,
    request: Json<GraphQLRequest>,
) -> FutureResponse<HttpResponse> {
    state
        .executor
        .send(ResolveMessage {
            request: request.0,
            connection_pool: state.connection_pool.clone(),
        })
        .from_err()
        .and_then(|resp| match resp {
            Ok((true, body)) => Ok(HttpResponse::Ok()
                .content_type("application/json")
                .body(body)),
            Ok((false, body)) => Ok(HttpResponse::BadRequest()
                .content_type("application/json")
                .body(body)),
            Err(..) => Ok(HttpResponse::InternalServerError().into()),
        })
        .responder()
}

pub fn graphiql<S>(_req: HttpRequest<S>) -> HttpResponse {
    let html = graphiql_source("/graphql");

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
        .into()
}
