use gloo::{console::log};
use shared::WeatherData;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use crate::components::front_image::FrontImage;
use crate::components::login::LoginButton;
use crate::components::login_modal::LoginModal;
use crate::components::user_info::UserInfo;
use crate::components::weather_card::WeatherCard;
use crate::{api::api::{fetch_weather_data, ButtonContent}, components::input_button::InputButton};
#[function_component(Home)]
pub fn home() -> Html{
    // let handle_submit: Callback<String> = Callback::from(|text:String| {
    //     web_sys::console::log_1(&format!("Text is: {}", text).into());
    // });
    let weather_data = use_state(|| WeatherData::default());
    let is_login_modal_open = use_state(|| false);

    let on_submit: Callback<ButtonContent> = {
        let weather_data_handle = weather_data.clone();        // keep one handle in the closure’s env
        let is_login_modal_open_handle = is_login_modal_open.clone();       
        Callback::from(move |btn: ButtonContent| {
            let state = weather_data_handle.clone();           // clone for this invocation
            let is_login_modal_open = is_login_modal_open_handle.clone();
            spawn_local(async move {
                match fetch_weather_data(&btn).await {
                    Ok(resp) => {
                        log!(&format!("Response: {:?}", resp));
                        if let Ok(parsed) = resp.json::<WeatherData>().await {
                            is_login_modal_open.set(false);
                            log!(&format!("WeatherData: {:?}", parsed))   ;  
                            state.set(parsed);  
                        }
                    }
                    Err(err) => {
                        log!(&format!("You are not logged in"))   ;  
                        is_login_modal_open.set(true);
                    }
                }
            });
        })
    };
    
    let data = (*weather_data).clone();
    html! {
        <>
        if *is_login_modal_open{
            {"Logged in"}
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