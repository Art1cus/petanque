use yew::prelude::*;
use yew_hooks::prelude::*;
use yew_router::prelude::*;

use crate::app::AppRoute;
use crate::services::teams::*;
use crate::types::TeamInfo;

const FAVORITED_CLASS: &str = "btn btn-sm btn-primary";
const NOT_FAVORITED_CLASS: &str = "btn btn-sm btn-outline-primary";

#[derive(Properties, Clone, PartialEq, Eq)]
pub struct Props {
    pub team: TeamInfo,
}

/// Single team preview component used by team list.
#[function_component(Team)]
pub fn team_preview(props: &Props) -> Html {
    let team = use_state(|| props.team.clone());

    {
        let team = team.clone();
    }

    html! {
        <div class="article-preview">
            <h1>
                { &team.name }
            </h1>
            <p>{ &team.captain_name }</p>
            <p>{ &team.contact_email }</p>
        </div>
    }
}