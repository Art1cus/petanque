use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Field {
    pub id: i32,
    pub name: String,
    pub location: String,
}