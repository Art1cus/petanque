use yew::prelude::*;
use yew_hooks::prelude::*;

use super::game::Game;
use crate::services::games;

#[derive(Properties, Clone, PartialEq, Eq)]
pub struct Props {
    pub filter: GameListFilter,
    pub editable: bool,
}

/// Filters for team list
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GameListFilter {
    All,
    ByRoundId(i32),
    ByFieldId(i32),
    IsPlayed(bool),
    ByRoundIdFieldId(i32, i32),
}

/// List of teams component
#[function_component(GameList)]
pub fn game_list(props: &Props) -> Html {
    let editable = use_state(|| props.editable.clone());
    let game_list = {
        let filter = props.filter.clone();

        use_async(async move {
            match filter {
                GameListFilter::All => games::all().await,
                GameListFilter::ByRoundId(id) => games::by_round_id(id).await,
                GameListFilter::ByFieldId(id) => games::by_field_id(id).await,
                GameListFilter::IsPlayed(id) => games::by_is_played(id).await,
                GameListFilter::ByRoundIdFieldId(round_id, field_id) => games::by_round_id_field_id(round_id, field_id).await,
            }
        })
    };

    

    {
        let game_list = game_list.clone();
        let props = props.clone();
        use_effect_with(
            props.filter.clone(),
            move |_| {
                game_list.run();
            },
        );
    }

    {
        let editable = editable.clone();
        let props = props.clone();
        use_effect_with(
            props.editable.clone(),
            move |_| {
                editable.set(props.editable)
            },
        );
    }

    if let Some(game_list) = &game_list.data {
        if !game_list.games.is_empty() {
            html! {
                <>
                    {for game_list.games.iter().map(|game| {
                        html! { <Game game={game.clone()} editable={*editable.clone()}/> }
                    })}
                </>
            }
        } else {
            html! {
                <div class="article-preview">{ "No games are here... yet." }</div>
            }
        }
    } else {
        html! {
            <div class="article-preview">{ "Loading..." }</div>
        }
    }
}