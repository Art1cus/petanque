//! Routes by yew_router
pub mod home;

use yew::prelude::*;
use yew_router::prelude::*;

use home::Home;

use crate::components::{
    header::Header,
};

/// App routes
#[derive(Routable, Debug, Clone, PartialEq, Eq)]
pub enum AppRoute {
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(route: AppRoute) -> Html {
    match route {
        AppRoute::Home => html! {<Home />},
        AppRoute::NotFound => html! { "Page not found" },
    }
}

/// The root app component
#[function_component(App)]
pub fn app() -> Html {
    html! {
        <HashRouter>
            <Header />
            <Switch<AppRoute> render={switch} />
        </HashRouter>
    }
}