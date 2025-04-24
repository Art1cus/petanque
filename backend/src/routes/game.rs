use crate::models::game::Game;
use chrono::NaiveDateTime;
use deadpool_postgres::Pool;

// TODO: check if I can delete this imports if also used somewhere else
use actix_web::{get, post, web, HttpResponse};

#[get("/games")]
pub async fn get_games(pool: web::Data<Pool>) -> HttpResponse {
    let client = match pool.get().await {
        Ok(client) => client,
        Err(err) => {
            log::debug!("unable to get postgres client: {:?}", err);
            return HttpResponse::InternalServerError().json("unable to get postgres client");
        }
    };
    match Game::all(&**client).await {
        Ok(list) => {
            log::debug!("able to fetch games: {:?}", list);
            HttpResponse::Ok().json(list)
        },
        Err(err) => {
            log::debug!("unable to fetch games: {:?}", err);
            return HttpResponse::InternalServerError().json("unable to fetch games");
        }
    }
}

#[get("/games/round/{round_id}")]
pub async fn get_games_by_round_id(pool: web::Data<Pool>, path: web::Path<i32>) -> HttpResponse {
    let round_id: i32 = path.into_inner();
    let client = match pool.get().await {
        Ok(client) => client,
        Err(err) => {
            log::debug!("unable to get postgres client: {:?}", err);
            return HttpResponse::InternalServerError().json("unable to get postgres client");
        }
    };
    match Game::by_round_id(&**client, round_id).await {
        Ok(list) => {
            log::debug!("able to fetch games: {:?}", list);
            HttpResponse::Ok().json(list)
        },
        Err(err) => {
            log::debug!("unable to fetch games: {:?}", err);
            return HttpResponse::InternalServerError().json("unable to fetch games");
        }
    }

}

#[get("/games/field/{field_id}")]
pub async fn get_games_by_field_id(pool: web::Data<Pool>, path: web::Path<i32>) -> HttpResponse {
    let field_id: i32 = path.into_inner();
    let client = match pool.get().await {
        Ok(client) => client,
        Err(err) => {
            log::debug!("unable to get postgres client: {:?}", err);
            return HttpResponse::InternalServerError().json("unable to get postgres client");
        }
    };
    match Game::by_field_id(&**client, field_id).await {
        Ok(list) => {
            log::debug!("able to fetch games: {:?}", list);
            HttpResponse::Ok().json(list)
        },
        Err(err) => {
            log::debug!("unable to fetch games: {:?}", err);
            return HttpResponse::InternalServerError().json("unable to fetch games");
        }
    }

}

#[get("/games/played/{is_played}")]
pub async fn get_games_is_played(pool: web::Data<Pool>, path: web::Path<bool>) -> HttpResponse {
    let is_played: bool = path.into_inner();
    let client = match pool.get().await {
        Ok(client) => client,
        Err(err) => {
            log::debug!("unable to get postgres client: {:?}", err);
            return HttpResponse::InternalServerError().json("unable to get postgres client");
        }
    };
    match Game::by_is_played(&**client, is_played).await {
        Ok(list) => {
            log::debug!("able to fetch games: {:?}", list);
            HttpResponse::Ok().json(list)
        },
        Err(err) => {
            log::debug!("unable to fetch games: {:?}", err);
            return HttpResponse::InternalServerError().json("unable to fetch games");
        }
    }

}

#[get("/games/field/{field_id}/round/{round_id}")]
pub async fn get_games_by_field_round_id(pool: web::Data<Pool>, path: web::Path<(i32, i32)>) -> HttpResponse {
    let field_id: i32 = path.0;
    let round_id: i32 = path.1;
    let client = match pool.get().await {
        Ok(client) => client,
        Err(err) => {
            log::debug!("unable to get postgres client: {:?}", err);
            return HttpResponse::InternalServerError().json("unable to get postgres client");
        }
    };
    match Game::by_field_round_id(&**client, field_id, round_id).await {
        Ok(list) => {
            log::debug!("able to fetch games: {:?}", list);
            HttpResponse::Ok().json(list)
        },
        Err(err) => {
            log::debug!("unable to fetch games: {:?}", err);
            return HttpResponse::InternalServerError().json("unable to fetch games");
        }
    }

}

#[get("/games/time/{start_datetime}")]
pub async fn get_games_by_start_time(pool: web::Data<Pool>, path: web::Path<String>) -> HttpResponse {
    let start_datetime: NaiveDateTime = NaiveDateTime::parse_from_str(path.into_inner().as_str(), "%Y-%m-%dT%H:%M:%S").unwrap();
    let client = match pool.get().await {
        Ok(client) => client,
        Err(err) => {
            log::debug!("unable to get postgres client: {:?}", err);
            return HttpResponse::InternalServerError().json("unable to get postgres client");
        }
    };
    match Game::by_start_time(&**client, start_datetime).await {
        Ok(list) => {
            log::debug!("able to fetch games: {:?}", list);
            HttpResponse::Ok().json(list)
        },
        Err(err) => {
            log::debug!("unable to fetch games: {:?}", err);
            return HttpResponse::InternalServerError().json("unable to fetch games");
        }
    }

}

#[post("/games/played/game/{game_id}")]
pub async fn set_played_true(pool: web::Data<Pool>, path: web::Path<i32>) -> HttpResponse {
    let game_id: i32 = path.into_inner();
    let client = match pool.get().await {
        Ok(client) => client,
        Err(err) => {
            log::debug!("unable to get postgres client: {:?}", err);
            return HttpResponse::InternalServerError().json("unable to get postgres client");
        }
    };
    match Game::set_played(&**client, game_id, true).await {
        Ok(list) => {
            log::debug!("able to fetch games: {:?}", list);
            HttpResponse::Ok().json(list)
        },
        Err(err) => {
            log::debug!("unable to fetch games: {:?}", err);
            return HttpResponse::InternalServerError().json("unable to fetch games");
        }
    }

}