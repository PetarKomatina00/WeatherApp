use gloo::console::log;
use gloo_net::http::{Request, Response};
use shared::WeatherData;
use web_sys::console::log;
use yew::prelude::*;
use serde::Deserialize;


#[derive(Clone, PartialEq, Default)]
pub struct ButtonContent {
    pub content: String,
}

pub async fn fetch_weather_data(data: &ButtonContent) -> Result<Response, String>{

    let response = Request::get(&format!("http://127.0.0.1:8000/fetch/{}", data.content))
        .send()
        .await
        .map_err(|error| error.to_string());
    let response = response.unwrap();
    if !response.ok(){
        return Err(format!("Failed to fetch weather, status: {}", response.status()));
    }

    Ok(response)
}