use crate::models::*;
use juniper::ID;

pub struct UserStats {
    pub id: String,
    pub last_played: Option<NaiveDateTime>,
}

#[juniper::graphql_object]
impl UserStats {
    fn id(&self) -> ID {
        ID::from(self.id.to_owned())
    }

    fn last_played(&self) -> Option<TimeWrapper> {
        self.last_played.map(TimeWrapper::from)
    }
}
