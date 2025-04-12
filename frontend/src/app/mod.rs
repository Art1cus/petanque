//! Routes by yew_router
pub mod home;
pub mod teamview;
pub mod gamesview;
pub mod rules;

use yew::prelude::*;
use yew_router::prelude::*;

use home::Home;
use teamview::TeamView;
use gamesview::GamesView;
use rules::RulesView;

use crate::components::header::Header;
use crate::components::auth_gate::AuthGate;

/// App routes
#[derive(Routable, Debug, Clone, PartialEq, Eq)]
pub enum AppRoute {
    #[at("/")]
    Home,
    #[at("/teamview")]
    TeamsView,
    #[at("/rounds")]
    RoundsView,
    #[at("/scores")]
    GamesView,
    #[at("/rules")]
    RulesView,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(route: AppRoute) -> Html {
    match route {
        AppRoute::Home => html! {<GamesView />},
        AppRoute::GamesView => html! {<GamesView />},
        AppRoute::TeamsView => html! {<TeamView />},
        AppRoute::RoundsView => html!(<Home />),
        AppRoute::RulesView => html!(<RulesView />),
        AppRoute::NotFound => html! { "Page not found" },
    }
}

/// The root app component
#[function_component(App)]
pub fn app() -> Html {
    html! {
        <AuthGate>
            <HashRouter>
                <Header />
                <Switch<AppRoute> render={switch} />
            </HashRouter>
        </AuthGate>
    }
}