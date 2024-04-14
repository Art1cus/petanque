use yew::prelude::*;
use crate::types::RoundExtraInfo;

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
                    disabled={!&round.all_played}>
                    { "Advance winners to next round" }
                </button>
            </div>
        </div>
    }
}