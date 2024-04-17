use super::request_get;
use crate::error::Error;
use crate::types::*;

/// Get all teams
pub async fn all() -> Result<GameListInfo, Error> {
    request_get::<GameListInfo>(format!("/games")).await
}

/// Get teams filtered by id
pub async fn by_round_id(id: i32) -> Result<GameListInfo, Error> {
    request_get::<GameListInfo>(format!("/games/round/{}", id)).await
}

/// Get teams filtered by id
pub async fn by_field_id(id: i32) -> Result<GameListInfo, Error> {
    request_get::<GameListInfo>(format!("/games/field/{}", id)).await
}

/// Get teams filtered by id
pub async fn by_is_played(is_played: bool) -> Result<GameListInfo, Error> {
    request_get::<GameListInfo>(format!("/games/played/{}", is_played)).await
}

/// Get teams filtered by id
pub async fn by_round_id_field_id(round_id: i32, field_id: i32) -> Result<GameListInfo, Error> {
    request_get::<GameListInfo>(format!("/games/field/{}/round/{}", field_id, round_id)).await
}

//// Make new games when the group fase is finished
pub async fn get_winners_group_fase() -> Result<ScoreListInfo, Error> {
    request_get::<ScoreListInfo>(format!("/winners/groupfase")).await
}

//// Make new games when the group fase is finished
pub async fn get_winners_by_round(round_id: i32) -> Result<ScoreListInfo, Error> {
    request_get::<ScoreListInfo>(format!("/winners/round/{}", round_id)).await
}

//// Make new games when the group fase is finished
pub async fn get_losers_by_round(round_id: i32) -> Result<ScoreListInfo, Error> {
    request_get::<ScoreListInfo>(format!("/losers/round/{}", round_id)).await
}