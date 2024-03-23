use gloo_net::http::{Headers, Request};
use serde::{Deserialize, Serialize};
use serde_json::{self, Value};
use web_sys::HtmlInputElement;

use yew::prelude::*;

use crate::components::team_list::{TeamList, TeamListFilter};

#[function_component(Home)]
pub fn home() -> Html {
    let filter = use_state(|| {
            TeamListFilter::All
    });

    {
        let filter = filter.clone();
    }

    html! {
        <div class="col-md-9 col-xs-12">
            <TeamList filter={(*filter).clone()} />
        </div>
    }
}
