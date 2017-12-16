pub mod model;
pub mod resolvers;

use juniper;

pub type Schema = juniper::RootNode<'static, resolvers::Query, resolvers::Mutation>;
