use web_sys::HtmlInputElement;

use yew::prelude::*;
use yew_hooks::use_async;

use crate::services::scores::push_scores;
use crate::types::{GameInfo, ScoreInfo, ScoreListInfo};

#[derive(Properties, Clone, PartialEq, Eq)]
pub struct Props {
    pub game: GameInfo,
    pub editable: bool,
}

/// Single team preview component used by team list.
#[function_component(ScoreInput)]
pub fn score_input(props: &Props) -> Html {
    let game = use_state(|| props.game.clone());
    let editable = use_state(|| props.editable.clone());

    let score_1 = use_state( || ScoreInfo::new(game.team_1_id, game.id, Some(0)));
    let score_2 = use_state( || ScoreInfo::new(game.team_2_id, game.id, Some(0)));

    {
        let game = game.clone();
        let editable = editable.clone();
        let score_1 = score_1.clone();
        let score_2 = score_2.clone();
        use_effect_with(
            props.clone(),
            move |props| {
                game.set(props.game.clone());
                editable.set(props.editable.clone());
                score_1.set(ScoreInfo::new(props.game.team_1_id, props.game.id, Some(0)));
                score_2.set(ScoreInfo::new(props.game.team_2_id, props.game.id, Some(0)));
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
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default(); /* Prevent event propagation */
            submit_scores.run();
        })
    };

    html! {
        <div class="col-md-12 col-xs-12">
            <form {onsubmit}>
                <fieldset class="row">
                    <fieldset class="form-group col-md-6 col-xs-12">
                        <input
                            class="form-control form-control-lg"
                            type="number"
                            value={score_1.score.clone().to_string()}
                            oninput={oninput_team_1_score}
                            disabled={game.played & !*editable}
                            />
                    </fieldset>
                    <fieldset class="form-group col-md-6 col-xs-12">
                        <input
                            class="form-control form-control-lg"
                            type="number"
                            value={score_2.score.clone().to_string()}
                            oninput={oninput_team_2_score}
                            disabled={game.played & !*editable}
                            />
                    </fieldset>
                    <button
                        class="btn btn-lg btn-primary pull-xs-right col-md-12"
                        type="submit"
                        disabled={game.played & !*editable}>
                        { "Submit Score" }
                    </button>
                </fieldset>
            </form>
        </div>
    }
}