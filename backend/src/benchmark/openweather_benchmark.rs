use rocket::{http::Status, serde::json::Json};
use shared::WeatherData;

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