use gloo::{console::log};
use shared::WeatherData;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use crate::{api::api::{fetch_weather_data, ButtonContent}, components::input_button::InputButton};
use crate::pages::frontpage::FrontPage;
#[function_component(Home)]
pub fn home() -> Html{
    let handle_submit: Callback<String> = Callback::from(|text:String| {
        web_sys::console::log_1(&format!("Text is: {}", text).into());
    });

    let on_submit: Callback<ButtonContent> = Callback::from(move |data: ButtonContent| {
        // Spawn the async API call
        spawn_local(async move {
            let result = fetch_weather_data(&data).await;
            match result {
                Ok(data) => {
                    log!(&format!("Data submitted successfully {:?}", data));
                    let content_from_api: WeatherData = data.json().await.expect("HOME: Failed to parse JSON");
                    log!(&format!("{:?}", content_from_api));
                },
                Err(e) => log!(format!("Error submitting data: {:?}", e)),
            };
        });
    });
    
    
    html! {

        <>
        <div>
        <FrontPage/>
            <div style="margin-top: 2rem;">
                <InputButton data = {on_submit}/>
            </div>
        </div>
        </>
    
    }
}