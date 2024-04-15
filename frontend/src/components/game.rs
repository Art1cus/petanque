use yew::prelude::*;
use crate::types::GameInfo;
use super::score_input::ScoreInput;


#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub game: GameInfo,
    pub editable: Option<bool>,
    pub reload_games: Callback<()>, 
}

/// Single game preview component used by game list.
#[function_component(Game)]
pub fn game_preview(props: &Props) -> Html {
    let game = use_state(|| props.game.clone());
    let editable = use_state(|| props.editable.clone());    

    {
        let game = game.clone();
        use_effect_with(
            props.clone(),
            move |props| {
                game.set(props.game.clone());
            },
            
        )
    }

    {
        let editable = editable.clone();
        use_effect_with(
            props.clone(), 
            move |props| {
                editable.set(props.editable.clone())
            },
        )
    }

    html! {
        <div class="card">
            <div class="card-body">
                <h3 style="border-bottom: 1px solid lightgray;">
                    {game.start_time.format("%H:%M-").to_string().clone() + &game.end_time.format("%H:%M  %b  %d").to_string()}
                </h3>
                <div>
                    <ScoreInput game={(*game).clone()} editable={editable.unwrap_or(false)} reload_games={props.reload_games.clone()}/>
                </div>
            </div>
        </div>
    }
}