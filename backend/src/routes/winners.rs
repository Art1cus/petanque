use crate::models::winner::Winner;
use deadpool_postgres::Pool;

// TODO: check if I can delete this imports if also used somewhere else
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
            HttpResponse::Ok().json(list)
        },
        Err(err) => {
            log::debug!("unable to fetch winners: {:?}", err);
            return HttpResponse::InternalServerError().json("unable to fetch winners");
        }
    }
}
