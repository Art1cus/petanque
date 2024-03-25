use web_sys::HtmlInputElement;

use yew::prelude::*;
use yew_hooks::use_async;

use crate::services::scores::push_scores;
use crate::types::{ScoreInfo, ScoreListInfo};

#[derive(Properties, Clone, PartialEq, Eq)]
pub struct Props {
    pub game_id: i32,
    pub team_1_id: i32,
    pub team_2_id: i32,
}

/// Single team preview component used by team list.
#[function_component(ScoreInput)]
pub fn score_input(props: &Props) -> Html {
    let game_id = props.game_id.clone();
    let team_1_id =props.team_1_id.clone();
    let team_2_id = props.team_2_id.clone();

    let score_1 = use_state( || ScoreInfo::new(team_1_id, game_id));
    let score_2 = use_state( || ScoreInfo::new(team_2_id, game_id));

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
        <div class="col-md-6 col-xs-6">
            <p>{ &team_1_id}</p>
            <p>{ &team_2_id }</p>
            <form {onsubmit}>
                <fieldset>
                    <fieldset class="form-group">
                        <input
                            class="form-control form-control-lg"
                            type="number"
                            placeholder="Email"
                            value={score_1.score.clone().to_string()}
                            oninput={oninput_team_1_score}
                            />
                    </fieldset>
                    <fieldset class="form-group">
                        <input
                            class="form-control form-control-lg"
                            type="number"
                            placeholder="Password"
                            value={score_2.score.clone().to_string()}
                            oninput={oninput_team_2_score}
                            />
                    </fieldset>
                    <button
                        class="btn btn-lg btn-primary pull-xs-right"
                        type="submit"
                        disabled=false>
                        { "Submit Score" }
                    </button>
                </fieldset>
            </form>
        </div>
    }
}