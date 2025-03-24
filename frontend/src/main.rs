use wasm_bindgen::JsValue;
use yew::prelude::*;
use gloo_net::http::Request;
use shared::WeatherData;
use web_sys::{console, Text};
use gloo::console::log;
#[function_component]
fn App() -> Html {
    // let counter = use_state(|| 0);
    // let onclick = {
    //     let counter = counter.clone();
    //     move |_| {
    //         let value = *counter + 1;
    //         counter.set(value);
    //     }
    // };

    // html! {
    //     <div>
    //         <button {onclick}>{ "+1" }</button>
    //         <p>{ *counter }</p>
    //     </div>
    // }


    let weather_data: UseStateHandle<WeatherData> = use_state(|| WeatherData::default());
        {
            let weather_data: UseStateHandle<WeatherData> = weather_data.clone();
            use_effect_with((), move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    let fetched_weather_data: String = Request::get("http://127.0.0.1:8000/fetch/Barcelona")
                    .send()
                    .await
                    .unwrap()
                    .text()
                    .await
                    .unwrap();
                    // .unwrap();
                //weather_data.set(fetched_weather_data);
                log!(format!("Response: {:?}", fetched_weather_data));
                //console::log_1(&JsValue::from_str(&format!("{:?}", fetched_weather_data)));
                });
                || ()
            });
        }
        
    html! {
        {"Hello"}
    }
}


fn main(){
    yew::Renderer::<App>::new().render();
}