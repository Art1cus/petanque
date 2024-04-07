use web_sys::console;

use super::{request_put, request_post};
use crate::error::Error;
use crate::types::*;

/// Get all teams
pub async fn push_scores(scores: ScoreListInfo) -> Result<(), Error> {
    console::log_1(&format!("scores data: {:?}", scores).into());

    for score in &scores.scores {
        console::log_1(&format!("score data: {:?}", score).into());
        request_put::<ScoreInfo, MessageInfo>(format!("/scores/add"), score.clone()).await?;
    }
    request_post(format!("/games/played/game/{}", scores.scores[0].game_id.clone()), "{}").await?;
    Ok(())
}