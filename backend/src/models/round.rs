use serde::{Deserialize, Serialize};
use tokio_postgres::{Error, GenericClient, Row};

#[derive(Deserialize, Serialize, Debug)]
pub struct Round {
    pub id: i32,
    pub name: String,
}

impl From<Row> for Round {
    fn from(row: Row) -> Self {
        Self {
            id: row.get(0),
            name: row.get(1),
        }
    }
}

impl Round {
    pub async fn all<C: GenericClient>(client: &C) -> Result<RoundList, Error> {
        let stmt = client.prepare("SELECT round_id, round_name FROM rounds").await?;
        let rows = client.query(&stmt, &[]).await?;
        let rounds: Vec<Round> = rows.into_iter().map(Round::from).collect();
        Ok(RoundList { rounds })
    }
    pub async fn by_id<C: GenericClient>(client: &C, round_id: i32) -> Result<RoundList, Error> {
        let stmt = client.prepare("SELECT round_id, round_name FROM rounds WHERE round_id = $1").await?;
        let rows = client.query(&stmt, &[&round_id]).await?;
        let rounds: Vec<Round> = rows.into_iter().map(Round::from).collect();
        Ok(RoundList { rounds })
    } 
}

#[derive(Deserialize, Serialize, Debug)]
pub struct RoundList {
    pub rounds: Vec<Round>,
}