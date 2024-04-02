use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct FieldInfo {
    pub id: i32,
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct FieldInfoWrapper {
    pub field: FieldInfo,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FieldListInfo {
    pub fields: Vec<FieldInfo>,
}