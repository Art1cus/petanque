use super::{request_delete, request_get, request_post, request_put};
use crate::error::Error;
use crate::types::*;

/// Get all teams
pub async fn all() -> Result<GameListInfo, Error> {
    request_get::<GameListInfo>(format!("/gameteams")).await
}

/// Get teams filtered by id
pub async fn by_round_id(id: i32) -> Result<GameListInfo, Error> {
    request_get::<GameListInfo>(format!("/gameteams/round/{}", id)).await
}

/// Get teams filtered by id
pub async fn by_field_id(id: i32) -> Result<GameListInfo, Error> {
    request_get::<GameListInfo>(format!("/gameteams/field/{}", id)).await
}

/// Get teams filtered by id
pub async fn by_is_played(is_played: bool) -> Result<GameListInfo, Error> {
    request_get::<GameListInfo>(format!("/gameteams/played/{}", is_played)).await
}

/// Get teams filtered by id
pub async fn by_round_id_field_id(round_id: u32, field_id: i32) -> Result<GameListInfo, Error> {
    request_get::<GameListInfo>(format!("/gameteams/field/{}/round/{}", field_id, round_id)).await
}