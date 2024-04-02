use super::request_get;
use crate::error::Error;
use crate::types::*;

/// Get all teams
pub async fn all() -> Result<TeamListInfo, Error> {
    request_get::<TeamListInfo>(format!("/teams")).await
}

/// Get teams filtered by id
pub async fn by_id(id: i32) -> Result<TeamListInfo, Error> {
    request_get::<TeamListInfo>(format!("/teams/team/{}", id)).await
}