use crate::pages::home::Home;
use components::login::LoginButton;
use gloo::console::log;
use gloo_net::http::{Request, Response};
use shared::WeatherData;
use time::Duration;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::spawn_local;
use web_sys::{console, HtmlInputElement, Text};
use yew::prelude::*;
mod api;
mod assets;
mod components;
mod pages;

#[function_component]
fn App() -> Html {
    html! {
    <>
        <Home/>
    </>
    }
}
fn main() {
    yew::Renderer::<App>::new().render();
}
