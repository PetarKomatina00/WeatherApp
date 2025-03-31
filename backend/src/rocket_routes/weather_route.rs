

use rocket::serde::json::Json;

use crate::repositories::weather_repository::WeatherRepository;
// use crate::models::weather::WeatherData;
use shared::WeatherData;
#[get("/fetch/<city>")]
pub async fn get_weather_api(city: String) -> Json<WeatherData>{
    //todo!("Do something with data...");
    let weather_data = WeatherRepository::get_city_weather_by_name(&city).await.unwrap();
    Json(weather_data)
}