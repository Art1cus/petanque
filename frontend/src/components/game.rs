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
                <h5 style="border-bottom: 1px solid lightgray;">
                    {game.start_time.format("%H:%M-").to_string().clone() + &game.end_time.format("%H:%M").to_string() + "   -   " + &game.field_name.to_string()}
                </h5>
                <div>
                    <ScoreInput game={props.game.clone()} editable={editable.unwrap_or(false)} reload_games={&props.reload_games}/>
                </div>
            </div>
        </div>
    }
}

#[function_component(GameNoInput)]
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
                <h5 style="border-bottom: 1px solid lightgray;">
                    {game.start_time.format("%H:%M-").to_string().clone() + &game.end_time.format("%H:%M").to_string() + "   -   " + &game.field_name.to_string()}
                </h5>
                <div class="row">
                    <div class="col-auto ml-3">
                        <h3 style="margin-left: 15px">{game.team_1_name.clone()}</h3>
                    </div>
                    <div class="col-auto ml-3">
                        <h3  style="margin-left: 15px">{"vs"}</h3>
                    </div>
                    <div class="col-auto ml-3">
                        <h3  style="margin-left: 15px">{game.team_2_name.clone()}</h3>
                    </div>
                </div>
            </div>
        </div>
    }
}