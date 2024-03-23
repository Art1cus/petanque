use yew::prelude::*;
use yew_hooks::prelude::*;

use super::team::Team;
use crate::services::teams::*;

#[derive(Properties, Clone, PartialEq, Eq)]
pub struct Props {
    pub filter: TeamListFilter,
}

/// Filters for team list
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TeamListFilter {
    All,
    ById(u32),
}

/// List of teams component
#[function_component(TeamList)]
pub fn team_list(props: &Props) -> Html {
    let team_list = {
        let filter = props.filter.clone();

        use_async(async move {
            match filter {
                TeamListFilter::All => all().await,
                TeamListFilter::ById(id) => by_id(id).await,
            }
        })
    };

    {
        let team_list = team_list.clone();
        use_effect_with_deps(
            move |_| {
                team_list.run();
                || ()
            },
            props.filter.clone(),
        );
    }

    if let Some(team_list) = &team_list.data {
        if !team_list.teams.is_empty() {
            
            html! {
                <>
                    {for team_list.teams.iter().map(|team| {
                        html! { <Team team={team.clone()} /> }
                    })}
                </>
            }
        } else {
            html! {
                <div class="article-preview">{ "No articles are here... yet." }</div>
            }
        }
    } else {
        html! {
            <div class="article-preview">{ "Loading..." }</div>
        }
    }
}