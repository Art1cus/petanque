use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Result {
    pub id: i32,
    pub match_id: i32,
    pub team_id: i32,
    pub score: i32,
}