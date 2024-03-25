use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct ScoreInfo {
    pub team_id: i32,
    pub game_id: i32,
    pub score: i32,
}

impl ScoreInfo {
    pub fn new(team_id: i32, game_id: i32) -> Self {
        Self {
            team_id,
            game_id,
            score: 0,
        }
    }
    pub fn set_score(&mut self, score: i32) {
        if score >= 0 && score <= 13 {
            self.score = score;
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ScoreListInfo {
    pub scores: Vec<ScoreInfo>,
}