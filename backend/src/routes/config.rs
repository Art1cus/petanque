use actix_web::web;

use super::field::{get_field_by_id, get_fields};
use super::team::{get_teams, get_teams_by_id};
use super::round::{get_rounds, get_round_by_id};
use super::game::{get_games, get_games_by_field_id, get_games_by_round_id, get_games_is_played, get_games_by_field_round_id};
use super::score::{get_scores, get_score_by_match_team_id, get_scores_by_match_id, get_scores_by_team_id};

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api")
        .service(get_games)
        .service(get_games_by_field_id)
        .service(get_games_by_round_id)
        .service(get_games_is_played)
        .service(get_games_by_field_round_id)
        .service(get_rounds)
        .service(get_round_by_id)
        .service(get_teams)
        .service(get_teams_by_id)
        .service(get_fields)
        .service(get_field_by_id)
        .service(get_scores)
        .service(get_scores_by_match_id)
        .service(get_scores_by_team_id)
        .service(get_score_by_match_team_id);
    conf.service(scope);
}