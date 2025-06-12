
use components::login::LoginButton;
use time::Duration;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use gloo_net::http::{Request, Response};
use shared::WeatherData;
use web_sys::{console, HtmlInputElement, Text};
use gloo::console::log;
use crate::pages::home::Home;
mod api;
mod components;
mod pages;
mod assets;


#[function_component]
fn App() -> Html {
    html! {
    <>
        <Home/>
    </>    
    }
}
fn main(){
    yew::Renderer::<App>::new().render();
}