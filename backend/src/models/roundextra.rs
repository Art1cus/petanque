use serde::{Deserialize, Serialize};
use tokio_postgres::{Error, GenericClient, Row};

#[derive(Deserialize, Serialize, Debug)]
pub struct RoundExtraInfo {
    pub id: i32,
    pub name: String,
    pub played_games: i32,
    pub total_games: i32,
    pub all_played: bool,
    pub select_winner: bool,
}

impl From<Row> for RoundExtraInfo {
    fn from(row: Row) -> Self {
        Self {
            id: row.get(0),
            name: row.get(1),
            played_games: row.get(2),
            total_games: row.get(3),
            all_played: row.get(4),
            select_winner: row.get(5),
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
                WHEN r.round_id <= 15 THEN
                    -- For group phase rounds (1-15), check if ALL games in the entire group phase are played
                    CASE
                        WHEN (SELECT COUNT(game_id) FROM games WHERE round_id <= 15 AND played = FALSE) = 0 THEN TRUE
                        ELSE FALSE
                    END
                ELSE
                    -- For knockout rounds 16+, check if all games in that specific round are played
                    CASE
                        WHEN COUNT(g.game_id) = 0 THEN FALSE
                        WHEN COUNT(g.game_id) = COUNT(CASE WHEN g.played = TRUE THEN 1 END) THEN TRUE
                        ELSE FALSE
                    END
            END AS game_status,
            r.select_winner
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