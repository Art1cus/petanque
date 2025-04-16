use yew::prelude::*;
use yew_router::prelude::*;

use crate::app::AppRoute;

#[function_component(Header)]
pub fn header() -> Html {

    html! {
        <nav class="navbar navbar-light">
            <div class="container">
                <Link<AppRoute> to={AppRoute::Home} classes="navbar-brand">
                    { "Boule App" }
                </Link<AppRoute>>
                <ul class="nav navbar-nav pull-xs-right">
                    <li class="nav-item">
                        <Link<AppRoute> to={AppRoute::GamesView} classes="nav-link">
                            { "Scores" }
                        </Link<AppRoute>>
                    </li>
                    <li class="nav-item">
                        <Link<AppRoute> to={AppRoute::RoundsView} classes="nav-link">
                            { "Rondes" }
                        </Link<AppRoute>>
                    </li>
                    <li class="nav-item">
                        <Link<AppRoute> to={AppRoute::RulesView} classes="nav-link">
                            { "Regels" }
                        </Link<AppRoute>>
                    </li>
                    <li class="nav-item">
                        <Link<AppRoute> to={AppRoute::TeamsView} classes="nav-link">
                            { "Teams" }
                        </Link<AppRoute>>
                    </li>
                    <li class="nav-item">
                        <Link<AppRoute> to={AppRoute::AnnouncerView} classes="nav-link">
                            { "Ronde Overzicht" }
                        </Link<AppRoute>>
                    </li>
                </ul>
            </div>
        </nav>
    }
}