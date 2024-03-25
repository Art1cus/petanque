use yew::prelude::*;

use crate::components::game_list::{GameList, GameListFilter};

#[function_component(Home)]
pub fn home() -> Html {
    let filter = use_state(|| {
            GameListFilter::All
    });

    html! {
        <div class="home-page">
            <div class="container page">
                <div class="col-md-12 col-xs-12 row">
                    <GameList filter={(*filter).clone()} />
                </div>
            </div>
        </div>
    }
}
