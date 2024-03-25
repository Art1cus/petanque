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
                <div class="col-md-12 col-xs-12 row">
                    <TeamList filter={(*filter).clone()} />
                </div>
            </div>
        </div>
    }
}