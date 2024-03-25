use crate::models::gameteams::GameTeams;
use deadpool_postgres::Pool;

// TODO: check if I can delete this imports if also used somewhere else
use actix_web::{get, web, HttpResponse};

#[get("/gameteams")]
pub async fn get_gameteams(pool: web::Data<Pool>) -> HttpResponse {
    let client = match pool.get().await {
        Ok(client) => client,
        Err(err) => {
            log::debug!("unable to get postgres client: {:?}", err);
            return HttpResponse::InternalServerError().json("unable to get postgres client");
        }
    };
    match GameTeams::all(&**client).await {
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

#[get("/gameteams/round/{round_id}")]
pub async fn get_gameteams_by_round_id(pool: web::Data<Pool>, path: web::Path<i32>) -> HttpResponse {
    let round_id: i32 = path.into_inner();
    let client = match pool.get().await {
        Ok(client) => client,
        Err(err) => {
            log::debug!("unable to get postgres client: {:?}", err);
            return HttpResponse::InternalServerError().json("unable to get postgres client");
        }
    };
    match GameTeams::by_round_id(&**client, round_id).await {
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

#[get("/gameteams/field/{field_id}")]
pub async fn get_gameteams_by_field_id(pool: web::Data<Pool>, path: web::Path<i32>) -> HttpResponse {
    let field_id: i32 = path.into_inner();
    let client = match pool.get().await {
        Ok(client) => client,
        Err(err) => {
            log::debug!("unable to get postgres client: {:?}", err);
            return HttpResponse::InternalServerError().json("unable to get postgres client");
        }
    };
    match GameTeams::by_field_id(&**client, field_id).await {
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

#[get("/gameteams/played/{is_played}")]
pub async fn get_gameteams_is_played(pool: web::Data<Pool>, path: web::Path<bool>) -> HttpResponse {
    let is_played: bool = path.into_inner();
    let client = match pool.get().await {
        Ok(client) => client,
        Err(err) => {
            log::debug!("unable to get postgres client: {:?}", err);
            return HttpResponse::InternalServerError().json("unable to get postgres client");
        }
    };
    match GameTeams::by_is_played(&**client, is_played).await {
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

#[get("/gameteams/field/{field_id}/round/{round_id}")]
pub async fn get_gameteams_by_field_round_id(pool: web::Data<Pool>, path: web::Path<(i32, i32)>) -> HttpResponse {
    let field_id: i32 = path.0;
    let round_id: i32 = path.1;
    let client = match pool.get().await {
        Ok(client) => client,
        Err(err) => {
            log::debug!("unable to get postgres client: {:?}", err);
            return HttpResponse::InternalServerError().json("unable to get postgres client");
        }
    };
    match GameTeams::by_field_round_id(&**client, field_id, round_id).await {
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