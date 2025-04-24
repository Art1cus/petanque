use web_sys::HtmlInputElement;

use web_sys::console;

use yew::prelude::*;
use yew_hooks::prelude::*;

use crate::services::scores::{by_game_id, push_scores};
use crate::types::{GameInfo, ScoreInfo, ScoreListInfo};

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub game: GameInfo,
    pub editable: bool,
    pub reload_games: Callback<()>,
}

/// Single team preview component used by team list.
#[function_component(ScoreInput)]
pub fn score_input(props: &Props) -> Html {
    let game = use_state(|| props.game.clone());
    let editable = use_state(|| props.editable.clone());

    
    let score_1 = use_state( || ScoreInfo::new(game.team_1_id, game.id, Some(0)));
    let score_2 = use_state( || ScoreInfo::new(game.team_2_id, game.id, Some(0)));


    let scores_get = {
        let game = game.clone();
        use_async(async move {by_game_id(game.id).await})
    };

    {
        let scores_get = scores_get.clone();
        use_effect_with(
            props.clone(),
            move |_props| {
                scores_get.run()
            },
        )
    }

    {
        let score_1 = score_1.clone();
        let score_2 = score_2.clone();
        let game = game.clone();
        use_effect_with(
            scores_get,
            move |scores_get| {
                if let Some(scores) = &scores_get.data {
                    for score in &scores.scores {
                        if score.team_id == game.team_1_id {
                            score_1.set(score.clone());
                        } else if score.team_id == game.team_2_id {
                            score_2.set(score.clone());
                        }
                    }
                }
            },
        )
    }

    {
        let game = game.clone();
        let editable = editable.clone();
        use_effect_with(
            props.clone(),
            move |props| {
                game.set(props.game.clone());
                editable.set(props.editable.clone());
            },
            
        )
    }

    {
        let score_1 = score_1.clone();
        let score_2 = score_2.clone();
        let game = game.clone();
        use_effect_with(
            game.clone(),
            move |game| {
                // Only reset if the game id changed
                if game.id != score_1.game_id || game.id != score_2.game_id {
                    score_1.set(ScoreInfo::new(game.team_1_id, game.id, Some(0)));
                    score_2.set(ScoreInfo::new(game.team_2_id, game.id, Some(0)));
                }
            },
        );
    }

    let submit_scores = {
        let scores = Vec::from([(*score_1).clone(),(*score_2).clone()]);
        use_async(async move {
            let request = ScoreListInfo {
                scores: scores.clone(),
            };
            console::log_1(&format!("scores data submit: {:?}", scores).into());
            push_scores(request).await
        })
    };

    let oninput_team_1_score = {
        let score_1 = score_1.clone();
        Callback::from( move | e: InputEvent | {
            let input: HtmlInputElement = e.target_unchecked_into();
            if let Ok(value) = input.value().parse::<i32>() {
                let mut score = (*score_1).clone();
                score.set_score(value);
                score_1.set(score);
            }
        })
    };

    let oninput_team_2_score = {
        let score_2 = score_2.clone();
        Callback::from( move | e: InputEvent | {
            let input: HtmlInputElement = e.target_unchecked_into();
            if let Ok(value) = input.value().parse::<i32>() {
                let mut score = (*score_2).clone();
                score.set_score(value);
                score_2.set(score);
            }
        })
    };

    let onsubmit = {
        let submit_scores = submit_scores.clone();
        let game = game.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            submit_scores.run();
            let mut _game = (*game).clone();
            _game.played = true;
            game.set(_game);
        })
    };

    html! {
        <div class="col-md-12 col-xs-12">
            <form {onsubmit}>
                <fieldset class="row">
                    <fieldset class="form-group col-md-6 col-xs-12"> {
                            html! {
                                <h3>{game.team_1_name.clone()}</h3>
                            }              
                        }
                        <input
                            class="form-control form-control-lg"
                            type="number"
                            value={score_1.score.clone().to_string()}
                            oninput={oninput_team_1_score}
                            disabled={game.played & !*editable}
                            />
                    </fieldset>
                    <fieldset class="form-group col-md-6 col-xs-12"> {
                            html! {
                                <h3>{game.team_2_name.clone()}</h3>
                            }              
                        }
                        <input
                            class="form-control form-control-lg"
                            type="number"
                            value={score_2.score.clone().to_string()}
                            oninput={oninput_team_2_score}
                            disabled={game.played & !*editable}
                            />
                    </fieldset>
                    <button
                        class="btn btn-lg btn-primary submit-btn"
                        type="submit"
                        disabled={game.played & !*editable}>
                        { "Submit Score" }
                    </button>
                </fieldset>
            </form>
        </div>
    } 
}