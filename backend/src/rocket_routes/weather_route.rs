

use crate::repositories::weather_repository::WeatherRepository;
#[get("/fetch/<city>")]
pub async fn get_weather_api(city: String) -> Result<String, String>{
    //todo!("Do something with data...");
    WeatherRepository::get_city_weather_by_name(&city).await
}