use web_sys::HtmlInputElement;

use yew::prelude::*;
use yew_hooks::prelude::*;

use crate::services::scores::{by_game_id_team_id, push_scores};
use crate::types::{GameInfo, ScoreInfo, ScoreListInfo};
use crate::services::teams;

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

    // let team_1 = {
    //     let game = game.clone();
    //     use_async_with_options(
    //         async move {teams::by_id(game.team_1_id).await},
    //         UseAsyncOptions::enable_auto(), 
    //     )
    // };

    // let team_2 = {
    //     let game = game.clone();
    //     use_async_with_options(
    //         async move {teams::by_id(game.team_2_id).await},
    //         UseAsyncOptions::enable_auto(), 
    //     )
    // };
    
    let score_1 = use_state( || ScoreInfo::new(game.team_1_id, game.id, Some(0)));
    let score_2 = use_state( || ScoreInfo::new(game.team_2_id, game.id, Some(0)));

    let score_1_get = {
        let game = game.clone();
        use_async(async move {by_game_id_team_id(game.id, game.team_1_id).await})
    };

    let score_2_get = {
        let game = game.clone();
        use_async(async move {by_game_id_team_id(game.id, game.team_2_id).await})
    };

    {
        let score_1_get = score_1_get.clone();
        use_effect_with(
            props.clone(),
            move |_props| {
                score_1_get.run()
            },
        )
    }

    {
        let score_1 = score_1.clone();
        let game = game.clone();
        use_effect_with(
            score_1_get,
            move |score_1_get| {
                if let Some(score) = &score_1_get.data {
                    if !score.scores.is_empty() {
                        score_1.set(score.scores[0].clone());
                    }
                    else {
                        score_1.set(ScoreInfo::new(game.team_1_id, game.id, Some(0)));
                    }
                }
            },
        )
    }

    {
        let score_2_get = score_2_get.clone();
        use_effect_with(
            props.clone(),
            move |_props| {
                score_2_get.run()
            },
        )
    }

    {
        let score_2 = score_2.clone();
        let game = game.clone();
        use_effect_with(
            score_2_get,
            move |score_2_get| {
                if let Some(score) = &score_2_get.data {
                    if !score.scores.is_empty() {
                        score_2.set(score.scores[0].clone());
                    }
                    else {
                        score_2.set(ScoreInfo::new(game.team_2_id, game.id, Some(0)));
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

    let submit_scores = {
        let scores = Vec::from([(*score_1).clone(),(*score_2).clone()]);
        use_async(async move {
            let request = ScoreListInfo {
                scores: scores.clone(),
            };
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
        // let reload_games = props.reload_games.clone();
        let game = game.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            submit_scores.run();
            let mut _game = (*game).clone();
            _game.played = true;
            game.set(_game);
            // reload_games.emit(());
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