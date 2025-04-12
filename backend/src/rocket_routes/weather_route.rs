

use rocket::serde::json::Json;

//use crate::models::weather::WeatherData;
use shared::WeatherData;
use utoipa::{openapi, OpenApi};

use crate::repositories::weather_repository::WeatherRepository;
// use crate::models::weather::WeatherData;


#[utoipa::path(
    get,
    path = "/fetch/{city}",
    responses(
        (status = 200, description = "WeatherData found successfully", body = WeatherData),
        (status = NOT_FOUND, description = "WeatherData was not found")
    ),
    params(
        ("city" = String, Path, description = "Enter city to call API"),
    ),
    tag = "Get Operation"
)]
#[get("/fetch/<city>")]
pub async fn get_weather_api(city: String) -> Json<WeatherData>{
    //todo!("Do something with data...");
    let weather_data = WeatherRepository::get_city_weather_by_name(&city).await.unwrap();
    Json(weather_data)
}
#[derive(OpenApi)]
#[openapi(
    paths(
        get_weather_api
    ),
    components(
        schemas(WeatherData)
    ),
    tags(
        (name = "Petar Komatina", description = "Endpoints for fetchin data")
    )
)]
pub struct ApiDoc;