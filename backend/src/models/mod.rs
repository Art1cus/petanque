pub mod team;
pub mod round;
pub mod game;
pub mod score;
pub mod field;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Message {
    pub message: String,
}