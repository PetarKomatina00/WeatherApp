
use gloo::console::log;
use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;
use web_sys::window;
use yew::prelude::*;



#[function_component(LoginButton)]
pub fn login_button() -> Html{

     let on_click = Callback::from(move |e: MouseEvent|{
         log!(&format!("Button clicked"));
         window()
         .unwrap()
         .location()
         .set_href("http://127.0.0.1:8000/auth0/login")
         .expect("Something went wront with redirection");
    });
    html!{
        <button onclick = {on_click} type = "button">{"Login with Auth0"}</button>
    }
}