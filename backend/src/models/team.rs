use serde::{Deserialize, Serialize};
use tokio_postgres::{Error, GenericClient, Row};

#[derive(Deserialize, Serialize, Debug)]
pub struct Team {
    pub id: i32,
    pub name: String,
    pub captain_name: String,
    pub contact_email: String
}

impl From<Row> for Team {
    fn from(row: Row) -> Self {
        Self {
            id: row.get(0),
            name: row.get(1),
            captain_name: row.get(2),
            contact_email: row.get(3),
        }
    }
}

impl Team {
    pub async fn all<C: GenericClient>(client: &C) -> Result<Vec<Team>, Error> {
        let stmt = client.prepare("SELECT team_id, team_name, captain_name, contact_email FROM teams").await?;
        let rows = client.query(&stmt, &[]).await?;
        Ok(rows.into_iter().map(Team::from).collect())
    }
    pub async fn by_id<C: GenericClient>(client: &C, team_id: i32) -> Result<Vec<Team>, Error> {
        let stmt = client.prepare("SELECT team_id, team_name, captain_name, contact_email FROM teams WHERE team_id = $1").await?;
        let rows = client.query(&stmt, &[&team_id]).await?;
        Ok(rows.into_iter().map(Team::from).collect())
    } 
}
