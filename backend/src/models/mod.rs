pub mod team;
pub mod round;
pub mod game;
pub mod score;
pub mod field;
pub mod winner;
pub mod roundextra;
pub mod start_time;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Message {
    pub message: String,
}