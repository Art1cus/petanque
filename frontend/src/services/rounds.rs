use super::request_get;
use crate::error::Error;
use crate::types::*;

/// Get all teams
pub async fn all() -> Result<RoundListInfo, Error> {
    request_get::<RoundListInfo>(format!("/rounds")).await
}