use yew::prelude::*;
use yew_hooks::prelude::*;
use crate::types::RoundExtraInfo;

use crate::services::games::{get_winners_group_fase, get_winners_by_round, get_losers_by_round};

#[derive(Properties, Clone, PartialEq, Eq)]
pub struct Props {
    pub round: RoundExtraInfo,
}

/// Single team preview component used by team list.
#[function_component(Round)]
pub fn round_preview(props: &Props) -> Html {
    let round = use_state(|| props.round.clone());

    {
        let round = round.clone();
        use_effect_with(
            props.clone(),
            move |props| {
                round.set(props.round.clone());
            },
            
        )
    }

    let make_new_rounds = {
        let round  = round.clone();
        use_async(async move {
            if round.id == 1 {
                get_winners_group_fase().await
            }
            else {
                let winners_result = get_winners_by_round(round.id).await;
                if round.id == 2 {
                    get_losers_by_round(round.id).await
                }
                else {
                    winners_result
                }
            }
        })
    };

    let onclick = {
        Callback::from(move |ev: MouseEvent| {
            ev.prevent_default();
            make_new_rounds.run();
        })
    };

    html! {
        <div class="card round">
            <div class="card-body">
                <div class="round-info">
                    <h3>
                        { &round.name }
                    </h3>
                    <p>{round.played_games.to_string() + "/" + &round.total_games.to_string()}</p>
                </div>
                <button
                    class="btn btn-lg btn-primary"
                    type="submit"
                    onclick={onclick}
                    disabled={!&round.all_played}>
                    { "Advance winners to next round" }
                </button>
            </div>
        </div>
    }
}