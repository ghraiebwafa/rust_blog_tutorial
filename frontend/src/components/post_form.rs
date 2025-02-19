use yew::prelude::*;
use serde::Serialize;
use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;
use yew_router::prelude::*;

use crate::app::Route;

#[derive(Debug,Serialize)]
pub struct NewPost {
    title: String,
    content: String,
}

#[function_component(PostForm)]
pub fn post_form() -> Html {
    let title = use_state(|| String::new());
    let content = use_state(|| String::new());
    let navigator = use_navigator().expect("Navigator should be available");

    let onsubmit = {
        let title = title.clone();
        let content = content.clone();
        let navigator = navigator.clone();

        Callback::from(move |event: SubmitEvent| {
            event.prevent_default();

            let new_post = NewPost {
                title: (*title).clone(),
                content: (*content).clone(),
            };

            spawn_local({
                let navigator = navigator.clone();

                async move {
                    let result = Request::post("http://127.0.0.1:3000/posts")
                        .json(&new_post)
                        .map_err(|e| e.to_string());

                    match result {
                        Ok(request) => {
                            let response = request.send().await;
                            match response {
                                Ok(response) if response.ok() => {
                                    log::info!("Post created successfully");


                                    navigator.push(&Route::Home);
                                }
                                Ok(response) => {
                                    log::error!("Failed to create post: {}", response.status());
                                }
                                Err(e) => {
                                    log::error!("Error sending request: {}", e);
                                }
                            }
                        }
                        Err(e) => {
                            log::error!("Error building request: {}", e);
                        }
                    }
                }
            });

            title.set(String::new());
            content.set(String::new());
        })
    };

    html! {
        <div class="container mt-4">
            <div class="row justify-content-center">
                <div class="col-md-6">
                    <div class="card shadow-sm">
                        <div class="card-body">
                            <h4 class="card-title text-center mb-4">{ "Create a New Post" }</h4>
                            <form onsubmit={onsubmit}>
                                <div class="mb-3">
                                    <label class="form-label">{ "Title" }</label>
                                    <input
                                        type="text"
                                        class="form-control"
                                        placeholder="Enter title"
                                        value={(*title).clone()}
                                        oninput={Callback::from(move |e: InputEvent| {
                                            title.set(e.target_unchecked_into::<web_sys::HtmlInputElement>().value());
                                        })}
                                    />
                                </div>
                                <div class="mb-3">
                                    <label class="form-label">{ "Content" }</label>
                                    <textarea
                                        class="form-control"
                                        rows="5"
                                        placeholder="Enter content"
                                        value={(*content).clone()}
                                        oninput={Callback::from(move |e: InputEvent| {
                                            content.set(e.target_unchecked_into::<web_sys::HtmlTextAreaElement>().value());
                                        })}
                                    />
                                </div>
                                <button type="submit" class="btn btn-primary w-100">{ "Create Post" }</button>
                            </form>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
