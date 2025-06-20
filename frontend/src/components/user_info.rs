use gloo::console::log;
use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[function_component(UserInfo)]
pub fn get_user_info() -> Html {
    let onclick = {
        Callback::from(move |_e: MouseEvent| {
            log!("onclick called");
            spawn_local(async move {
                let resp = Request::get(&format!("http://127.0.0.1:8000/private"))
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
