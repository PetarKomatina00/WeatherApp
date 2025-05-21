use gloo::{console::log};
use shared::WeatherData;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use crate::components::front_image::FrontImage;
use crate::components::login::LoginButton;
use crate::components::user_info::UserInfo;
use crate::components::weather_card::WeatherCard;
use crate::{api::api::{fetch_weather_data, ButtonContent}, components::input_button::InputButton};
#[function_component(Home)]
pub fn home() -> Html{
    // let handle_submit: Callback<String> = Callback::from(|text:String| {
    //     web_sys::console::log_1(&format!("Text is: {}", text).into());
    // });
    let weather_data = use_state(|| WeatherData::default());
    

    let on_submit: Callback<ButtonContent> = {
        let weather_data_handle = weather_data.clone();        // keep one handle in the closure’s env
        Callback::from(move |btn: ButtonContent| {
            let state = weather_data_handle.clone();           // clone for this invocation
    
            spawn_local(async move {
                match fetch_weather_data(&btn).await {
                    Ok(resp) => {
                        if let Ok(parsed) = resp.json::<WeatherData>().await {
                            log!(&format!("WeatherData: {:?}", parsed))   ;  
                            state.set(parsed);  
                                      // update state
                        }
                    }
                    Err(err) => log!("{:?}", err),
                }
            });
        })
    };
    
    let data = (*weather_data).clone();
    html! {

        <>
        <div>
            <LoginButton/>
            <UserInfo/>
        <FrontImage data = {on_submit}/>
        if data != WeatherData::default(){
            <WeatherCard weather_data = {data}/>
        }
        </div>
        </>
    
    }
}