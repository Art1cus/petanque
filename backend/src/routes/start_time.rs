use crate::models::start_time::StartTime;
use deadpool_postgres::Pool;

// TODO: check if I can delete this imports if also used somewhere else
use actix_web::{get, web, HttpResponse};

#[get("/start_times")]
pub async fn get_start_times(pool: web::Data<Pool>) -> HttpResponse {
    let client = match pool.get().await {
        Ok(client) => client,
        Err(err) => {
            log::debug!("unable to get postgres client: {:?}", err);
            return HttpResponse::InternalServerError().json("unable to get postgres client");
        }
    };
    match StartTime::all(&**client).await {
        Ok(list) => {
            log::debug!("able to fetch teams: {:?}", list);
            HttpResponse::Ok().json(list)
        },
        Err(err) => {
            log::debug!("unable to fetch teams: {:?}", err);
            return HttpResponse::InternalServerError().json("unable to fetch teams");
        }
    }
}