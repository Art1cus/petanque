use serde::{Deserialize, Serialize};
use tokio_postgres::{Error, GenericClient, Row};

#[derive(Deserialize, Serialize, Debug)]
pub struct Game {
    pub id: i32,
    pub field_id: i32,
    pub round_id: i32,
    pub played: bool,
    pub start_time: Option<chrono::DateTime<chrono::Utc>>,
    pub end_time: Option<chrono::DateTime<chrono::Utc>>,
}

impl From<Row> for Game {
    fn from(row: Row) -> Self {
        Self {
            id: row.get(0),
            field_id: row.get(1),
            round_id: row.get(2),
            played: row.get(3),
            start_time: row.get(4),
            end_time: row.get(5),
        }
    }
}

impl Game {
    pub async fn all<C: GenericClient>(client: &C) -> Result<Vec<Game>, Error> {
        let stmt = client.prepare("SELECT match_id, field_id, round_id, played, match_start_time, match_end_time FROM matches").await?;
        let rows = client.query(&stmt, &[]).await?;
        Ok(rows.into_iter().map(Game::from).collect())
    }
    pub async fn by_field_round_id<C: GenericClient>(client: &C, field_id: i32, round_id: i32) -> Result<Vec<Game>, Error> {
        let stmt = client.prepare("SELECT match_id, field_id, round_id, played, match_start_time, match_end_time FROM matches WHERE field_id = $1 and round_id = $2").await?;
        let rows = client.query(&stmt, &[&field_id, &round_id]).await?;
        Ok(rows.into_iter().map(Game::from).collect())
    } 
}