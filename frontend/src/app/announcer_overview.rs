use yew::prelude::*;

use crate::components::game_list::{GameList, GameListFilter};
use crate::components::filters::{RoundFilter, StartTimeFilter};

#[function_component(AnnouncerOverview)]
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

    let start_time_callback = {
        let filter = filter.clone();
        Callback::from(move |start_time| {
            if start_time != "".to_string() {filter.set(GameListFilter::ByStartTime(start_time))}
            else {filter.set(GameListFilter::All)}
        }) 
    };

    html! {
        <div class="home-page">
            <div class="container page">
                <div class="row" style="margin-bottom: 10px;">
                    <RoundFilter callback={round_callback}/>
                    <StartTimeFilter callback={start_time_callback}/>
                </div>
                <div>
                    <GameList filter={(*filter).clone()} editable={false} show_score={false}/>
                </div>
            </div>
        </div>
    }
}