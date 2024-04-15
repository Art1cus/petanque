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

    {
        let team = team.clone();
        use_effect_with(
            props.clone(),
            move |props| {
                team.set(props.team.clone());
            },
            
        )
    }

    html! {
        <div class="card col-xs-12">
            <div class="card-body">
                <h3>
                    { &team.name }
                </h3>
                <p style="margin-bottom: 0px;">{ &team.captain_name }</p>
            </div>
        </div>
    }
}