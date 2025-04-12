// components/auth_gate.rs

use yew::prelude::*;
use crate::{components::login::LoginView, services::request_get};

#[derive(Properties, PartialEq)]
pub struct AuthGateProps {
    pub children: Children,
}

#[function_component(AuthGate)]
pub fn auth_gate(props: &AuthGateProps) -> Html {
    let is_logged_in = use_state(|| None); // None = loading, Some(true/false) = result

    {
        let is_logged_in = is_logged_in.clone();
        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let resp =request_get::<String>(format!("/check_auth")).await;
                match resp {
                    Ok(_) => is_logged_in.set(Some(true)),
                    Err(_) => is_logged_in.set(Some(false)), // fallback
                }
            });
            || ()
        });
    }

    match *is_logged_in {
        None => html! { <p>{ "Checking login status..." }</p> },
        Some(true) => html! { for props.children.iter() },
        Some(false) => html! { <LoginView /> },
    }
}
