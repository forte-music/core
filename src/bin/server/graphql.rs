use crate::server::transcoder::Transcoder;
use actix_web::web::{Data, Payload};
use actix_web::{error, get, post, HttpRequest, HttpResponse};
use forte_core::context;
use forte_core::context::GraphQLContext;
use forte_core::models::Schema;
use juniper_actix::{graphiql_handler, graphql_handler};

pub struct AppState {
    pub schema: Schema,
    pub connection_pool: context::Pool,
    pub transcoder: Transcoder,
}

impl AppState {
    pub fn build_context(&self) -> Result<GraphQLContext, r2d2::Error> {
        let connection = self.connection_pool.get()?;
        Ok(GraphQLContext::new(connection))
    }
}

#[post("/graphql")]
pub async fn graphql(
    request: HttpRequest,
    payload: Payload,
    state: Data<AppState>,
) -> actix_web::Result<HttpResponse> {
    let context = state
        .build_context()
        .map_err(error::ErrorInternalServerError)?;
    graphql_handler(&state.schema, &context, request, payload).await
}

#[get("/graphiql")]
pub async fn graphiql() -> actix_web::Result<HttpResponse> {
    graphiql_handler("/graphql", None).await
}
