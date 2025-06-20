use crate::pages::home::Home;
use assets::utility::{switch, Route};
use gloo::console::log;
use gloo_net::http::{Request, Response};
use shared::WeatherData;
use time::Duration;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::spawn_local;
use web_sys::{console, HtmlInputElement, Text};
use yew::prelude::*;
use yew_router::prelude::*;
mod api;
mod assets;
mod components;
mod pages;

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
