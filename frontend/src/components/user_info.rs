use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use gloo::console::log;
#[function_component(UserInfo)]
pub fn get_user_info() -> Html{

    let local_storage = web_sys::window().unwrap().local_storage().unwrap().unwrap();
    let cookie = local_storage.get_item("access_token").unwrap();
    let onclick = {
        Callback::from(move |e: MouseEvent| {
            log!("onclick called");
            spawn_local(async move {
                let resp = Request::get(&format!("http://127.0.0.1:8000/auth0/userinfo"))
                .credentials(reqwasm::http::RequestCredentials::Include)
                .send()
                .await
                .unwrap();
            log!(&format!("{}", resp.status()));
            if resp.status() == 200{
                log!("Super");
            }
            else {
                log!("Not super :(");
            }
            });
        })
    };
    html!{
        <button type = "button" onclick = {onclick}>{"Get User Info"}</button>
    }
}