use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct RoundInfo {
    pub id: i32,
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct RoundInfoWrapper {
    pub round: RoundInfo,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RoundListInfo {
    pub rounds: Vec<RoundInfo>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct RoundExtraInfo {
    pub id: i32,
    pub name: String,
    pub played_games: i32,
    pub total_games: i32,
    pub all_played: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct RoundExtraInfoWrapper {
    pub round: RoundExtraInfo,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RoundExtraListInfo {
    pub rounds: Vec<RoundExtraInfo>,
}