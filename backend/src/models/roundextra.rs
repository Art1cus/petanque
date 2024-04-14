use serde::{Deserialize, Serialize};
use tokio_postgres::{Error, GenericClient, Row};

#[derive(Deserialize, Serialize, Debug)]
pub struct RoundExtraInfo {
    pub id: i32,
    pub name: String,
    pub played_games: i32,
    pub total_games: i32,
    pub all_played: bool,
}

impl From<Row> for RoundExtraInfo {
    fn from(row: Row) -> Self {
        Self {
            id: row.get(0),
            name: row.get(1),
            played_games: row.get(2),
            total_games: row.get(3),
            all_played: row.get(4)
        }
    }
}

impl RoundExtraInfo {
    pub async fn all<C: GenericClient>(client: &C) -> Result<RoundExtraInfoList, Error> {
        let stmt = client.prepare("SELECT
            r.round_id,
            r.round_name,
            COUNT(CASE WHEN g.played = TRUE THEN 1 END)::INTEGER AS played_games,
            COUNT(g.game_id)::INTEGER AS total_games,
            CASE
                WHEN COUNT(g.game_id) = 0 THEN FALSE
                WHEN COUNT(g.game_id) = COUNT(CASE WHEN g.played = TRUE THEN 1 END) THEN TRUE
                ELSE FALSE
            END AS game_status
        FROM
            rounds r
        LEFT JOIN
            games g ON r.round_id = g.round_id
        GROUP BY
            r.round_id
        ORDER BY
            r.round_id;").await?;
        let rows = client.query(&stmt, &[]).await?;
        let rounds: Vec<RoundExtraInfo> = rows.into_iter().map(RoundExtraInfo::from).collect();
        Ok(RoundExtraInfoList { rounds })
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct RoundExtraInfoList {
    pub rounds: Vec<RoundExtraInfo>,
}