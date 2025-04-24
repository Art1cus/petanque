use serde::{Deserialize, Serialize};
use chrono::prelude::*;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct StartTimeInfo {
    pub id: i32,
    pub start_time_dt: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct StartTimeInfoWrapper {
    pub start_time: StartTimeInfo,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct StartTimeListInfo {
    pub start_times: Vec<StartTimeInfo>,
}