use std::{sync::atomic::{AtomicU32, Ordering}, thread::sleep};
use std::time::Duration;
use rocket::{http::Status, serde::json::Json};
use shared::{Coords, Main, Sys, WeatherData};

use crate::repositories::weather_repository::WeatherRepository;

//This is used only for local benchmarking purposes using oha
//This is a Fake API

#[get("/weather/benchmark?<city>")]
pub async fn benchmark_weather(city: String) -> Result<Json<WeatherData>, Status>{
    match WeatherRepository::fetch_data_weather_api(&city).await{
        Ok(result) => {
            Ok(Json(result))
        }
        Err(err) => {
            eprint!("Openweather error: {err}");
            Err(Status::BadGateway)
        }
    }
}

static MOCK_API_CALLS: AtomicU32 = AtomicU32::new(0);

pub async fn fetch_data_weather_api_mock(city: &str) -> Result<WeatherData, String>{
    let call_number = MOCK_API_CALLS.fetch_add(1, Ordering::Relaxed) + 1;

    println!("API call {call_number} for {city}");

    sleep(Duration::from_millis(2100));

    Ok(WeatherData {
        id: 1,
        name : "Test city".to_string(),
        main: Main::default(),
        sys: Sys::default(),
        timezone: 1,
        coord: Coords::default(),
        cod: 1
    })
}