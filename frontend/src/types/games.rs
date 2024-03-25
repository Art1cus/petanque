use serde::{Deserialize, Serialize};
use chrono::prelude::*;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct GameInfo {
    pub id: i32,
    pub field_id: i32,
    pub round_id: i32,
    pub played: bool,
    pub start_time: NaiveDateTime,
    pub end_time: NaiveDateTime,
    pub team_1_id: i32,
    pub team_2_id: i32,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct GameInfoWrapper {
    pub game: GameInfo,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GameListInfo {
    pub games: Vec<GameInfo>,
}