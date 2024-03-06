use crate::models::round::Round;
use deadpool_postgres::Pool;

// TODO: check if I can delete this imports if also used somewhere else
use actix_web::{get, web, HttpResponse};

#[get("/rounds")]
pub async fn get_rounds(pool: web::Data<Pool>) -> HttpResponse {
    let client = match pool.get().await {
        Ok(client) => client,
        Err(err) => {
            log::debug!("unable to get postgres client: {:?}", err);
            return HttpResponse::InternalServerError().json("unable to get postgres client");
        }
    };
    match Round::all(&**client).await {
        Ok(list) => {
            log::debug!("able to fetch rounds: {:?}", list);
            HttpResponse::Ok().json(list)
        },
        Err(err) => {
            log::debug!("unable to fetch round: {:?}", err);
            return HttpResponse::InternalServerError().json("unable to fetch rounds");
        }
    }
}

#[get("/rounds/round/{round_id}")]
pub async fn get_round_by_id(pool: web::Data<Pool>, path: web::Path<i32>) -> HttpResponse {
    let round_id = path.into_inner();
    let client = match pool.get().await {
        Ok(client) => client,
        Err(err) => {
            log::debug!("unable to get postgres client: {:?}", err);
            return HttpResponse::InternalServerError().json("unable to get postgres client");
        }
    };
    match Round::by_id(&**client, round_id).await {
        Ok(list) => {
            log::debug!("able to fetch round: {:?}", list);
            HttpResponse::Ok().json(list)
        },
        Err(err) => {
            log::debug!("unable to fetch round: {:?}", err);
            return HttpResponse::InternalServerError().json("unable to fetch round");
        }
    }
}
