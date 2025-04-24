use serde::{Deserialize, Serialize};
use tokio_postgres::{Error, GenericClient, Row};

#[derive(Deserialize, Serialize, Debug)]
pub struct StartTime {
    pub id: i32,
    pub start_time_dt: chrono::NaiveDateTime,
}

impl From<Row> for StartTime {
    fn from(row: Row) -> Self {

        let _id: i64 = row.get(0);

        Self {
            id: _id as i32,
            start_time_dt: row.get(1),
        }
    }
}

impl StartTime {
    pub async fn all<C: GenericClient>(client: &C) -> Result<StartTimeList, Error> {
        let stmt = client.prepare("SELECT 
                ROW_NUMBER() OVER (ORDER BY start_datetime) AS id,
                start_datetime
            FROM (
                SELECT DISTINCT start_datetime
                FROM public.games
            ) AS distinct_dates
            ORDER BY start_datetime;").await?;
        let rows = client.query(&stmt, &[]).await?;
        let start_times: Vec<StartTime> = rows.into_iter().map(StartTime::from).collect();
        Ok(StartTimeList { start_times })
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct StartTimeList {
    pub start_times: Vec<StartTime>,
}