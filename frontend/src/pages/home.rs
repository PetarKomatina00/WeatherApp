use std::thread::spawn;

use crate::components::front_image::FrontImage;
use crate::components::login::{Auth0Action, LoginButton};
use crate::components::login_modal::LoginModal;
use crate::components::user_info::UserInfo;
use crate::components::weather_card::WeatherCard;
use crate::{
    api::api::{fetch_weather_data, ButtonContent},
    components::input_button::InputButton,
};
use gloo::console::log;
use gloo::net::http::Response;
use reqwasm::http::Request;
use serde::Deserialize;
use shared::WeatherData;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;


#[derive(Deserialize)]
pub struct Profile {
    pub name: String,
    pub sub: String,
    pub email: String,
    pub family_name: String,
    pub given_name: String,
    pub email_verified: bool,
}


#[function_component(Home)]
pub fn home() -> Html {
    // let handle_submit: Callback<String> = Callback::from(|text:String| {
    //     web_sys::console::log_1(&format!("Text is: {}", text).into());
    // });
    let weather_data = use_state(|| WeatherData::default());
    let is_login_modal_open = use_state(|| false);
    let user_profile = use_state(|| None::<Profile>);
    let is_logged_in = use_state(|| false);

    let is_logged_in_handle = is_logged_in.clone();
    use_effect_with((), move |_| {
        let user_profile_handle = user_profile.clone();
        spawn_local(async move {
            let url = format!("http://127.0.0.1:8000/whoami");
            let response = Request::get(&url)
            .header("Accept", "application/json")
            .credentials(web_sys::RequestCredentials::Include)
            .send()
            .await;

            match response{
                Ok(r) if r.ok() => {
                    let user_profile: Profile = r.json().await.expect("Error converting JSON to Profile");
                    user_profile_handle.set(Some(user_profile));
                    is_logged_in_handle.set(true);
                }
                _ => {
                    log!("Error");
                }
            }
        });
        || ()
    });

    
    let on_submit: Callback<ButtonContent> = {
        let weather_data_handle = weather_data.clone(); // keep one handle in the closure’s env
        let is_login_modal_open_handle = is_login_modal_open.clone();
        Callback::from(move |btn: ButtonContent| {
            let state = weather_data_handle.clone(); // clone for this invocation
            let is_login_modal_open = is_login_modal_open_handle.clone();
            spawn_local(async move {
                match fetch_weather_data(&btn).await {
                    Ok(resp) => {
                        log!(&format!("Response: {:?}", resp));
                        if let Ok(parsed) = resp.json::<WeatherData>().await {
                            is_login_modal_open.set(false);
                            log!(&format!("WeatherData: {:?}", parsed));
                            state.set(parsed);
                        }
                    }
                    Err(err) => {
                        log!(&format!("You are not logged in"));
                        is_login_modal_open.set(true);
                    }
                }
            });
        })
    };

    let data = (*weather_data).clone();
    html! {
        <>
        if *is_logged_in{
            
            <LoginButton action = {Auth0Action::Logout} onclick = {Callback::from(move |_| {})}/>
            // <button class="futuristic-button">{"Log out"}</button>
        }
        else {
            <LoginButton action = {Auth0Action::Login} onclick = {Callback::from(move |_| {})}/>
            // <button class="futuristic-button">{"Log in"}</button>
        }
        if *is_login_modal_open{
            {"Not Logged in"}
            <LoginModal login_model_open = {
                let is_login_modal_open = is_login_modal_open.clone();
                Callback::from(move |_| is_login_modal_open.set(false))
            }/>
        }
        <div>

            // <UserInfo/>
        <FrontImage data = {on_submit}/>
        if data != WeatherData::default(){
            <WeatherCard weather_data = {data}/>
        }
        // <LoginButton/>
        </div>
        </>
    }
}
