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