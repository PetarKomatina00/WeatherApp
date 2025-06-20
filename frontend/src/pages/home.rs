use std::thread::spawn;

use crate::assets::utility::Route;
use crate::components::front_image::FrontImage;
use crate::components::login_button::{Auth0Action, LoginButton};
use crate::components::login_modal::LoginModal;
use crate::components::spinner::Spinner;
use crate::components::user_info::UserInfo;
use crate::components::weather_card::WeatherCard;
use crate::pages::admin_page::AdminPage;
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
use yew::html::ImplicitClone;
use yew::prelude::*;
use yew_router::hooks::use_navigator;
use yew_router::Routable;

#[derive(Deserialize, Debug)]
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

    let navigator = use_navigator().unwrap();
    let weather_data = use_state(|| WeatherData::default());
    let is_login_modal_open = use_state(|| false);
    let user_profile: UseStateHandle<Option<Profile>> = use_state(|| None::<Profile>);
    let is_logged_in = use_state(|| false);
    let is_logged_in_handle = is_logged_in.clone();
    let user_profile_handle = user_profile.clone();
    let is_admin_logged = use_state(|| false);
    let is_loading = use_state(|| false);

    use_effect_with((), move |_| {
        let user_profile_handle = user_profile_handle.clone();
        spawn_local(async move {
            let url = format!("http://127.0.0.1:8000/whoami");
            let response = Request::get(&url)
                .header("Accept", "application/json")
                .credentials(web_sys::RequestCredentials::Include)
                .send()
                .await;

            match response {
                Ok(r) if r.ok() => {
                    let profile: Profile =
                        r.json().await.expect("Error converting JSON to Profile");
                    user_profile_handle.set(Some(profile));
                    is_logged_in_handle.set(true);
                }
                _ => {
                    log!("Error");
                }
            }
        });
        || ()
    });

    let is_admin_logged_handle = is_admin_logged.clone();
    use_effect_with((*is_logged_in).clone(), move |_| {
        let is_admin_logged_handle = is_admin_logged_handle.clone();
        spawn_local(async move {
            let is_admin_logged_handle = is_admin_logged_handle.clone();
            let url = format!("http://127.0.0.1:8000/get/user/claim");
            let response = Request::get(&url)
                .header("Accept", "application/json")
                .credentials(web_sys::RequestCredentials::Include)
                .send()
                .await;

            match response {
                Ok(r) if r.ok() => is_admin_logged_handle.set(true),
                _ => {
                    log!("Error2");
                }
            }
        });
        || ()
    });


    let is_loading_handle = is_loading.clone();
    let on_submit: Callback<ButtonContent> = {
        let weather_data_handle = weather_data.clone(); // keep one handle in the closure’s env
        let is_login_modal_open_handle = is_login_modal_open.clone();
        let is_loading_handle = is_loading_handle.clone();
        Callback::from(move |btn: ButtonContent| {
            let state = weather_data_handle.clone(); // clone for this invocation
            let is_login_modal_open = is_login_modal_open_handle.clone();
            let is_loading_handle = is_loading_handle.clone();
            spawn_local(async move {
                is_loading_handle.set(true);
                match fetch_weather_data(&btn).await {
                    Ok(resp) => {
                        //log!(&format!("Response: {:?}", resp));
                        is_loading_handle.set(false);
                        if let Ok(parsed) = resp.json::<WeatherData>().await {
                            is_login_modal_open.set(false);
                            //log!(&format!("WeatherData: {:?}", parsed));
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

    let move_to_dashboard = Callback::from(move |e: MouseEvent| {
        navigator.push(&Route::AdminPage);
    });
    let data = (*weather_data).clone();
    html! {
    <>
    
        <nav class="navbar navbar-expand-md navbar-light bg-light px-3">
            <button class="navbar-toggler" type="button" data-bs-toggle="collapse" data-bs-target="#navbarContent"
                aria-controls="navbarContent" aria-expanded="false" aria-label="Toggle navigation">
                <span class="navbar-toggler-icon"></span>
            </button>
            <div class="collapse navbar-collapse d-md-none" id="navbarContent">
                <ul class="navbar-nav me-auto mb-2 mb-lg-0 d-flex gap-2">
                    if *is_admin_logged{
                        <li class="nav-item">
                            <button class="apple-button" id="admin-button" onclick = {move_to_dashboard}>{"View API Logs"}</button>
                        </li>
                    }
                    if *is_logged_in{
                        <li class="nav-item">
                            <LoginButton action = {Auth0Action::Logout} onclick = {Callback::from(move |_| {})}/>
                        </li>
                    }
                    else{
                        <li>
                            <LoginButton action = {Auth0Action::Login} onclick = {Callback::from(move |_| {})}/>
                        </li>
                    }

                </ul>
                <span class="navbar-text text-center w-md-auto">
                    <div class = ""><h1 class = "welcome-message">{format!("Hello {}", user_profile.as_ref().map(|p| &p.name).unwrap_or(&format!("Guest"))) }</h1></div>
                </span>
            </div>
        </nav>
        if *is_login_modal_open{
            {"Not Logged in"}
            <LoginModal login_model_open = {
                let is_login_modal_open = is_login_modal_open.clone();
                Callback::from(move |_| is_login_modal_open.set(false))
            }/>
        }
        <div class="background-image-section">
            <FrontImage data = {on_submit}/>
            if data != WeatherData::default(){
                <WeatherCard weather_data = {data}/>
            }
            // <LoginButton/>
        </div>
        if *is_loading{
        <Spinner/>
    }
    </>
    }
}
