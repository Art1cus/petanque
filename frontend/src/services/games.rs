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