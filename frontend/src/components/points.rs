use yew::prelude::*;
use web_sys::HtmlInputElement as InputElement;
use yew::html::Scope;
use yew::events::KeyboardEvent;
use gloo_console::log;

enum Msg {
    PushPoints(i32, i32),
}

struct PointsComponent {
    team_a: i32,
    team_b: i32,
}


impl Component for PointsComponent {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            team_a: 0, 
            team_b: 0,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool{
        match msg {
            Msg::PushPoints(team, points) => {
                log!("in message:", team, points);
                if team == 1 {
                    self.team_a = points;
                }
                else {
                    self.team_b = points;
                }
            },
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
            <header>
                <h1>{"Petanque Tournament Score Tracker"}</h1>
            </header>
            <div class="container">
                <div class="team">
                    <label for="team1">{"Team 1:"}</label>
                    { self.score_input(1) }
                </div>
                <div class="team">
                    <label for="team2">{"Team 2:"}</label>
                    { self.score_input(2) }
                </div>
                <button onclick={self.push_points(ctx.link())}>{"Add Points"}</button>
                <p id="score">{"Total Score: 0"}</p>

                <div class="match-list">
                    <div class="match-item">
                        <span>{self.team_a}</span>
                    </div>
                    <div class="match-item">
                        <span>{self.team_b}</span>
                    </div>
                </div>
            </div>
            </>
        }   
    }
}

impl PointsComponent {
    fn score_input(&self, team: i32) -> Html {
        html! {
           <input type="number" id={team.to_string()} value={self.get_score(team)}/>
        }
    }
    fn get_score(&self, team: i32) -> String {
        if team == 1 {
            self.team_a.to_string()
        }
        else {
            self.team_b.to_string()
        }
    }
    fn push_points(&self, link: &Scope<Self>) -> Html {

    }
}

fn main() {
    yew::Renderer::<PointsComponent>::new().render();
}