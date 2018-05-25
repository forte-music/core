#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserStatsSource {
    pub last_played: Option<i64>,
}
