use yew::prelude::*;
use serde::{Deserialize, Serialize};
use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;
use yew_router::prelude::*;
use crate::app::Route;

#[derive(Deserialize, Clone, PartialEq, Debug)]
struct BlogPost {
    id: String,
    title: String,
    content: String,
    created_at: String,
    updated_at: String,
}

#[derive(Serialize, Clone, Debug)]
struct UpdatePost {
    title: String,
    content: String,
}

#[derive(Clone,Properties, PartialEq)]
pub struct PostUpdateProps {
    pub post_id: String,
}

#[function_component(PostUpdate)]
pub fn post_update(props: &PostUpdateProps) -> Html {
    let title = use_state(|| String::new());
    let content = use_state(|| String::new());
    let navigator = use_navigator().expect("Navigator should be available");

    let post_id = props.post_id.clone();

    {
        let title = title.clone();
        let content = content.clone();
        let post_id = post_id.clone();

        use_effect_with(post_id.clone(), move |_| {
            spawn_local(async move {
                match Request::get(&format!("http://127.0.0.1:3000/posts/{}", post_id))
                    .send()
                    .await
                {
                    Ok(response) => {
                        match response.json::<BlogPost>().await {
                            Ok(post) => {
                                title.set(post.title);
                                content.set(post.content);
                            }
                            Err(err) => log::error!("Failed to parse post JSON: {:?}", err),
                        }
                    }
                    Err(err) => log::error!("Failed to fetch post: {:?}", err),
                }
            });

            || ()
        });
    }

    let onsubmit = {
        let title = title.clone();
        let content = content.clone();
        let navigator = navigator.clone();

        Callback::from(move |event: SubmitEvent| {
            event.prevent_default();

            let updated_post = UpdatePost {
                title: (*title).clone(),
                content: (*content).clone(),
            };

            let post_id_clone = post_id.clone();
            let navigator_clone = navigator.clone();

            spawn_local(async move {
                let url = format!("http://127.0.0.1:3000/posts/{}", post_id_clone);
                let result = match Request::put(&url).json(&updated_post) {
                    Ok(request) => request.send().await,
                    Err(e) => {
                        log::error!("Error building request: {}", e);
                        return;
                    }
                };

                match result {
                    Ok(response) => {
                        if response.status() == 200 {
                            log::info!("Post updated successfully");
                            navigator_clone.push(&Route::Home); // Use the cloned navigator
                        } else {
                            log::error!("Failed to update post. Status: {}", response.status());
                        }
                    }
                    Err(e) => {
                        log::error!("Error sending request: {}", e);
                    }
                }
            });
        })
    };
    html! {
        <div class="container mt-4">
            <div class="row justify-content-center">
                <div class="col-md-6">
                    <div class="card shadow-sm">
                        <div class="card-body">
                            <h4 class="card-title text-center mb-4">{ "Update Post" }</h4>
                            <form onsubmit={onsubmit}>
                                <div class="mb-3">
                                    <label class="form-label">{ "Title" }</label>
                                    <input
                                        type="text"
                                        class="form-control"
                                        placeholder="Enter title"
                                        value={(*title).clone()}
                                        oninput={Callback::from({
                                            let title = title.clone();
                                            move |e: InputEvent| {
                                                title.set(e.target_unchecked_into::<web_sys::HtmlInputElement>().value());
                                            }
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
                                        oninput={Callback::from({
                                            let content = content.clone();
                                            move |e: InputEvent| {
                                                content.set(e.target_unchecked_into::<web_sys::HtmlTextAreaElement>().value());
                                            }
                                        })}
                                    />
                                </div>
                                <button type="submit" class="btn btn-primary w-100">{ "Update Post" }</button>
                            </form>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}