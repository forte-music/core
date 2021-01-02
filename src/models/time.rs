use chrono::NaiveDateTime;
use juniper::GraphQLScalarValue;
use std::ops::Deref;

#[derive(GraphQLScalarValue)]
#[graphql(transparent, name = "Time")]
pub struct TimeWrapper(NaiveDateTime);

impl Deref for TimeWrapper {
    type Target = NaiveDateTime;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<NaiveDateTime> for TimeWrapper {
    fn from(t: NaiveDateTime) -> Self {
        TimeWrapper(t)
    }
}
