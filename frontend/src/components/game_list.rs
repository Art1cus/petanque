use yew::prelude::*;
use yew_hooks::prelude::*;

use super::game::Game;
use crate::services::{games, teams};

#[derive(Properties, Clone, PartialEq, Eq)]
pub struct Props {
    pub filter: GameListFilter,
}

/// Filters for team list
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GameListFilter {
    All,
    ByRoundId(i32),
    ByFieldId(i32),
    IsPlayed(bool),
}

/// List of teams component
#[function_component(GameList)]
pub fn game_list(props: &Props) -> Html {
    let game_list = {
        let filter = props.filter.clone();

        use_async(async move {
            match filter {
                GameListFilter::All => games::all().await,
                GameListFilter::ByRoundId(id) => games::by_round_id(id).await,
                GameListFilter::ByFieldId(id) => games::by_field_id(id).await,
                GameListFilter::IsPlayed(id) => games::by_is_played(id).await,
            }
        })
    };

    {
        let game_list = game_list.clone();
        use_effect_with_deps(
            move |_| {
                game_list.run();
                || ()
            },
            props.filter.clone(),
        );
    }

    if let Some(game_list) = &game_list.data {
        if !game_list.games.is_empty() {
            html! {
                <>
                    {for game_list.games.iter().map(|game| {
                        html! { <Game game={game.clone()} /> }
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