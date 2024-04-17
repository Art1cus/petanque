use web_sys::console;

use super::{request_put, request_post, request_get};
use crate::error::Error;
use crate::types::*;

pub async fn push_scores(scores: ScoreListInfo) -> Result<(), Error> {
    console::log_1(&format!("scores  data: {:?}", scores).into());

    for score in &scores.scores {
        console::log_1(&format!("score data: {:?}", score).into());
        request_put::<ScoreInfo, MessageInfo>(format!("/scores/add"), score.clone()).await?;
    }
    request_post(format!("/games/played/game/{}", scores.scores[0].game_id.clone()), "{}").await?;
    Ok(())
}

/// Get teams filtered by id
pub async fn by_game_id_team_id(game_id: i32, team_id: i32) -> Result<ScoreListInfo, Error> {
    request_get::<ScoreListInfo>(format!("/scores/game/{}/team/{}", game_id, team_id)).await
}