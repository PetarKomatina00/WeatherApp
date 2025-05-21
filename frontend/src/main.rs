
use components::login::LoginButton;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use gloo_net::http::{Request, Response};
use shared::WeatherData;
use web_sys::{console, Text};
use gloo::console::log;
use crate::pages::home::Home;

mod api;
mod components;
mod pages;
mod assets;
#[function_component]
fn App() -> Html {
    // let weather_data: UseStateHandle<WeatherData> = use_state(|| WeatherData::default());
    //     {
    //         let weather_data: UseStateHandle<WeatherData> = weather_data.clone();
    //         use_effect_with((), move |_| {
    //             wasm_bindgen_futures::spawn_local(async move {
    //                 let fetched_weather_data: String = Request::get("http://127.0.0.1:8000/fetch/Barcelona")
    //                 .send()
    //                 .await
    //                 .unwrap()
    //                 .text()
    //                 .await
    //                 .unwrap();
    //                 // .unwrap();
    //             //weather_data.set(fetched_weather_data);
    //             log!(format!("Response: {:?}", fetched_weather_data));
    //             //console::log_1(&JsValue::from_str(&format!("{:?}", fetched_weather_data)));
    //             });
    //             || ()
    //         });
    //     }

    let weather_response = use_state(|| None::<WeatherData>);
    let error = use_state(|| None::<String>);
    {
        let weather_response_use_effect = weather_response.clone();
        
        
    }

    use_effect_with(weather_response, move |_|{

    });


        
    html! {
    <>
        
        <Home/>
        
    </>    
    }
}


fn main(){
    yew::Renderer::<App>::new().render();
}