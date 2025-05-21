

use rocket::serde::json::Json;

//use crate::models::weather::WeatherData;
use shared::WeatherData;
use utoipa::{openapi, OpenApi};

use crate::{models::UserInfo, repositories::weather_repository::WeatherRepository};
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

pub async fn fetch_auth0_userinfo(access_token: &str) -> Result<UserInfo, String>{
    let client = reqwest::Client::builder()
        .build().expect("Could not create client to send HTTP");

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("Accept", "application/json".parse().expect("Could not parse http"));

    println!("Headers: {:?}", headers);

    let request = client
    .request(reqwest::Method::GET, "https://dev-kr7vi67c2vo4vs3w.eu.auth0.com/userinfo")
    .bearer_auth(access_token);
    let response = request.send().await.expect("Could not send HTTP");

    println!("Response : {:?}", response);
    let body = response.json::<UserInfo>().await.expect("Could get BODY from response");

    println!("Body is: {:?}", body);
    Ok(body)
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