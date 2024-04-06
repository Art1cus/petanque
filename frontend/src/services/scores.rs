use web_sys::console;

use super::request_put;
use crate::error::Error;
use crate::types::*;

/// Get all teams
pub async fn push_scores(scores: ScoreListInfo) -> Result<(), Error> {
    console::log_1(&format!("scores data: {:?}", scores).into());

    for score in scores.scores {
        console::log_1(&format!("score data: {:?}", score).into());
        request_put::<ScoreInfo, MessageInfo>(format!("/scores/add"), score.clone()).await?;
    }
    Ok(())
}