use std::convert::TryInto;
use chrono::{NaiveDateTime, Duration};

use crate::models::{game::Game, winner::Winner};
use deadpool_postgres::Pool;

use actix_web::{get, web, HttpResponse};

#[get("/winners/groupfase")]
pub async fn get_winners(pool: web::Data<Pool>) -> HttpResponse {
    let client = match pool.get().await {
        Ok(client) => client,
        Err(err) => {
            log::debug!("unable to get postgres client: {:?}", err);
            return HttpResponse::InternalServerError().json("unable to get postgres client");
        }
    };
    match Winner::all_group_fase(&**client).await {
        Ok(list) => {
            log::debug!("able to fetch winners: {:?}", list);
            let start_datetime = NaiveDateTime::parse_from_str("2024-04-20 17:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
            let end_datetime = start_datetime + Duration::minutes(15);

            for i in (0..16).step_by(2) {
                let winner_1 = &list.winners[i];
                let winner_2 = &list.winners[i+1];
                let field_id: i32 = (i/2+1).try_into().unwrap();
                match Game::new(&**client, field_id, 2, winner_1.team_id, winner_2.team_id, false, start_datetime, end_datetime).await {
                    Ok(()) => {
                        log::debug!("able to crate new games from winners: {:?}", list);
                    },
                    Err(err) => {
                        log::debug!("unable to crate new games from winners: {:?}", err);
                    }
                }
            };
            HttpResponse::Ok().json(list)
        },
        Err(err) => {
            log::debug!("unable to fetch winners: {:?}", err);
            return HttpResponse::InternalServerError().json("unable to fetch winners");
        }
    }
}

#[get("/winners/round/{round_id}")]
pub async fn get_winners_by_round(pool: web::Data<Pool>, path: web::Path<i32>) -> HttpResponse {
    let round_id = path.into_inner();
    let client = match pool.get().await {
        Ok(client) => client,
        Err(err) => {
            log::debug!("unable to get postgres client: {:?}", err);
            return HttpResponse::InternalServerError().json("unable to get postgres client");
        }
    };
    match Winner::all_normal_fase(&**client, round_id).await {
        Ok(list) => {
            log::debug!("able to fetch winners: {:?}", list);
            let initial_datetime = NaiveDateTime::parse_from_str("2024-04-20 17:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
            let tdelta: i64 = (15*(round_id+1)).try_into().unwrap();
            let start_datetime = initial_datetime + Duration::minutes(tdelta);
            let end_datetime = start_datetime + Duration::minutes(15);

            for i in (0..list.winners.len()).step_by(2) {
                let winner_1 = &list.winners[i];
                let winner_2 = &list.winners[i+1];
                let field_id: i32 = (i/2+1).try_into().unwrap();
                match Game::new(&**client, field_id, round_id+1, winner_1.team_id, winner_2.team_id, false, start_datetime, end_datetime).await {
                    Ok(()) => {
                        log::debug!("able to crate new games from winners: {:?}", list);
                    },
                    Err(err) => {
                        log::debug!("unable to crate new games from winners: {:?}", err);
                    }
                }
            };
            HttpResponse::Ok().json(list)
        },
        Err(err) => {
            log::debug!("unable to fetch winners: {:?}", err);
            return HttpResponse::InternalServerError().json("unable to fetch winners");
        }
    }
}

#[get("/losers/round/{round_id}")]
pub async fn get_losers_by_round(pool: web::Data<Pool>, path: web::Path<i32>) -> HttpResponse {
    let round_id = path.into_inner();
    let client = match pool.get().await {
        Ok(client) => client,
        Err(err) => {
            log::debug!("unable to get postgres client: {:?}", err);
            return HttpResponse::InternalServerError().json("unable to get postgres client");
        }
    };
    match Winner::all_losers_normal_fase(&**client, round_id).await {
        Ok(list) => {
            log::debug!("able to fetch losers: {:?}", list);
            let initial_datetime = NaiveDateTime::parse_from_str("2024-04-20 17:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
            let tdelta: i64 = (15*(round_id+1)).try_into().unwrap();
            let start_datetime = initial_datetime + Duration::minutes(tdelta);
            let end_datetime = start_datetime + Duration::minutes(15);

            for i in (0..list.winners.len()).step_by(2) {
                let winner_1 = &list.winners[i];
                let winner_2 = &list.winners[i+1];
                let field_id: i32 = (i/2+1).try_into().unwrap();
                match Game::new(&**client, field_id, round_id+2, winner_1.team_id, winner_2.team_id, false, start_datetime, end_datetime).await {
                    Ok(()) => {
                        log::debug!("able to crate new games from losers: {:?}", list);
                    },
                    Err(err) => {
                        log::debug!("unable to crate new games from losers: {:?}", err);
                    }
                }
            };
            HttpResponse::Ok().json(list)
        },
        Err(err) => {
            log::debug!("unable to fetch losers: {:?}", err);
            return HttpResponse::InternalServerError().json("unable to fetch losers");
        }
    }
}