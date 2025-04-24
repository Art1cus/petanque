use serde::{Deserialize, Serialize};
use tokio_postgres::{Error, GenericClient, Row};

#[derive(Deserialize, Serialize, Debug)]
pub struct Team {
    pub id: i32,
    pub name: String,
    pub group_id: i32
}

impl From<Row> for Team {
    fn from(row: Row) -> Self {
        Self {
            id: row.get(0),
            name: row.get(1),
            group_id: row.get(2),
        }
    }
}

impl Team {
    pub async fn all<C: GenericClient>(client: &C) -> Result<TeamList, Error> {
        let stmt = client.prepare("SELECT team_id, team_name, group_id FROM teams").await?;
        let rows = client.query(&stmt, &[]).await?;
        let teams: Vec<Team> = rows.into_iter().map(Team::from).collect();
        Ok(TeamList { teams })
    }
    pub async fn by_id<C: GenericClient>(client: &C, team_id: i32) -> Result<TeamList, Error> {
        let stmt = client.prepare("SELECT team_id, team_name, group_id FROM teams WHERE team_id = $1").await?;
        let rows = client.query(&stmt, &[&team_id]).await?;
        let teams: Vec<Team> = rows.into_iter().map(Team::from).collect();
        Ok(TeamList { teams })
    } 
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TeamList {
    pub teams: Vec<Team>,
}