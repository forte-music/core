use juniper::ID;

pub struct SongUserStats {
    pub id: String,
    pub play_count: i32,
    pub liked: bool,
}

#[juniper::graphql_object]
impl SongUserStats {
    fn id(&self) -> ID {
        ID::from(self.id.to_owned())
    }

    fn play_count(&self) -> i32 {
        self.play_count
    }

    fn liked(&self) -> bool {
        self.liked
    }
}
