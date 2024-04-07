use serde::{Deserialize, Serialize};
use tokio_postgres::{Error, GenericClient, Row};

#[derive(Deserialize, Serialize, Debug)]
pub struct Score {
    pub id: Option<i32>,
    pub game_id: i32,
    pub team_id: i32,
    pub score: i32,
}

impl From<Row> for Score {
    fn from(row: Row) -> Self {
        Self {
            id: row.get(0),
            game_id: row.get(1),
            team_id: row.get(2),
            score: row.get(3),
        }
    }
}

impl Score {
    pub async fn all<C: GenericClient>(client: &C) -> Result<ScoreList, Error> {
        let stmt = client.prepare("SELECT result_id, game_id, team_id, score FROM game_results").await?;
        let rows = client.query(&stmt, &[]).await?;
        let scores: Vec<Score> = rows.into_iter().map(Score::from).collect();
        Ok(ScoreList { scores })
    }
    pub async fn by_game_id<C: GenericClient>(client: &C, game_id: i32) -> Result<ScoreList, Error> {
        let stmt = client.prepare("SELECT result_id, game_id, team_id, score FROM game_results WHERE game_id = $1").await?;
        let rows = client.query(&stmt, &[&game_id]).await?;
        let scores: Vec<Score> = rows.into_iter().map(Score::from).collect();
        Ok(ScoreList { scores })
    } 
    pub async fn by_team_id<C: GenericClient>(client: &C, team_id: i32) -> Result<ScoreList, Error> {
        let stmt = client.prepare("SELECT result_id, game_id, team_id, score FROM game_results WHERE team_id = $1").await?;
        let rows = client.query(&stmt, &[&team_id]).await?;
        let scores: Vec<Score> = rows.into_iter().map(Score::from).collect();
        Ok(ScoreList { scores })
    } 
    pub async fn by_game_team_id<C: GenericClient>(client: &C, game_id: i32, team_id: i32) -> Result<ScoreList, Error> {
        let stmt = client.prepare("SELECT result_id, game_id, team_id, score FROM game_results WHERE game_id = $1 and team_id = $2").await?;
        let rows = client.query(&stmt, &[&game_id, &team_id]).await?;
        let scores: Vec<Score> = rows.into_iter().map(Score::from).collect();
        Ok(ScoreList { scores })
    } 
    pub async fn new<C: GenericClient>(client: &C, game_id: i32, team_id: i32, score: i32) -> Result<(), Error> {
        let stmt = client.prepare("INSERT INTO game_results (game_id, team_id, score) VALUES ($1, $2, $3) ON CONFLICT ON CONSTRAINT unique_game_team_constraint DO UPDATE SET score = $3 WHERE game_results.game_id = $1 AND game_results.team_id = $2").await?;
        client.execute(&stmt, &[&game_id, &team_id, &score]).await?;
        Ok(())
    } 
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ScoreList {
    pub scores: Vec<Score>,
}