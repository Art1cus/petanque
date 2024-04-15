use yew::prelude::*;

use crate::components::team_list::{TeamList, TeamListFilter};

#[function_component(TeamView)]
pub fn home() -> Html {
    let filter = use_state(|| {
            TeamListFilter::All
    });

    html! {
        <div class="home-page">
            <div class="container page">
                    <TeamList filter={(*filter).clone()} />
            </div>
        </div>
    }
}