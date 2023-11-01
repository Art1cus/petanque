use actix_web::web;

use super::team::{get_teams, get_teams_by_id};
use super::round::get_rounds;
use super::game::{get_games, get_games_by_field_round_id};

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api")
        .service(get_games_by_field_round_id)
        .service(get_games)
        .service(get_rounds)
        .service(get_teams)
        .service(get_teams_by_id);
    conf.service(scope);
}