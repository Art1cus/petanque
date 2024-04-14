use super::request_get;
use crate::error::Error;
use crate::types::*;

/// Get all rounds
pub async fn all() -> Result<RoundListInfo, Error> {
    request_get::<RoundListInfo>(format!("/rounds")).await
}

pub async fn all_extra() -> Result<RoundExtraListInfo, Error>{
    request_get::<RoundExtraListInfo>(format!("/roundsextra")).await
}