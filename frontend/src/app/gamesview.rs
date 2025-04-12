use yew::prelude::*;

use crate::components::game_list::{GameList, GameListFilter};
use crate::components::filters::{RoundFilter, FieldFilter};

#[function_component(GamesView)]
pub fn home() -> Html {
    let filter = use_state(|| {
        GameListFilter::All
    });

    let round_callback = {
        let filter = filter.clone();
        Callback::from(move |id| {
            if id > 0 {filter.set(GameListFilter::ByRoundId(id))}
            else {filter.set(GameListFilter::All)}
        })
    };

    let field_callback = {
        let filter = filter.clone();
        Callback::from(move |id| {
            if id > 0 {filter.set(GameListFilter::ByFieldId(id))}
            else {filter.set(GameListFilter::All)}
        })
    };

    let editable = use_state(|| {
        false
    });

    let editable_callback = {
        let editable = editable.clone();
        Callback::from(move |_| {
            editable.set(!(*editable).clone()) 
        })
    };

    html! {
        <div class="home-page">
            <div class="container page">
                <div class="row" style="margin-bottom: 10px;">
                    <RoundFilter callback={round_callback}/>
                    <FieldFilter callback={field_callback}/>
                </div>
                <div>
                    <GameList filter={(*filter).clone()} editable={(*editable).clone()} />
                </div>
                <div class="card">
                    <div class="card-body">
                        <div class="custom-checkbox">
                            <label>
                                <input type="checkbox" checked={(*editable).clone()} onchange={editable_callback}/>
                                {"Edit scores"}
                            </label>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}