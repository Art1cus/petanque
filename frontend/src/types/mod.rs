//! Common types

mod teams;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub use teams::{TeamInfo, TeamInfoWrapper, TeamListInfo};

/// Conduit api error info for Unprocessable Entity error
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct ErrorInfo {
    pub errors: HashMap<String, Vec<String>>,
}

pub type DeleteWrapper = HashMap<(), ()>;