use gloo_net::http::{Headers, Request};
use serde::{Deserialize, Serialize};
use serde_json;
use wasm_bindgen::JsValue;
use web_sys::HtmlInputElement;

use yew::prelude::*;

use crate::components::input::InputField;

#[derive(Clone, PartialEq, Properties, Debug, Default, Serialize, Deserialize)]
pub struct RegistrationForm {
    pub score_team1: String,
    pub score_team2: String,
}

#[function_component(Home)]
pub fn home() -> Html {
    let registration_form = use_state(|| RegistrationForm::default());

    let score_team1_ref = use_node_ref();
    let score_team2_ref = use_node_ref();

    let name_team1 = "AA".to_string();
    let name_team2 = "BB".to_string();

    let both_filled_in = use_state(|| true);

    log::info!("registration_form {:?}", &registration_form.clone());
    let onsubmit = {
        let registration_form = registration_form.clone();

        let score_team1_ref = score_team1_ref.clone();
        let score_team2_ref = score_team2_ref.clone();

        // let form_is_valid = form_is_valid.clone();
        let both_filled_in = both_filled_in.clone();

        Callback::from(move |event: SubmitEvent| {
            event.prevent_default();
            log::info!("registration_form {:?}", &registration_form.clone());

            let score_team1 = score_team1_ref.cast::<HtmlInputElement>().unwrap().value();
            let score_team2 = score_team2_ref.cast::<HtmlInputElement>().unwrap().value();

            if score_team1 != "" && score_team2 != "" {
                both_filled_in.set(true);
            } else {
                both_filled_in.set(false);
                return;
            };

            log::info!("{} {}", score_team1, score_team2);

            let registration_form = RegistrationForm {
                score_team1,
                score_team2,
            };

            log::info!("registration_form {:?}", &registration_form);

            wasm_bindgen_futures::spawn_local(async move {
                let post_request = Request::post("http://127.0.0.1:8000/test")
                    .headers({
                        let headers = Headers::new();
                        headers.append("Access-Control-Allow-Origin", "*");
                        headers.append("Access-Control-Allow-Methods", "DELETE, POST, GET, OPTIONS");
                        headers.append("Access-Control-Allow-Headers", "Content-Type, Authorization, X-Requested-With");
                        headers.append("Content-Type", "application/json");
                        headers
                    })
                    .body(JsValue::from(
                        serde_json::to_string(&registration_form).unwrap(),
                    ))
                    .send()
                    .await
                    .unwrap();

                log::info!("post_request {:?}", &post_request);
            });
        })
    };

    html! {
        <main class="home">
            <h1>{"User Registration"}</h1>
            <form {onsubmit} class="registration-form">
                <InputField input_node_ref={score_team1_ref} label={name_team1.to_owned()} name={"team_1".clone()} field_type={"text".clone()} />
                <InputField input_node_ref={score_team2_ref} label={name_team2.to_owned()} name={"team_2".clone()} field_type={"text".clone()}  />
                <p class="error-text">{ if *both_filled_in { "" } else { "fill in both scores" } }</p>
                <button type="submit" class="button button-primary">{"Submit"}</button>
            </form>
        </main>
    }
}