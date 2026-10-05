use gloo::console::log;
use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::assets::config::AppConfig;

#[function_component(UserInfo)]
pub fn get_user_info() -> Html {
    let config = use_context::<AppConfig>().expect("Could not load config.json");
    let backend_url = config.backend_url;

    let backend_url_copy = backend_url.clone();
    let onclick = {
        Callback::from(move |_e: MouseEvent| {
            let backend_url = backend_url_copy.clone();
            log!("onclick called");
            spawn_local(async move {
                let resp = Request::get(&format!("{}/private", backend_url))
                    .credentials(reqwasm::http::RequestCredentials::Include)
                    .send()
                    .await
                    .unwrap();
                log!(&format!("{}", resp.status()));
                if resp.status() == 200 {
                    //prompt(message, default)
                } else {
                    log!("Not super :(");
                }
            });
        })
    };
    html! {
        <>
        <button type = "button" onclick = {onclick}>{"Get user info & AT after log"}</button>
        </>
    }
}
