use super::request_get;
use crate::error::Error;
use crate::types::*;

/// Get all teams
pub async fn all() -> Result<FieldListInfo, Error> {
    request_get::<FieldListInfo>(format!("/fields")).await
}