extern crate uuid;

mod graphql;

use forte_core::context;
use forte_core::models::create_schema;
use forte_core::models::song::Song;

use server::graphql::{graphiql, graphql, AppState, GraphQLExecutor};

use actix::System;
use actix::prelude::*;
use actix_web::App;
use actix_web::http;
use actix_web::server;

use std::sync::Arc;

pub fn serve(pool: context::Pool, host: &str) {
    let sys = System::new("forte");

    let schema = Arc::new(create_schema());
    let addr = SyncArbiter::start(3, move || GraphQLExecutor::new(schema.clone()));

    server::new(move || {
        App::with_state(AppState::new(addr.clone(), pool.clone()))
            .resource("/graphql", |r| r.method(http::Method::POST).with2(graphql))
            .resource("/", |r| r.method(http::Method::GET).h(graphiql))
    }).bind(host)
        .unwrap()
        .start();

    println!("Started Server on {}", host);

    let _ = sys.run();
}
