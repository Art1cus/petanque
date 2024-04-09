use yew::prelude::*;
use yew_hooks::prelude::*;
use crate::types::GameInfo;
use crate::services::teams;
use super::team::Team;
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

    let team_1 = {
        let game = game.clone();
        use_async_with_options(
            async move {teams::by_id(game.team_1_id).await},
            UseAsyncOptions::enable_auto(), 
        )
    };

    let team_2 = {
        let game = game.clone();
        use_async_with_options(
            async move {teams::by_id(game.team_2_id).await},
            UseAsyncOptions::enable_auto(), 
        )
    };

    {
        let game = game.clone();
        let team_1 = team_1.clone();
        let team_2 = team_2.clone();
        use_effect_with(
            props.clone(),
            move |props| {
                game.set(props.game.clone());
                team_1.run();
                team_2.run();
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
        <div class="col-md-12 col-xs-12">
            <h3>
                { &game.start_time.to_string() }
            </h3>
            <div class="row"> {
                if let Some(team_1) = &team_1.data {
                    if !team_1.teams.is_empty() {
                        html! {
                            <>
                                {for team_1.teams.iter().map(|team| {
                                    html! { <Team team={team.clone()} /> }
                                })}
                            </>
                        }
                    } else {
                        html! {
                            <div class="article-preview">{ "No teams are here... yet." }</div>
                        }
                    }
                } else {
                    html! {
                        <div class="article-preview">{ "Loading..." }</div>
                    }
                }
            }
            {
                if let Some(team_2) = &team_2.data {
                    if !team_2.teams.is_empty() {
                        html! {
                            <>
                                {for team_2.teams.iter().map(|team| {
                                    html! { <Team team={team.clone()} /> }
                                })}
                            </>
                        }
                    } else {
                        html! {
                            <div class="article-preview">{ "No teams are here... yet." }</div>
                        }
                    }
                } else {
                    html! {
                        <div class="article-preview">{ "Loading..." }</div>
                    }
                }
            }
            </div>
            <div>
                <ScoreInput game={(*game).clone()} editable={editable.unwrap_or(false)} reload_games={props.reload_games.clone()}/>
            </div>
        </div>
    }
}