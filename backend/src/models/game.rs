use serde::{Deserialize, Serialize};
use tokio_postgres::{Error, GenericClient, Row};

#[derive(Deserialize, Serialize, Debug)]
pub struct Game {
    pub id: i32,
    pub field_id: i32,
    pub round_id: i32,
    pub team_1_id: i32,
    pub team_2_id: i32,
    pub played: bool,
    pub start_time: Option<chrono::NaiveDateTime>,
    pub end_time: Option<chrono::NaiveDateTime>,
}

impl From<Row> for Game {
    fn from(row: Row) -> Self {
        Self {
            id: row.get(0),
            field_id: row.get(1),
            round_id: row.get(2),
            team_1_id: row.get(3),
            team_2_id: row.get(4),
            played: row.get(5),
            start_time: row.get(6),
            end_time: row.get(7),
        }
    }
}

impl Game {
    pub async fn all<C: GenericClient>(client: &C) -> Result<GameList, Error> {
        let stmt = client.prepare("SELECT match_id, field_id, round_id, team_1_id, team_2_id, played, start_datetime, end_datetime FROM matches").await?;
        let rows = client.query(&stmt, &[]).await?;
        let games: Vec<Game> = rows.into_iter().map(Game::from).collect();
        Ok(GameList { games })
    }
    pub async fn by_field_id<C: GenericClient>(client: &C, field_id: i32) -> Result<GameList, Error> {
        let stmt = client.prepare("SELECT match_id, field_id, round_id, team_1_id, team_2_id, played, start_datetime, end_datetime FROM matches WHERE field_id = $1").await?;
        let rows = client.query(&stmt, &[&field_id]).await?;
        let games: Vec<Game> = rows.into_iter().map(Game::from).collect();
        Ok(GameList { games })
    } 
    pub async fn by_round_id<C: GenericClient>(client: &C, round_id: i32) -> Result<GameList, Error> {
        let stmt = client.prepare("SELECT match_id, field_id, round_id, team_1_id, team_2_id, played, start_datetime, end_datetime FROM matches WHERE round_id = $1").await?;
        let rows = client.query(&stmt, &[&round_id]).await?;
        let games: Vec<Game> = rows.into_iter().map(Game::from).collect();
        Ok(GameList { games })
    } 
    pub async fn by_is_played<C: GenericClient>(client: &C, played: bool) -> Result<GameList, Error> {
        let stmt = client.prepare("SELECT match_id, field_id, round_id, team_1_id, team_2_id, played, start_datetime, end_datetime FROM matches WHERE played = $1").await?;
        let rows = client.query(&stmt, &[&played]).await?;
        let games: Vec<Game> = rows.into_iter().map(Game::from).collect();
        Ok(GameList { games })
    } 
    pub async fn by_field_round_id<C: GenericClient>(client: &C, field_id: i32, round_id: i32) -> Result<GameList, Error> {
        let stmt = client.prepare("SELECT match_id, field_id, round_id, team_1_id, team_2_id, played, start_datetime, end_datetime FROM matches WHERE field_id = $1 and round_id = $2").await?;
        let rows = client.query(&stmt, &[&field_id, &round_id]).await?;
        let games: Vec<Game> = rows.into_iter().map(Game::from).collect();
        Ok(GameList { games })
    } 
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GameList {
    pub games: Vec<Game>,
}