use chrono::NaiveDateTime;
use juniper::Value;
use std::ops::Deref;

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

graphql_scalar!(TimeWrapper as "Time" {
    resolve(&self) -> Value {
        Value::int(self.timestamp() as i32)
    }

    from_input_value(v: &InputValue) -> Option<TimeWrapper> {
        None
    }

});
