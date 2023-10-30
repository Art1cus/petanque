use serde::{Deserialize, Serialize};
use tokio_postgres::{Error, GenericClient, Row};

#[derive(Deserialize, Serialize, Debug)]
pub struct Round {
    pub id: i32,
    pub name: String,
    pub start_time: chrono::DateTime<chrono::Utc>,
    pub end_time: chrono::DateTime<chrono::Utc>
}

impl From<Row> for Round {
    fn from(row: Row) -> Self {
        Self {
            id: row.get(0),
            name: row.get(1),
            start_time: row.get(2),
            end_time: row.get(3),
        }
    }
}

impl Round {
    pub async fn all<C: GenericClient>(client: &C) -> Result<Vec<Round>, Error> {
        let stmt = client.prepare("SELECT round_id, round_name, start_date, end_date FROM rounds").await?;
        let rows = client.query(&stmt, &[]).await?;
        Ok(rows.into_iter().map(Round::from).collect())
    }
//     pub async fn by_id<C: GenericClient>(client: &C, round_id: i32) -> Result<Vec<Round>, Error> {
//         let stmt = client.prepare("SELECT round_id, round_name, start_date, end_date FROM rounds WHERE round_id = $1").await?;
//         let rows = client.query(&stmt, &[&round_id]).await?;
//         Ok(rows.into_iter().map(Round::from).collect())
//     } 
}