use crate::models::{score::Score, Message};
use deadpool_postgres::Pool;

// TODO: check if I can delete this imports if also used somewhere else
use actix_web::{get, put, web, HttpResponse};

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

#[get("/scores/game/{game_id}")]
pub async fn get_scores_by_game_id(pool: web::Data<Pool>, path: web::Path<i32>) -> HttpResponse {
    let game_id = path.into_inner();
    let client = match pool.get().await {
        Ok(client) => client,
        Err(err) => {
            log::debug!("unable to get postgres client: {:?}", err);
            return HttpResponse::InternalServerError().json("unable to get postgres client");
        }
    };
    match Score::by_game_id(&**client, game_id).await {
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

#[get("/scores/game/{game_id}/team/{team_id}")]
pub async fn get_score_by_game_team_id(pool: web::Data<Pool>, path: web::Path<(i32, i32)>) -> HttpResponse {
    let game_id: i32 = path.0;
    let team_id: i32 = path.1;
    let client = match pool.get().await {
        Ok(client) => client,
        Err(err) => {
            log::debug!("unable to get postgres client: {:?}", err);
            return HttpResponse::InternalServerError().json("unable to get postgres client");
        }
    };
    match Score::by_game_team_id(&**client, game_id, team_id).await {
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

#[put("/scores/add")]
pub async fn update_or_insert_score(pool: web::Data<Pool>, score: web::Json<Score>) -> HttpResponse {
    let client = match pool.get().await {
        Ok(client) => client,
        Err(err) => {
            log::debug!("unable to get postgres client: {:?}", err);
            return HttpResponse::InternalServerError().json("unable to get postgres client");
        }
    };
    
    match Score::new(&**client, score.game_id, score.team_id, score.score).await {
        Ok(_) => HttpResponse::Ok().json(Message{message:"Score updated/inserted successfully".to_string()}),
        Err(err) => {
            log::debug!("unable to update score: {:?}", err);
            HttpResponse::InternalServerError().json(Message{message:"unable to update/insert score".to_string()})
        }
    }
}
