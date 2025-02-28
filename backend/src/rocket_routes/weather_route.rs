use std::env;

use rocket::{futures::TryFutureExt, serde::{self, json::{self, serde_json::json}}};
use serde_json::Value;
use reqwest;
use crate::{repositories::weather_repository::WeatherRepository, DbConnection};
use crate::models::weather::Weather;



#[get("/fetch/<city>")]
pub async fn get_weather_api(city: String) -> Value{
    dotenv::dotenv().ok();
    let api_key = env::var("WEATHER_API_KEY").expect("Missing OpenWeather API KEY");
    let url = format!(
        "http://api.openweathermap.org/data/2.5/weather?q={}&appid={}&units=metric",
        city, api_key
    );

    let response = reqwest::get(url).await.expect("Failed to get response").text().await.expect("Failed to convert to text");
    let weather_data: Weather = serde_json::from_str(&response).expect("Failed to deserialize string");

    
    
    
    println!("Hello from the weather app");
    println!("{:?}", response);
    json!([
        
    ])
}