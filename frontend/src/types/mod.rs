//! Common types

mod teams;
mod games;
mod scores;
mod rounds;
mod fields;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub use teams::{TeamInfo, TeamInfoWrapper, TeamListInfo};
pub use games::{GameInfo, GameInfoWrapper, GameListInfo};
pub use scores::{ScoreInfo, ScoreInfoWrapper, ScoreListInfo};
pub use rounds::{RoundInfo, RoundInfoWrapper, RoundListInfo};
pub use fields::{FieldInfo, FieldInfoWrapper, FieldListInfo};

/// Conduit api error info for Unprocessable Entity error
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct ErrorInfo {
    pub errors: HashMap<String, Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct MessageInfo {
    pub message: String,
}

pub type DeleteWrapper = HashMap<(), ()>;