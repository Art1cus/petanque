use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct TeamInfo {
    pub id: i32,
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct TeamInfoWrapper {
    pub team: TeamInfo,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TeamListInfo {
    pub teams: Vec<TeamInfo>,
}