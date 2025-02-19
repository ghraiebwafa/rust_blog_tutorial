use yew::prelude::*;
use serde::Deserialize;
use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;

#[derive(Properties, PartialEq)]
pub struct PostDetailsProps {
    pub post_id: String,
}

#[derive(Deserialize, Clone, PartialEq)]
 pub struct BlogPost {
    id: String,
    title: String,
    content: String,
    created_at: String,
    updated_at: String,
}

#[function_component(PostDetails)]
pub fn post_details(props: &PostDetailsProps) -> Html {
    let post = use_state(|| None);

    {
        let post = post.clone();
        let post_id = props.post_id.clone();
        use_effect_with(
            (),
            move |_| {
                let post_id = post_id.clone();
                spawn_local(async move {
                    let fetched_post: BlogPost = Request::get(&format!("http://127.0.0.1:3000/posts/{}", post_id))
                        .send()
                        .await
                        .unwrap()
                        .json()
                        .await
                        .unwrap();
                    post.set(Some(fetched_post));
                });
                || ()
            },
        );
    }
    html! {
        <div class="container mt-5">
            <div class="row justify-content-center">
                <div class="col-md-8">
                    <div class="card shadow-sm">
                        <div class="card-body">
                            { if let Some(post) = (*post).clone() {
                                html! {
                                    <>
                                        <h1 class="card-title">{ post.title }</h1>
                                        <p class="card-text">{ post.content }</p>
                                    </>
                                }
                            } else {
                                html! {
                                    <div class="text-center">
                                        <div class="spinner-border text-primary" role="status">
                                            <span class="visually-hidden">{ "Loading..." }</span>
                                        </div>
                                    </div>
                                }
                            }}
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
