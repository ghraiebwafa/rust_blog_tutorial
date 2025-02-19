use gloo::console::log;
use yew::prelude::*;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew_router::prelude::*;
use crate::app::Route;

#[derive(Serialize, Deserialize)]
struct LoginRequest {
    email: String,
    password: String,
}

#[derive(Serialize, Deserialize)]
struct AuthResponse {
    token: String,
    user_id: String,
}

#[function_component(LoginComponent)]
pub fn login_component() -> Html {
    let email = use_state(|| String::new());
    let password = use_state(|| String::new());
    let is_loading = use_state(|| false);
    let error_message = use_state(|| None::<String>);
    let navigator = use_navigator().unwrap();

    let on_submit = {
        let email = email.clone();
        let password = password.clone();
        let is_loading = is_loading.clone();
        let error_message = error_message.clone();
        let navigator = navigator.clone();

        Callback::from(move |e: MouseEvent| {
            let navigator = navigator.clone();

            e.prevent_default();
            let email = (*email).clone();
            let password = (*password).clone();
            let request = LoginRequest { email, password };

            is_loading.set(true);
            error_message.set(None);

            let is_loading = is_loading.clone();
            let error_message = error_message.clone();
            let navigator = navigator.clone();

            spawn_local(async move {
                let client = Client::new();
                let res = client.post("http://localhost:3000/login")
                    .json(&request)
                    .send()
                    .await;

                match res {
                    Ok(response) if response.status().is_success() => {
                        let auth_response: AuthResponse = response.json().await.unwrap();
                        log!("Login successful! Token:", auth_response.token);
                        navigator.push(&Route::Home);
                    }
                    Ok(_) => {
                        error_message.set(Some("Invalid email or password.".to_string()));
                    }
                    Err(_) => {
                        error_message.set(Some("Network error. Try again.".to_string()));
                    }
                }

                is_loading.set(false);
            });
        })
    };

    let on_input = |state: UseStateHandle<String>| {
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            state.set(input.value());
        })
    };

    html! {
        <div class="container d-flex justify-content-center align-items-center vh-100">
            <div class="card shadow-lg p-4" style="width: 400px;">
                <h2 class="text-center mb-4">{"Login"}</h2>

                if let Some(error) = &*error_message {
                    <div class="alert alert-danger" role="alert">{error}</div>
                }

                <div class="mb-3">
                    <label class="form-label">{"Email"}</label>
                    <input
                        type="email"
                        class="form-control"
                        placeholder="Enter your email"
                        value={(*email).clone()}
                        oninput={on_input(email.clone())}
                    />
                </div>

                <div class="mb-3">
                    <label class="form-label">{"Password"}</label>
                    <input
                        type="password"
                        class="form-control"
                        placeholder="Enter your password"
                        value={(*password).clone()}
                        oninput={on_input(password.clone())}
                    />
                </div>

                <button class="btn btn-primary w-100" onclick={on_submit} disabled={*is_loading}>
                    if *is_loading {
                        <span class="spinner-border spinner-border-sm"></span> {" Logging in..."}
                    } else {
                        {"Login"}
                    }
                </button>
        <div class="mt-3 text-center">
                    <p>{"Don't have an account? "}<Link<Route> to={Route::Register}>{"Register here"}</Link<Route>></p>
                </div>
            </div>
        </div>
    }
}
