
use assets::utility::{switch, Route};
use yew::prelude::*;
use yew_router::prelude::*;
mod api;
mod assets;
mod components;
mod pages;
mod models;
#[function_component]
fn App() -> Html {
    html! {
    <>
    <BrowserRouter>
        <Switch<Route> render = {switch}/>
    </BrowserRouter>
    </>
    }
}
fn main() {
    yew::Renderer::<App>::new().render();
}
