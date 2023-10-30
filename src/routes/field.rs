use crate::models::team::Team;
use deadpool_postgres::Pool;

// TODO: check if I can delete this imports if also used somewhere else
use actix_web::{get, web, HttpResponse};

#[get("/teams")]
pub async fn get_teams(pool: web::Data<Pool>) -> HttpResponse {
    let client = match pool.get().await {
        Ok(client) => client,
        Err(err) => {
            log::debug!("unable to get postgres client: {:?}", err);
            return HttpResponse::InternalServerError().json("unable to get postgres client");
        }
    };
    match Team::all(&**client).await {
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

#[get("/teams/team/{team_id}")]
pub async fn get_teams_by_id(pool: web::Data<Pool>, path: web::Path<i32>) -> HttpResponse {
    let team_id = path.into_inner();
    let client = match pool.get().await {
        Ok(client) => client,
        Err(err) => {
            log::debug!("unable to get postgres client: {:?}", err);
            return HttpResponse::InternalServerError().json("unable to get postgres client");
        }
    };
    match Team::by_id(&**client, team_id).await {
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