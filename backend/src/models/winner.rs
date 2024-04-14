use serde::{Deserialize, Serialize};
use tokio_postgres::{Error, GenericClient, Row};

#[derive(Deserialize, Serialize, Debug)]
pub struct Winner {
    pub group_id: i32,
    pub team_id: i32,
}

impl From<Row> for Winner {
    fn from(row: Row) -> Self {
        Self {
            group_id: row.get(0),
            team_id: row.get(1),
        }
    }
}

impl Winner {
    pub async fn all_group_fase<C: GenericClient>(client: &C) -> Result<Winners, Error> {
        let stmt = client.prepare("WITH scores AS (
            SELECT
                ga.game_id,
                gr.team_id,
                t.group_id,
                CASE 
                    WHEN gr.score > COALESCE(gr_opp.score, 0) THEN 2
                    WHEN gr.score < COALESCE(gr_opp.score, 0) THEN 0
                    ELSE 1 
                END AS wins_count,
                gr.score AS score
            FROM
                games ga
            JOIN
                (
                    SELECT
                        game_id,
                        team_id,
                        score
                    FROM
                        game_results
                ) gr ON ga.game_id = gr.game_id
            JOIN
                teams t ON gr.team_id = t.team_id
            LEFT JOIN
                (
                    SELECT
                        game_id,
                        team_id,
                        score
                    FROM
                        game_results
                ) gr_opp ON ga.game_id = gr_opp.game_id AND (gr.team_id <> gr_opp.team_id)
        ),
        group_wins AS(
            SELECT
                team_id,
                group_id,
                SUM(wins_count) AS total_wins,
                SUM(score) AS total_score
            FROM
                scores
            GROUP BY
                team_id,
                group_id
            ORDER BY
                team_id
        ),
        group_ranks AS (
            SELECT
                group_id,
                team_id,
                ROW_NUMBER() OVER (PARTITION BY group_id ORDER BY total_wins DESC, total_score DESC) AS rank
            FROM
                group_wins
        )
        SELECT
            group_id,
            team_id
        FROM
            group_ranks
        WHERE
            rank = 1;").await?;
        let rows = client.query(&stmt, &[]).await?;
        let winners: Vec<Winner> = rows.into_iter().map(Winner::from).collect();
        Ok(Winners { winners })
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Winners {
    pub winners: Vec<Winner>,
}