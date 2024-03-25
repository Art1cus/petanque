use web_sys::console;

use super::{request_delete, request_get, request_post, request_put};
use crate::error::Error;
use crate::types::*;

/// Get all teams
pub async fn push_scores(scores: ScoreListInfo) -> Result<GameListInfo, Error> {
    console::log_1(&format!("reqwest data: {:?}", scores).into());
    request_get::<GameListInfo>(format!("/gameteams")).await
}