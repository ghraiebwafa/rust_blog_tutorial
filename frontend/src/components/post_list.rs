use yew::prelude::*;
use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::spawn_local;
use crate::app::Route;
use yew_router::prelude::*;
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BlogPost {
    pub id: String,
    pub title: String,
    pub content: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize)]
pub struct NewPost {
    pub title: String,
    pub content: String,
}

#[function_component(PostList)]
pub fn post_list() -> Html {
    let posts = use_state(|| Vec::<BlogPost>::new());

    {
        let posts = posts.clone();
        use_effect(move || {
            spawn_local(async move {
                if let Ok(response) = Request::get("http://127.0.0.1:3000/posts").send().await {
                    if response.ok() {
                        if let Ok(fetched_posts) = response.json::<Vec<BlogPost>>().await {
                            posts.set(fetched_posts);
                        }
                    }
                }
            });

            || ()
        });
    }

    let delete_post = {
        let posts = posts.clone();
        Callback::from(move |post_id: String| {
            let posts = posts.clone();
            spawn_local(async move {
                let url = format!("http://127.0.0.1:3000/posts/{}", post_id);
                match Request::delete(&url).send().await {
                    Ok(response) if response.ok() => {
                        log::info!("Post deleted successfully");
                        posts.set(posts.iter().cloned().filter(|p| p.id != post_id).collect());
                    }
                    _ => log::error!("Failed to delete post"),
                }
            });
        })
    };

    html! {
    <div class="container my-4">
        <div class="row">
           { for (*posts).iter().map(|post| {
    let delete_post = delete_post.clone();
    let post_id = post.id.clone();
    let post_id_clone = post_id.clone();

    html! {
        <div key={post_id.clone()} class="col-md-4 mb-4">
            <div class="card shadow-sm">
                <div class="card-body">
                    <h5 class="card-title">{ &post.title }</h5>
                    <p class="card-text">{ &post.content }</p>

                    <div class="d-flex justify-content-between">
                        <Link<Route> to={Route::PostUpdate { id: post_id.clone() }}>
                            <button class="btn btn-warning w-48 mb-2"> { "Update" } </button>
                            </Link<Route>>


                        <button class="btn btn-danger w-48 mb-2"
                            onclick={Callback::from(move |_| delete_post.emit(post_id.clone()))}>
                            { "Delete" }
                        </button>
                    </div>

                    <div class="mt-2 text-end">
                        <Link<Route> to={Route::PostDetails { id: post_id_clone }} classes="text-muted small">
                            { "Click here to read more..." }
                        </Link<Route>>
                    </div>
                </div>
            </div>
        </div>
    }
}) }

        </div>
    </div>
}
}
