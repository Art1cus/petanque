use yew::prelude::*;
use yew_hooks::prelude::*;

use crate::components::round::Round;
use crate::services::rounds;

#[function_component(Home)]
pub fn home() -> Html {

    let round_list = {
        use_async_with_options(
            async move {rounds::all_extra().await},
            UseAsyncOptions::enable_auto(), 
        )
    };
    
    html! {
        <div class="home-page">
            <div class="container page">
                {
                    if let Some(round_list) = &round_list.data {
                        if !round_list.rounds.is_empty() {
                            html! {
                                <>
                                    {for round_list.rounds.iter().map(|round| {
                                        html! { 
                                            <Round round={round.clone()}/>
                                        }
                                    })}
                                </>
                            }
                        } else {
                            html! {
                                <div class="article-preview">{ "No rounds are here... yet." }</div>
                            }
                        }
                    } else {
                        html! {
                            <div class="article-preview">{ "Loading..." }</div>
                        }
                    }
                }
            </div>
        </div>
    }
}
