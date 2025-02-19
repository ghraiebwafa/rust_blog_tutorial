use yew::prelude::*;
mod app;
mod components;

#[function_component]
fn Main() -> Html {
    html! {
        <app::App />
    }
}

fn main() {
    yew::Renderer::<Main>::new().render();
}
