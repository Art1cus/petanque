use serde::{Deserialize, Serialize};
use tokio_postgres::{Error, GenericClient, Row};

#[derive(Deserialize, Serialize, Debug)]
pub struct Field {
    pub id: i32,
    pub name: String,
}

impl From<Row> for Field {
    fn from(row: Row) -> Self {
        Self {
            id: row.get(0),
            name: row.get(1),
        }
    }
}

impl Field {
    pub async fn all<C: GenericClient>(client: &C) -> Result<FieldList, Error> {
        let stmt = client.prepare("SELECT field_id, field_name FROM fields").await?;
        let rows = client.query(&stmt, &[]).await?;
        let fields: Vec<Field> = rows.into_iter().map(Field::from).collect();
        Ok(FieldList { fields })
    }
    pub async fn by_id<C: GenericClient>(client: &C, field_id: i32) -> Result<FieldList, Error> {
        let stmt = client.prepare("SELECT field_id, field_name FROM fields WHERE field_id = $1").await?;
        let rows = client.query(&stmt, &[&field_id]).await?;
        let fields: Vec<Field> = rows.into_iter().map(Field::from).collect();
        Ok(FieldList { fields })
    } 
}

#[derive(Deserialize, Serialize, Debug)]
pub struct FieldList {
    pub fields: Vec<Field>,
}