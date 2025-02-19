use yew::prelude::*;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew_router::prelude::*;
use crate::app::Route;

#[derive(Serialize, Deserialize)]
struct RegisterRequest {
    username: String,
    email: String,
    password: String,
}

#[derive(Serialize, Deserialize)]
struct AuthResponse {
    token: String,
    user_id: String,
}


#[function_component(RegisterComponent)]
pub fn register_component() -> Html {
    let username = use_state(|| String::new());
    let email = use_state(|| String::new());
    let password = use_state(|| String::new());
    let is_loading = use_state(|| false);
    let navigator = use_navigator().expect("Navigator should be available");

    let on_submit = {
        let username = username.clone();
        let email = email.clone();
        let password = password.clone();
        let is_loading = is_loading.clone();
        let navigator = navigator.clone();

        Callback::from(move |_| {
            let navigator = navigator.clone();
            let is_loading = is_loading.clone();
            is_loading.set(true);

            let request = RegisterRequest {
                username: (*username).clone(),
                email: (*email).clone(),
                password: (*password).clone(),
            };

            spawn_local(async move {
                let client = Client::new();
                let res = client
                    .post("http://localhost:3000/register")
                    .json(&request)
                    .send()
                    .await;

                if let Ok(response) = res {
                    if response.status().is_success() {
                        log::info!("Registration successful!");
                        navigator.push(&Route::Login);
                    } else {
                        log::error!("Registration failed!");
                    }
                } else {
                    log::error!("Request error!");
                }

                is_loading.set(false);
            });
        })
    };


    html! {
        <div class="d-flex justify-content-center align-items-center vh-100 bg-light">
            <div class="card shadow-lg" style="width: 25rem;">
                <div class="card-body">
                    <h2 class="card-title text-center mb-4">{"Register"}</h2>

                    <div class="mb-3">
                        <label class="form-label">{"Username"}</label>
                        <input
                            type="text"
                            class="form-control"
                            placeholder="Enter username"
                            value={(*username).clone()}
                            oninput={Callback::from(move |e: InputEvent| {
                                let input: HtmlInputElement = e.target_unchecked_into();
                                username.set(input.value());
                            })}
                        />
                    </div>

                    <div class="mb-3">
                        <label class="form-label">{"Email"}</label>
                        <input
                            type="email"
                            class="form-control"
                            placeholder="Enter email"
                            value={(*email).clone()}
                            oninput={Callback::from(move |e: InputEvent| {
                                let input: HtmlInputElement = e.target_unchecked_into();
                                email.set(input.value());
                            })}
                        />
                    </div>

                    <div class="mb-3">
                        <label class="form-label">{"Password"}</label>
                        <input
                            type="password"
                            class="form-control"
                            placeholder="Enter password"
                            value={(*password).clone()}
                            oninput={Callback::from(move |e: InputEvent| {
                                let input: HtmlInputElement = e.target_unchecked_into();
                                password.set(input.value());
                            })}
                        />
                    </div>

                    <button class="btn btn-primary w-100" onclick={on_submit} disabled={*is_loading}>
                        { if *is_loading { "Registering..." } else { "Register" } }
                    </button>
                </div>
            </div>
        </div>
    }
}
