use super::request_get;
use crate::error::Error;
use crate::types::*;

/// Get all rounds
pub async fn all() -> Result<StartTimeListInfo, Error> {
    request_get::<StartTimeListInfo>(format!("/start_times")).await
}