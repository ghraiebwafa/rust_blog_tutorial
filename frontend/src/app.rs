use yew::prelude::*;
use yew_router::prelude::*;
use crate::components::{PostList, PostDetails, PostForm, PostUpdate, Header, LoginComponent, RegisterComponent};


#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Login,
    #[at("/register")]
    Register,
    #[at("/posts")]
    Home,
    #[at("/post/:id")]
    PostDetails { id: String },
    #[at("/create")]
    CreatePost,
    #[at("/update/:id")]
    PostUpdate { id: String },
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Login => html! { <LoginComponent /> },
        Route::Register => html! { <RegisterComponent /> },
        Route::Home => html! { <PostList /> },
        Route::PostDetails { id } => html! { <PostDetails post_id={id} /> },
        Route::CreatePost => html! { <PostForm /> },
        Route::PostUpdate { id } => html! { <PostUpdate post_id={id} /> },
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Header />
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}
