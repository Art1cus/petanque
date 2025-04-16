// components/login.rs

use yew::prelude::*;
use serde::Serialize;
use crate::error::Error;
use crate::services::{request_post, request_get};

#[derive(Serialize, Debug)]
struct LoginRequest {
    password: String,
}

#[function_component(LoginView)]
pub fn login_view() -> Html {
    let password = use_state(|| "".to_string());
    let message = use_state(|| "".to_string());
    let password_clone = password.clone();
    let message_clone = message.clone();
    let password_on_input = Callback::from(move |e: InputEvent| {
        let input: web_sys::HtmlInputElement = e.target_unchecked_into();
        password_clone.set(input.value());
    });

    let on_login = Callback::from(move |_| {
        let password = password.clone();
        let message = message.clone();

        wasm_bindgen_futures::spawn_local(async move {
            let payload = LoginRequest {
                password: (*password).clone(),
            };

            let res = request_post::<LoginRequest, String>(format!("/login"), payload).await;


            match res {
                Ok(_) => {
                    message.set("Login successful!".to_string());
                    gloo_utils::window().location().reload().unwrap(); // trigger AuthGate to recheck
                }
                Err(Error::Unauthorized) => message.set("Invalid password".to_string()),
                Err(_) => message.set("Something went wrong".to_string()),
            }
        });
    });

    html! {
    <div class="home-page">
        <div class="container page">
            <div class="card round">
                <div class="card-body">
                    <h2>{ "Please log in" }</h2>
                    <input style="margin-bottom:10px;" type="password" placeholder="Password" oninput={password_on_input} />
                    <button onclick={on_login} class="btn btn-lg btn-primary">{ "Login" }</button>
                    <p>{ (*message_clone).clone()}</p>
                </div>
            </div>
        </div>
    </div>
    }
}
