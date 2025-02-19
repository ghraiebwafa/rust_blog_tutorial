use yew::prelude::*;
use yew_router::prelude::*;

use crate::app::Route;

#[function_component(Header)]
pub fn header() -> Html {
    html! {
        <nav class="navbar navbar-expand-lg navbar-dark bg-dark shadow-sm">
            <div class="container-fluid">
                <Link<Route> to={Route::Home} classes="navbar-brand fs-4">{ "My Blog" }</Link<Route>>
                <button class="navbar-toggler" type="button" data-bs-toggle="collapse" data-bs-target="#navbarNav" aria-controls="navbarNav" aria-expanded="false" aria-label="Toggle navigation">
                    <span class="navbar-toggler-icon"></span>
                </button>
                <div class="collapse navbar-collapse" id="navbarNav">
                    <ul class="navbar-nav ms-auto">
                        <li class="nav-item">
                            <Link<Route> to={Route::Home} classes="nav-link active">{ "Home" }</Link<Route>>
                        </li>
                        <li class="nav-item">
                            <Link<Route> to={Route::CreatePost} classes="nav-link">{ "Create Post" }</Link<Route>>
                        </li>
                    </ul>
                </div>
            </div>
        </nav>
    }
}
