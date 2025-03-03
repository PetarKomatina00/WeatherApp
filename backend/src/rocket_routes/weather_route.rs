use std::env;

use rocket::{futures::TryFutureExt, response::Responder, serde::{self, json::{self, serde_json::json, Json}}};
use serde_json::Value;
use reqwest::{self, Response};
use crate::{repositories::weather_repository::WeatherRepository, DbConnection};
use crate::models::weather::WeatherData;



#[get("/fetch/<city>")]
pub async fn get_weather_api(city: String) -> Json<WeatherData>{

    dotenv::dotenv().ok();
    let api_key = env::var("WEATHER_API_KEY").expect("Missing OpenWeather API KEY");
    let url = format!(
        "http://api.openweathermap.org/data/2.5/weather?q={}&appid={}&units=metric",
        city, api_key
    );

    let response = reqwest::get(url).await.expect("Failed to get response from GET request").text().await.expect("Failed to convert the response to a text");
    let weather_data: WeatherData = serde_json::from_str(&response).expect("Failed to deserialize string");

    Json(weather_data)
}