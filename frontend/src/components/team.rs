use yew::prelude::*;
use crate::types::TeamInfo;

#[derive(Properties, Clone, PartialEq, Eq)]
pub struct Props {
    pub team: TeamInfo,
}

/// Single team preview component used by team list.
#[function_component(Team)]
pub fn team_preview(props: &Props) -> Html {
    let team = use_state(|| props.team.clone());

    html! {
        <div class="col-md-6 col-xs-6">
            <h3>
                { &team.name }
            </h3>
            <p>{ &team.captain_name }</p>
        </div>
    }
}