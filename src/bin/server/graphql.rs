use crate::server::transcoder::Transcoder;
use actix::prelude::*;
use actix_web::error;
use actix_web::AsyncResponder;
use actix_web::FutureResponse;
use actix_web::HttpRequest;
use actix_web::HttpResponse;
use actix_web::Json;
use actix_web::State;
use forte_core::context;
use forte_core::context::GraphQLContext;
use forte_core::models::Schema;
use futures::Future;
use juniper::graphiql::graphiql_source;
use juniper::http::GraphQLRequest;
use std::sync::Arc;

mod errors {
    error_chain! {
        foreign_links {
            R2d2(::r2d2::Error);
            SerdeJson(::serde_json::Error);
        }
    }
}

pub struct AppState {
    pub executor: Addr<GraphQLExecutor>,
    pub connection_pool: context::Pool,
    pub transcoder: Addr<Transcoder>,
}

impl AppState {
    pub fn new(
        executor: Addr<GraphQLExecutor>,
        transcoder: Addr<Transcoder>,
        connection_pool: context::Pool,
    ) -> AppState {
        AppState {
            executor,
            transcoder,
            connection_pool,
        }
    }

    pub fn build_context(&self) -> Result<GraphQLContext, ::r2d2::Error> {
        let connection = self.connection_pool.get()?;
        Ok(GraphQLContext::new(connection))
    }
}

struct ResolveMessage {
    request: GraphQLRequest,
    context: GraphQLContext,
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
        let response = request.request.execute(&self.schema, &request.context);
        let text = serde_json::to_string(&response)?;

        Ok((response.is_ok(), text))
    }
}

pub fn graphql(params: (State<AppState>, Json<GraphQLRequest>)) -> FutureResponse<HttpResponse> {
    let state = params.0;
    let request = params.1;

    let context_future = futures::done(
        state
            .build_context()
            .map_err(error::ErrorInternalServerError),
    );

    context_future
        .and_then(move |context| {
            state
                .executor
                .send(ResolveMessage {
                    request: request.0,
                    context,
                })
                .from_err()
        })
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

pub fn graphiql<S: 'static>(_req: &HttpRequest<S>) -> HttpResponse {
    let html = graphiql_source("/graphql");

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}
