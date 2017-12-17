pub mod model;
mod binding;
mod resolvers;

use juniper;

pub type Schema = juniper::RootNode<'static, model::Query, model::Mutation>;
