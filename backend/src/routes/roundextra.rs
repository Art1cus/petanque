use crate::models::roundextra::RoundExtraInfo;
use deadpool_postgres::Pool;

// TODO: check if I can delete this imports if also used somewhere else
use actix_web::{get, web, HttpResponse};

#[get("/roundsextra")]
pub async fn get_rounds_w_extrainfo(pool: web::Data<Pool>) -> HttpResponse {
    let client = match pool.get().await {
        Ok(client) => client,
        Err(err) => {
            log::debug!("unable to get postgres client: {:?}", err);
            return HttpResponse::InternalServerError().json("unable to get postgres client");
        }
    };
    match RoundExtraInfo::all(&**client).await {
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