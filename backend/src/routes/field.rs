use crate::models::field::Field;
use deadpool_postgres::Pool;

// TODO: check if I can delete this imports if also used somewhere else
use actix_web::{get, web, HttpResponse};

#[get("/fields")]
pub async fn get_fields(pool: web::Data<Pool>) -> HttpResponse {
    let client = match pool.get().await {
        Ok(client) => client,
        Err(err) => {
            log::debug!("unable to get postgres client: {:?}", err);
            return HttpResponse::InternalServerError().json("unable to get postgres client");
        }
    };
    match Field::all(&**client).await {
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

#[get("/fields/field/{field_id}")]
pub async fn get_field_by_id(pool: web::Data<Pool>, path: web::Path<i32>) -> HttpResponse {
    let field_id = path.into_inner();
    let client = match pool.get().await {
        Ok(client) => client,
        Err(err) => {
            log::debug!("unable to get postgres client: {:?}", err);
            return HttpResponse::InternalServerError().json("unable to get postgres client");
        }
    };
    match Field::by_id(&**client, field_id).await {
        Ok(list) => {
            log::debug!("able to fetch field: {:?}", list);
            HttpResponse::Ok().json(list)
        },
        Err(err) => {
            log::debug!("unable to fetch field: {:?}", err);
            return HttpResponse::InternalServerError().json("unable to fetch field");
        }
    }

}