use crate::models::score::Score;
use deadpool_postgres::Pool;

// TODO: check if I can delete this imports if also used somewhere else
use actix_web::{get, web, HttpResponse};

#[get("/scores")]
pub async fn get_scores(pool: web::Data<Pool>) -> HttpResponse {
    let client = match pool.get().await {
        Ok(client) => client,
        Err(err) => {
            log::debug!("unable to get postgres client: {:?}", err);
            return HttpResponse::InternalServerError().json("unable to get postgres client");
        }
    };
    match Score::all(&**client).await {
        Ok(list) => {
            log::debug!("able to fetch scores: {:?}", list);
            HttpResponse::Ok().json(list)
        },
        Err(err) => {
            log::debug!("unable to fetch scores: {:?}", err);
            return HttpResponse::InternalServerError().json("unable to fetch scores");
        }
    }
}

#[get("/scores/match/{match_id}")]
pub async fn get_scores_by_match_id(pool: web::Data<Pool>, path: web::Path<i32>) -> HttpResponse {
    let match_id = path.into_inner();
    let client = match pool.get().await {
        Ok(client) => client,
        Err(err) => {
            log::debug!("unable to get postgres client: {:?}", err);
            return HttpResponse::InternalServerError().json("unable to get postgres client");
        }
    };
    match Score::by_match_id(&**client, match_id).await {
        Ok(list) => {
            log::debug!("able to fetch scores: {:?}", list);
            HttpResponse::Ok().json(list)
        },
        Err(err) => {
            log::debug!("unable to fetch scores: {:?}", err);
            return HttpResponse::InternalServerError().json("unable to fetch scores");
        }
    }
}

#[get("/scores/team/{team_id}")]
pub async fn get_scores_by_team_id(pool: web::Data<Pool>, path: web::Path<i32>) -> HttpResponse {
    let team_id = path.into_inner();
    let client = match pool.get().await {
        Ok(client) => client,
        Err(err) => {
            log::debug!("unable to get postgres client: {:?}", err);
            return HttpResponse::InternalServerError().json("unable to get postgres client");
        }
    };
    match Score::by_team_id(&**client, team_id).await {
        Ok(list) => {
            log::debug!("able to fetch scores: {:?}", list);
            HttpResponse::Ok().json(list)
        },
        Err(err) => {
            log::debug!("unable to fetch scores: {:?}", err);
            return HttpResponse::InternalServerError().json("unable to fetch scores");
        }
    }
}

#[get("/scores/match/{match_id}/team/{team_id}")]
pub async fn get_score_by_match_team_id(pool: web::Data<Pool>, path: web::Path<(i32, i32)>) -> HttpResponse {
    let match_id: i32 = path.0;
    let team_id: i32 = path.1;
    let client = match pool.get().await {
        Ok(client) => client,
        Err(err) => {
            log::debug!("unable to get postgres client: {:?}", err);
            return HttpResponse::InternalServerError().json("unable to get postgres client");
        }
    };
    match Score::by_match_team_id(&**client, match_id, team_id).await {
        Ok(list) => {
            log::debug!("able to fetch score: {:?}", list);
            HttpResponse::Ok().json(list)
        },
        Err(err) => {
            log::debug!("unable to fetch score: {:?}", err);
            return HttpResponse::InternalServerError().json("unable to fetch score");
        }
    }

}